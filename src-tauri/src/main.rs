#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

extern crate rand;
extern crate bip39;
extern crate secstr;
extern crate base64;

use std::fs;
use std::path;
use std::str;
use std::str::{from_utf8};
use std::sync::{Mutex, Arc};
use std::rc::{Rc};
use secstr::*;
use bip39::{Mnemonic, Language};
use openssl::rand::rand_bytes;
use openssl::symm::encrypt;
use openssl::symm::decrypt;
use openssl::symm::Cipher;
use openssl::sha::sha256;
use zeroize::Zeroize;
use hex::ToHex;
use rusqlite::{params, Connection, Result};

mod lib;

#[derive(Debug)]
struct Context{
  db: Option<Connection>,
  logged_in: bool,
  account_key: secstr::SecStr
}




fn main() {

  let mut context = Context{
    db: Default::default(),
    logged_in: false,
    account_key: SecStr::from("")
  };

  if is_vault_setup() { // SETUP CONTEXT

    let db = match Connection::open("./db/database.db"){
      Ok(conn) => conn,
      Err(error) => panic!("Failed to create db connection for context {}", error)
    };

    context.db = Some(db);

  }


  tauri::Builder::default()
  .manage(Mutex::new(context))
  .invoke_handler(tauri::generate_handler![generate_mnemonic, setup_vault, login, is_vault_setup, logout, generate_password, add_new_password, get_password])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}


#[tauri::command]
fn get_password(state: tauri::State<Mutex<Context>>, id: u32) -> String {

  let context = state.lock().expect("Failed to acquire lock on context");
  
  if !context.logged_in {
    panic!("Not logged in")
  }

  let conn = context.db.as_ref().unwrap();

  conn.pragma_update(None, "key", base64::encode(context.account_key.unsecure())).unwrap();

  let password : String = conn.query_row("SELECT * FROM Password WHERE password_id = ?", params![id], |row| {row.get(3)}).unwrap();

  let vault_key : String = conn.query_row("SELECT * FROM UserData LIMIT 1", [], |row| {row.get(1)}).unwrap();

  let password_plain_text = match decrypt(Cipher::aes_256_cbc(), &base64::decode(vault_key).unwrap(), None, &base64::decode(password).unwrap()) {
    Ok(pt) => pt,
    Err(error) => panic!("Failed to create cipher text {:?}", error),
  };

  return String::from(from_utf8(&password_plain_text).unwrap());
}

#[tauri::command]
fn add_new_password(state: tauri::State<Mutex<Context>>, source: String, username: String, password: String, image: String){
  let context = state.lock().unwrap();

  if !context.logged_in {
    return;
  }

  let conn = context.db.as_ref().unwrap();

  conn.pragma_update(None, "key", base64::encode(context.account_key.unsecure())).unwrap();

  // encrypt the password

  let mut user_data_stmt = conn.prepare("SELECT * FROM UserData").unwrap();


  let user_data : String = user_data_stmt.query_row([], |row| {
    row.get(1)
  }).unwrap();


  let mut key = base64::decode(user_data).unwrap();

  let password_cipher_text = match encrypt(Cipher::aes_256_cbc(), &key, None, &password.as_bytes()) {
    Ok(cipher) => cipher,
    Err(error) => panic!("Failed to create cipher text {:?}", error),
  };

  key.zeroize();

  match conn.execute("INSERT INTO Password (source, username, password, added, icon) VALUES (?1, ?2, ?3, ?4, ?5)", params![source, username, base64::encode(password_cipher_text), lib::time::get_current_secs(), image]){
    Ok(_) => println!("Inserted New Password Item!"),
    Err(error) => panic!("Err: {}", error)
  }

  conn.pragma_update(None, "key", "").unwrap();

}

#[tauri::command]
fn generate_password(state: tauri::State<Mutex<Context>>) -> String{

  let context = state.lock().unwrap();

  if context.logged_in {
    return lib::crypto::generate_password();
  }

  return String::from("");
}

#[tauri::command]
fn logout(state: tauri::State<Mutex<Context>>)  {

  let mut context = state.lock().unwrap();

  context.logged_in = false;

  context.account_key.zero_out();

}

#[tauri::command]
fn is_vault_setup() -> bool {
  return path::Path::new("./db/database.db").exists() && path::Path::new("./keys").exists();
}


#[tauri::command]
fn login(state: tauri::State<Mutex<Context>>, master_key: String) -> Result<Vault, String> {

  let mk_hash = sha256(master_key.as_bytes());

  let keys_string = fs::read_to_string("./keys").unwrap();

  let mut keys = keys_string.lines();

  let sk_cipher_hex = keys.next().unwrap();

  let sk_cipher_raw = base64::decode(sk_cipher_hex).unwrap();

  let sk = match decrypt(Cipher::aes_256_cbc(), &mk_hash, None, &sk_cipher_raw){
    Ok(plain) => plain,
    Err(error) => panic!("Error while decrypting secret key: {}", error)
  };

  let mut raw_auk : Vec<u8> = sk.iter().zip(mk_hash).map(|(x, y)| x ^ y).collect();

  let mut context = state.lock().unwrap();

  context.logged_in = true;

  let conn = context.db.as_ref().unwrap();

  conn.pragma_update(None, "key", base64::encode(&raw_auk)).unwrap();

  raw_auk.zeroize();

  let mut passwords_stmt = conn.prepare("SELECT * FROM Password").unwrap();

  let passwords_stmt_iter = passwords_stmt.query_map([], |row| {
    Ok(
      Password{
        password_id: row.get(0)?,
        source: row.get(1)?,
        username: row.get(2)?,
        added: row.get(4)?,
        icon: row.get(5)?
      }
    )
  }).unwrap();


  let mut user_data_stmt = conn.prepare("SELECT * FROM UserData").unwrap();

  let mut user_data_iter = user_data_stmt.query_map([], |row| {
    Ok(
      UserData{
        userdata_id: row.get(0)?,
        last_unlock: row.get(2)?,
      }
    )


  }).unwrap();

  let user_data = user_data_iter.next().unwrap();

  let mut passwords = Vec::new();

  for password in passwords_stmt_iter {
    passwords.push(password.unwrap());
  }

  conn.pragma_update(None, "key", "").unwrap();


  Ok(Vault{
    user_data: user_data.unwrap(),
    passwords: passwords
  })
}

#[tauri::command]
fn generate_mnemonic() -> Mnemonic {

  let mut rng = rand::thread_rng();

  let phrase = Mnemonic::generate_in_with(&mut rng, Language::English, 12).unwrap();

  return phrase;
}

#[tauri::command]
fn setup_vault(state: tauri::State<Mutex<Context>>, master_key: String, pass_phrase: String) -> bool {

  let mut buf = [0; 32];

  rand_bytes(&mut buf).unwrap(); // generate a secret key

  let master_key_hash = sha256(master_key.as_bytes());

  let pass_phrase_hash = sha256(pass_phrase.as_bytes());

  let mut master_key_private_key_cipher_text = match encrypt(Cipher::aes_256_cbc(), &master_key_hash, None, &buf) {
    Ok(cipher) => cipher,
    Err(error) => panic!("Failed to create cipher text {:?}", error),
  };

  let mut pass_phrase_private_key_cipher_text = match encrypt(Cipher::aes_256_cbc(), &pass_phrase_hash, None, &buf) {
    Ok(cipher) => cipher,
    Err(error) => panic!("Failed to create cipher text {:?}", error),
  };

  let mut mk_hex = base64::encode(&master_key_private_key_cipher_text);
  master_key_private_key_cipher_text.zeroize();

  let mut pp_hex = base64::encode(&pass_phrase_private_key_cipher_text);
  pass_phrase_private_key_cipher_text.zeroize();


  fs::write("./keys", format!("{}\n{}", mk_hex, pp_hex)).expect("Unable to write key file");

  mk_hex.zeroize();
  pp_hex.zeroize();
  
  let mut raw_auk : Vec<u8> = lib::crypto::generate_account_key(buf, master_key_hash).unwrap(); // xor secret key with master key hash

  buf.zeroize();
  
  let conn = Connection::open("./db/database.db").unwrap();

  conn.pragma_update(None, "key", base64::encode(&raw_auk)).unwrap();

  raw_auk.zeroize();

  let init_sql = fs::read_to_string("./db/init.sql").unwrap();

  match conn.execute_batch(&init_sql){
    Ok(_) => println!("Created Tables!"),
    Err(error) => panic!("Err: {}", error)
  }

  rand_bytes(&mut buf).unwrap(); // generate a vault key



  match conn.execute("INSERT INTO UserData (vault_key, last_unlock) VALUES (?1, ?2)", params![base64::encode(buf), lib::time::get_current_secs()]){
    Ok(_) => println!("Inserted Item!"),
    Err(error) => panic!("Err: {}", error)
  }

  buf.zeroize();

  conn.pragma_update(None, "key", "").unwrap();

  let mut context = state.lock().unwrap();

  context.account_key = SecStr::from(base64::encode(&raw_auk));

  context.db = Some(conn);

  return true;
}



#[derive(Debug, serde::Serialize)]
struct Password{
  password_id: u32,
  username: String,
  source: String,
  added: u32,
  icon: String
}


#[derive(Debug, serde::Serialize)]
struct UserData{
  userdata_id : u32,
  last_unlock: u32,
}


#[derive(Debug, serde::Serialize)]
struct Vault{
  user_data: UserData,
  passwords: Vec<Password>
}



#[derive(Debug)]
struct AppContext{
  
}
