#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

extern crate rand;
extern crate bip39;
extern crate secstr;
extern crate base64;
extern crate argon2;

use std::fs;
use std::path;
use std::str::{from_utf8};
use std::sync::{Mutex};
use lib::db::get_passwords;
use lib::db::get_userdata;
use lib::db::get_vault_key;
use lib::util::acquire_context_lock;
use secstr::*;
use bip39::{Mnemonic, Language};
use openssl::rand::rand_bytes;
use openssl::symm::encrypt;
use openssl::symm::decrypt;
use openssl::symm::Cipher;
use types::Context;
use types::Password;
use types::UserData;
use types::Vault;
use zeroize::Zeroize;
use rusqlite::{params, Connection, Result};
use argon2::{
  password_hash::{
      rand_core::OsRng,
      PasswordHasher, SaltString
  },
  Argon2
};

mod lib;
mod types;




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
  .invoke_handler(tauri::generate_handler![generate_mnemonic, setup_vault, login, is_vault_setup, logout, generate_password, add_new_password, get_password, remove_password, get_vault])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}

#[tauri::command]
fn get_vault(state: tauri::State<Mutex<Context>>) -> Result<Vec<Password>, String>{

  let context = match acquire_context_lock(&state) {
    Ok(ctx) => ctx,
    Err(_) => return Err("Failed to acquire context".into()),
  };

  let conn = match context.db.as_ref() {
    Some(conn) => conn,
    None => return Err("Failed to get database reference".into()),
  };


  let passwords = match sqlcipher_block!(conn, base64::encode(context.account_key.unsecure()), || -> Result<Vec<Password>, String> {get_passwords(conn)}){
    Ok(passwords) => passwords,
    Err(err) => return Err(err)
  };

  return Ok(passwords);

}

#[tauri::command]
fn remove_password(state: tauri::State<Mutex<Context>>, id: u32){

  let context = state.lock().expect("Failed to acquire lock on context");

  if !context.logged_in {
    panic!("Not logged in")
  }

  let conn = context.db.as_ref().unwrap();

  conn.pragma_update(None, "key", base64::encode(context.account_key.unsecure())).unwrap();
  

  conn.execute("DELETE FROM Password WHERE password_id = ?", params![id]).unwrap();


  conn.pragma_update(None, "key", "").unwrap();

}

#[tauri::command]
fn get_password(state: tauri::State<Mutex<Context>>, id: u32) -> Result<String, String> {

  let context = match acquire_context_lock(&state) {
    Ok(ctx) => ctx,
    Err(_) => return Err("Failed to acquire context".into()),
  };

  let conn = match context.db.as_ref() {
    Some(conn) => conn,
    None => return Err("Failed to get database reference".into()),
  };


  let password: String = match sqlcipher_block!(conn, base64::encode(context.account_key.unsecure()), || -> Result<String, String> {

    let password : String = conn.query_row("SELECT * FROM Password WHERE password_id = ?", params![id], |row| {row.get(3)}).unwrap();

    let vault_key : String = conn.query_row("SELECT * FROM UserData LIMIT 1", [], |row| {row.get(1)}).unwrap();
  
    let password_pt = match decrypt(Cipher::aes_256_cbc(), &base64::decode(vault_key).unwrap(), None, &base64::decode(password).unwrap()) {
      Ok(pt) => pt,
      Err(_) => return Err("Failed to create cipher text".into()),
    };

    let password_string = match from_utf8(&password_pt) {
      Ok(str) => str,
      Err(_) => return Err("Failed to parse plain text bytes into string".into()),
    };

    Ok(String::from(password_string))
  }){
    Ok(str) => str,
    Err(err) => return Err(err)
  }; 


  Ok(password)
}

#[tauri::command]
fn add_new_password(state: tauri::State<Mutex<Context>>, source: String, username: String, password: String, image: String) -> Result<(), String> {
  
  let context = match acquire_context_lock(&state) {
    Ok(ctx) => ctx,
    Err(_) => return Err("Failed to acquire context".into()),
  };


  let conn = match context.db.as_ref() {
    Some(conn) => conn,
    None => return Err("Failed to get database reference".into()),
  };


  return sqlcipher_block!(conn, base64::encode(context.account_key.unsecure()), || -> Result<(), String>  {

    let mut key = match get_vault_key(conn){
      Ok(key) => key,
      Err(e) => return Err(e)
    };
  
    let password_cipher_text = match encrypt(Cipher::aes_256_cbc(), &key, None, &password.as_bytes()) {
      Ok(cipher) => cipher,
      Err(_) => return Err("failed to encrypt password".into()),
    };
  
    key.zeroize();
  
    match conn.execute("INSERT INTO Password (source, username, password, added, icon) VALUES (?1, ?2, ?3, ?4, ?5)", params![source, username, base64::encode(password_cipher_text), lib::time::get_current_secs(), image]){
      Ok(_) => println!("Inserted New Password Item!"),
      Err(_) => return Err("Failed to insert password into vault".into())
    }


    Ok(())
  });

}

#[tauri::command]
fn generate_password(state: tauri::State<Mutex<Context>>) -> Result<String, String>{

  let _context = match acquire_context_lock(&state) {
    Ok(ctx) => ctx,
    Err(_) => return Err("Failed to acquire context".into()),
  };

  Ok(lib::crypto::generate_password())
}

#[tauri::command]
fn logout(state: tauri::State<Mutex<Context>>) -> Result<(), String>  {

  let mut context = match acquire_context_lock(&state) {
    Ok(ctx) => ctx,
    Err(_) => return Err("Failed to acquire context".into()),
  };

  context.logged_in = false;

  context.account_key.zero_out();

  Ok(())
}

#[tauri::command]
fn is_vault_setup() -> bool {
  return path::Path::new("./db/database.db").exists() && path::Path::new("./keys").exists();
}

#[tauri::command]
fn login(state: tauri::State<Mutex<Context>>, master_key: String) -> Result<Vault, String> {

  let keys_string = match fs::read_to_string("./keys"){
    Ok(str) => str,
    Err(_) => return Err("Failed to read keys file".into())
  };

  let argon2 = Argon2::default();

  let mut keys = keys_string.lines();

  let sk_cipher_b64 = keys.next().unwrap();

  let sk_cipher_raw = match base64::decode(sk_cipher_b64) {
    Ok(raw) => raw,
    Err(_) => return Err("Failed to decode secret key".into())
  };

  keys.next().unwrap();

  let salt = keys.next().unwrap();

  let mk_hash = match argon2.hash_password(master_key.as_bytes(), &salt){
    Ok(hash) => hash,
    Err(_) => return Err("Failed to hash master key".into())
  };

  let mk_hash_bytes = mk_hash.hash.unwrap();

  let sk = match decrypt(Cipher::aes_256_cbc(), &mk_hash_bytes.as_bytes(), None, &sk_cipher_raw){
    Ok(plain) => plain,
    Err(error) => panic!("Error while decrypting secret key: {}", error)
  };

  let mut raw_auk : Vec<u8> = sk.iter().zip(mk_hash_bytes.as_bytes()).map(|(x, y)| x ^ y).collect();

  let mut context = match acquire_context_lock(&state) {
    Ok(ctx) => ctx,
    Err(_) => return Err("Failed to acquire context".into()),
  };

  context.logged_in = true;

  let conn = match context.db.as_ref() {
    Some(conn) => conn,
    None => return Err("Failed to get database reference".into()),
  };


  raw_auk.zeroize();

  let vault = match sqlcipher_block!(conn, base64::encode(context.account_key.unsecure()), || -> Result<Vault, String> {

    let passwords = match get_passwords(conn){
      Ok(pwds) => pwds,
      Err(err) => return Err(err)
    };

    let user_data = match get_userdata(conn) {
      Ok(ud) => ud,
      Err(err) => return Err(err)
    };

    Ok(Vault{
      passwords: passwords,
      user_data: user_data
    })

  }){
    Ok(vault) => vault,
    Err(err) => return Err(err)
  };

  Ok(vault)
  
}

#[tauri::command]
fn generate_mnemonic() -> Result<Mnemonic, String> {

  let mut rng = rand::thread_rng();

  let phrase = match Mnemonic::generate_in_with(&mut rng, Language::English, 12){
    Ok(phr) => phr,
    Err(_) => return Err("Failed to generate mnemonic phrase".into())
  };

  Ok(phrase)
}

#[tauri::command]
fn setup_vault(state: tauri::State<Mutex<Context>>, master_key: String, pass_phrase: String) -> bool {

  let mut buf = [0; 32];

  rand_bytes(&mut buf).unwrap(); // generate a secret key

  let salt = SaltString::generate(&mut OsRng); // generate salt

  let argon2 = Argon2::default();

  let master_key_hash = argon2.hash_password(master_key.as_bytes(), &salt).unwrap();

  let pass_phrase_hash = argon2.hash_password(pass_phrase.as_bytes(), &salt).unwrap();


  let mk_hash_bytes = master_key_hash.hash.unwrap();

  let mut master_key_private_key_cipher_text = match encrypt(Cipher::aes_256_cbc(), &mk_hash_bytes.as_bytes(), None, &buf) {
    Ok(cipher) => cipher,
    Err(error) => panic!("Failed to create cipher text {:?}", error),
  };

  let mut pass_phrase_private_key_cipher_text = match encrypt(Cipher::aes_256_cbc(), &pass_phrase_hash.hash.unwrap().as_bytes(), None, &buf) {
    Ok(cipher) => cipher,
    Err(error) => panic!("Failed to create cipher text {:?}", error),
  };

  let mut mk_hex = base64::encode(&master_key_private_key_cipher_text);
  master_key_private_key_cipher_text.zeroize();

  let mut pp_hex = base64::encode(&pass_phrase_private_key_cipher_text);
  pass_phrase_private_key_cipher_text.zeroize();


  fs::write("./keys", format!("{}\n{}\n{}", mk_hex, pp_hex, salt.as_str())).expect("Unable to write key file");

  mk_hex.zeroize();
  pp_hex.zeroize();
  
  let mut raw_auk : Vec<u8> = lib::crypto::generate_account_key(&buf, mk_hash_bytes.as_bytes()).unwrap(); // xor secret key with master key hash

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


  let vault_key_salt = &SaltString::generate(&mut OsRng);

  let vault_key = argon2.hash_password(&buf, vault_key_salt).unwrap();


  match conn.execute("INSERT INTO UserData (vault_key, last_unlock) VALUES (?1, ?2)", params![base64::encode(vault_key.hash.unwrap().as_bytes()), lib::time::get_current_secs()]){
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

