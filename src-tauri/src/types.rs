use rusqlite::Connection;



#[derive(Debug, serde::Serialize)]
pub struct Password{
  pub password_id: u32,
  pub username: String,
  pub source: String,
  pub added: u32,
  pub icon: String
}


#[derive(Debug, serde::Serialize)]
pub struct UserData{
  pub userdata_id : u32,
  pub last_unlock: u32,
}


#[derive(Debug, serde::Serialize)]
pub struct Vault{
  pub user_data: UserData,
  pub passwords: Vec<Password>
}


#[derive(Debug)]
pub struct Context{
  pub db: Option<Connection>,
  pub logged_in: bool,
  pub account_key: secstr::SecStr
}




