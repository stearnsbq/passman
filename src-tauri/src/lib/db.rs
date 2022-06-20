use rusqlite::Connection;

use crate::types::Password;



pub fn get_passwords(conn: &Connection) -> Result<Vec<Password>, String>{

    let mut passwords_stmt = match conn.prepare("SELECT * FROM Password"){
        Ok(stmt) => stmt,
        Err(_) => return Err("Failed to create password sql statement".into())
    };

    let passwords_stmt_iter = match passwords_stmt.query_map([], |row| {
        Ok(
          Password{
            password_id: row.get(0)?,
            source: row.get(1)?,
            username: row.get(2)?,
            added: row.get(4)?,
            icon: row.get(5)?
          }
        )
      }){
        Ok(iter) => iter,
        Err(_) => return Err("Failed to query password rows".into())
      };
    
      let mut passwords = Vec::new();
    
      for password in passwords_stmt_iter {

        let pwd = match password {
            Ok(pd) => pd,
            Err(_) => return Err("Failed to get password row".into()) ,
        };

        passwords.push(pwd);
      }

      return Ok(passwords)

}