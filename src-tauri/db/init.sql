CREATE TABLE IF NOT EXISTS Passwords(
  password_id INTEGER PRIMARY KEY,  
  source varchar(255)  NOT NULL,
  password varchar(255) NOT NULL,
  icon BLOB
);


CREATE TABLE IF NOT EXISTS UserData(
    userdata_id INTEGER PRIMARY KEY,
    vault_key varchar(32) NOT NULL,
    last_unlock INTEGER NOT NULL
);