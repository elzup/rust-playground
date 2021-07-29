extern crate base64;
// extern crate crypto;
extern crate rand;
extern crate regex;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use base64::encode;
use regex::Regex;
use std::env;

trait BinarySearch<T> {
    fn lower_bound(&self, f: impl Fn(&T) -> bool) -> usize;
}

impl<T> BinarySearch<T> for [T] {
    fn lower_bound(&self, f: impl Fn(&T) -> bool) -> usize {
        let mut left: isize = -1;
        let mut right = self.len() as isize;

        while right - left > 1 {
            let mid = (left + right) / 2;
            if f(&self[mid as usize]) {
                right = mid;
            } else {
                left = mid;
            }
        }

        right as usize
    }
}

pub fn main() {
    let re = Regex::new(search).unwrap();

    for _n in 1..100000 {
        let seed = &format!("{}{}", salt, rand_str());
        let b64str = encode(seed);
// read hash digest

        if re.is_match(&b64str) {
            println!("{}", &b64str);
            println!("{}", seed);
            break;
        }
    }
}

fn rand_str() -> String {
    return thread_rng().sample_iter(&Alphanumeric).take(30).collect();
}
