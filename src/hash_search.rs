extern crate base64;
// extern crate crypto;
extern crate rand;
extern crate regex;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use base64::encode;
use regex::Regex;

pub fn main() {
    // let search = "oeisorg";
    let re = Regex::new(r"oeis").unwrap();
    println!("hello");

    for n in 1..10 {
        let seed = format!("{}{}", rand_str(), n);
        let b64str = encode(&seed);
        // read hash digest

        println!("{}", &b64str);

        if re.is_match(&b64str) {
            println!("{}", seed);
        }
    }
}

fn rand_str() -> String {
    return thread_rng().sample_iter(&Alphanumeric).take(30).collect();
}
