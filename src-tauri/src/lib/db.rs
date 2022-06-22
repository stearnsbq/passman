use rusqlite::Connection;

use crate::types::{Password, UserData};

pub fn get_passwords(conn: &Connection) -> Result<Vec<Password>, String> {
    let mut passwords_stmt = match conn.prepare("SELECT * FROM Password") {
        Ok(stmt) => stmt,
        Err(e) => return Err(e.to_string()),
    };

    let passwords_stmt_iter = match passwords_stmt.query_map([], |row| {
        Ok(Password {
            password_id: row.get(0)?,
            source: row.get(1)?,
            username: row.get(2)?,
            added: row.get(4)?,
            icon: row.get(5)?,
        })
    }) {
        Ok(iter) => iter,
        Err(_) => return Err("Failed to query password rows".into()),
    };

    let mut passwords = Vec::new();

    for password in passwords_stmt_iter {
        let pwd = match password {
            Ok(pd) => pd,
            Err(_) => return Err("Failed to get password row".into()),
        };

        passwords.push(pwd);
    }

    return Ok(passwords);
}

pub fn get_userdata(conn: &Connection) -> Result<UserData, String> {
    let user_data: UserData = match conn.query_row("SELECT * FROM UserData LIMIT 1", [], |row| {
        Ok(UserData {
            userdata_id: row.get(0)?,
            last_unlock: row.get(2)?,
        })
    }) {
        Ok(data) => data,
        Err(_) => return Err("Failed to get user data".into()),
    };

    Ok(user_data)
}

pub fn get_vault_key(conn: &Connection) -> Result<Vec<u8>, String> {
    let vault_key: String =
        match conn.query_row("SELECT * FROM UserData LIMIT 1", [], |row| row.get(1)) {
            Ok(data) => data,
            Err(_) => return Err("Failed to get vault key".into()),
        };

    let key = match base64::decode(vault_key) {
        Ok(vec) => vec,
        Err(_) => return Err("Failed to decode key".into()),
    };

    Ok(key)
}


pub fn get_password(conn: &Connection, id: u32){
  
}