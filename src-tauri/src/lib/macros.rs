
#[macro_export]
macro_rules! sqlcipher_block  {
    ($conn:expr , $key:expr , $e:expr) => {{
        
        $conn.pragma_update(None, "key", $key).unwrap();

        let _res = $e();
            
        $conn.pragma_update(None, "key", "").unwrap();

        _res
    }
    };
}