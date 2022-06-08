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
use std::time;

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


fn main() {


  println!("started");
 // login(String::from("U2eolreredcira!!")).unwrap();

  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![generate_mnemonic, setup_vault, login, is_vault_setup])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");



    
}

#[tauri::command]
fn is_vault_setup() -> bool {
  return path::Path::new("./db/database.db").exists() && path::Path::new("./keys").exists();
}


#[tauri::command]
fn login(master_key: String) -> Result<Vault, String> {

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

  let conn = Connection::open("./db/database.db").unwrap();

  conn.pragma_update(None, "key", base64::encode(&raw_auk)).unwrap();

  raw_auk.zeroize();


  let mut passwords_stmt = conn.prepare("SELECT * FROM Password").unwrap();

  let passwords_stmt_iter = passwords_stmt.query_map([], |row| {
    Ok(
      Password{
        password_id: row.get(0)?,
        source: row.get(1)?,
        password: row.get(2)?,
        icon: row.get(3)?
      }
    )
  }).unwrap();


  let mut user_data_stmt = conn.prepare("SELECT * FROM UserData").unwrap();

  let mut user_data_iter = user_data_stmt.query_map([], |row| {
    Ok(
      UserData{
        userdata_id: row.get(0)?,
        vault_key: row.get(1)?,
        last_unlock: row.get(2)?,
      }
    )


  }).unwrap();


  let user_data = user_data_iter.next().unwrap();

  let mut passwords = Vec::new();

  for password in passwords_stmt_iter {
    passwords.push(password.unwrap());
  }


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
fn setup_vault(master_key: String, pass_phrase: String) -> bool {

  /*
  Steps:
  1. Generate Secret Key
  2. Encrypt Secret Key with master key
  3. Encrypt Secret Key with pass phrase as backup
  4. Store Keys on disc
  5. Setup account key
  6. Setup sqlite database
  7. generate private key for decrypting passwords
  8. store user info in database
  9. encrypt database
  10. return to user that process is finished and redirect them to login

  */


  let mut buf = [0; 32];

  rand_bytes(&mut buf).unwrap(); // generate a secret key

  println!("MK : {}", master_key);

  let master_key_hash = sha256(master_key.as_bytes());

  println!("MK HASH: {}", base64::encode(&master_key_hash));

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
  
  let mut raw_auk : Vec<u8> = buf.iter().zip(master_key_hash).map(|(x, y)| x ^ y).collect(); // xor secret key with master key hash

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

  let start = time::SystemTime::now();
  let since_the_epoch = start
      .duration_since(time::SystemTime::UNIX_EPOCH)
      .expect("Time went backwards");

  match conn.execute("INSERT INTO UserData (vault_key, last_unlock) VALUES (?1, ?2)", params![base64::encode(buf), since_the_epoch.as_secs()]){
    Ok(_) => println!("Inserted Item!"),
    Err(error) => panic!("Err: {}", error)
  }

  buf.zeroize();

  return true;
}



#[derive(Debug, serde::Serialize)]
struct Password{
  password_id: u32,
  source: String,
  password: String,
  icon: Option<Vec<u8>>
}


#[derive(Debug, serde::Serialize)]
struct UserData{
  userdata_id : u32,
  vault_key: String,
  last_unlock: u32,
}


#[derive(Debug, serde::Serialize)]
struct Vault{
  user_data: UserData,
  passwords: Vec<Password>
}