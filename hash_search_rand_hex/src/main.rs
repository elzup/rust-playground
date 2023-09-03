extern crate hex;
extern crate rand;
extern crate sha2;

use hex::encode;
use sha2::{Digest, Sha256};

fn random_hex_string(length: usize) -> String {
    let bytes: Vec<u8> = (0..length).map(|_| rand::random::<u8>()).collect();
    encode(&bytes)
}

fn check_sha256_prefix(input: &str, prefix: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();

    let hex_result = hex::encode(result);

    hex_result.starts_with(prefix)
}

fn main() {
    let mut count = 0u32;

    // const searchB32 = 'MUNISYSTEM'
    let prefix = "651a89625323";
    // let prefix = "abcd";

    loop {
        count += 1;
        let s = random_hex_string(16);

        if check_sha256_prefix(&s, prefix) {
            println!("{}", s);
            break;
        }

        if count % 1000000 == 0 {
            println!("{}: {}", count, s);
        }
    }
}
