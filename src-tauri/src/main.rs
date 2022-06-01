#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

extern crate rand;
extern crate bip39;

use bip39::{Mnemonic, Language};


fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![generateMnemonic])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}



#[tauri::command]
fn generateMnemonic() -> Mnemonic {

  let mut rng = rand::thread_rng();

  let phrase = Mnemonic::generate_in_with(&mut rng, Language::English, 12).unwrap();

  return phrase;
}

#[tauri::command]
fn setupVault() {
  

}