use std::env;
use sha2::{Sha512, Digest};

pub fn to_hex_string(bytes: Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter()
                                 .map(|b| format!("{:02x}", b))
                                 .collect();
    strs.join("")
}

fn main() -> Result<(), String>  {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1
    {
        println!("Usage : {} Public_key", &args[0]);
        return Err("invalid argument".to_string());
    }
    let public_key = &args[1];
    //println!("public key : {:}", &args[1]);
    let hash = Sha512::digest(public_key.as_bytes());

    let  str_hash = to_hex_string(hash[34..64].to_vec());
    //println!("sighash    : {:}",str_hash );    
    println!("{}", &str_hash);
    return Ok(());
}
