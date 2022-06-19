use rand::{seq::SliceRandom, thread_rng};

pub fn generate_account_key(master_key_hash: &[u8], secret_key: &[u8]) -> Result<Vec<u8>, &'static str>{

    let auk : Vec<u8> = secret_key.iter().zip(master_key_hash).map(|(x, y)| x ^ y).collect();

    Ok(auk)
}


pub fn generate_password() -> String {
    let char_array = generate_char_array();
    let mut rng = thread_rng();
    (0..12)
        .map(|_| char_array.choose(&mut rng).unwrap().to_owned() as char)
        .collect()
}

fn generate_char_array() -> Vec<u8> {
    const ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMBERS: &str = "0123456789";
    const SYMBOLS: &str = "!@#$%^&*_-+=";
    let mut charset = ALPHA.to_string();
    charset.push_str(NUMBERS);
    charset.push_str(SYMBOLS);
    return charset.into_bytes()
}