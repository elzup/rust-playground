// extern crate crypto;
extern crate base64;
extern crate rand;
extern crate regex;
use base64::engine::general_purpose;
use base64::Engine;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use regex::Regex;
use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let search = &args[1];
    let salt = &args[2];

    let re = Regex::new(search).unwrap();

    for _n in 1..100000 {
        let seed = &format!("{}{}", salt, rand_str());

        let b64str: String = general_purpose::STANDARD_NO_PAD.encode(seed);
        // read hash digest

        if re.is_match(&b64str) {
            println!("{}", &b64str);
            println!("{}", seed);
            break;
        }
    }
}

fn rand_str() -> String {
    return thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .take(30)
        .collect();
}
