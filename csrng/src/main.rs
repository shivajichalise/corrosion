use std::{env, process};

use aes::{
    cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit},
    Aes256,
};
use rand::{
    distributions::{Alphanumeric, DistString},
    rngs::OsRng,
};

fn main() {
    let secret_key = process_args(env::args()).expect("Failed to get secret string");

    let mut counter: u64 = 0;
    let mut key = [0u8; 32];

    key.copy_from_slice(secret_key.as_bytes());

    for _ in 0..10 {
        let res = encrypt_counter(counter, &key);
        println!("CS-RNG: {}", res);
        counter += 1;
    }
}

fn encrypt_counter(counter: u64, key: &[u8; 32]) -> u64 {
    let mut buffer = [0; 16];

    // convert the u64 into bytes in big-edian format
    buffer[..8].copy_from_slice(&counter.to_be_bytes());

    // encrypt the buffer
    let encryptor = Aes256::new(key.into());

    encryptor.encrypt_block(&mut GenericArray::from_mut_slice(&mut buffer));

    // Convert the encrypted result back to u64
    let result_bytes: [u8; 16] = buffer;

    u64::from_be_bytes(result_bytes[..8].try_into().expect("Failed to convert"))
}

fn process_args(mut args: impl Iterator<Item = String>) -> Result<String, String> {
    args.next();

    let secret_key = match args.next() {
        Some(arg) => {
            if arg.len() < 32 {
                eprintln!("Please provide 32 characters long secret key.");
                process::exit(1);
            }
            arg[..32].to_string()
        }
        None => {
            println!("No secret key was provided thus generating one automatically.");
            Alphanumeric.sample_string(&mut OsRng, 32) // generaterandom string of
                                                       // given length
        }
    };

    Ok(secret_key)
}
