extern crate hex;
extern crate sha2;

fn increment_char(ch: &mut char, carry: &mut bool) {
    *carry = false;
    match *ch {
        '0'..='8' | 'a'..='y' | 'A'..='Y' => {
            *ch = ((*ch as u8) + 1) as char;
        }
        '9' => {
            *ch = 'a';
        }
        'z' => {
            *ch = 'A';
        }
        'Z' => {
            *ch = '0';
            *carry = true;
        }
        _ => {}
    }
}

fn increment_string(s: &str) -> Option<String> {
    let mut chars: Vec<char> = s.chars().collect();
    let mut carry = false;

    for ch in chars.iter_mut().rev() {
        increment_char(ch, &mut carry);
        if !carry {
            break;
        }
    }

    if carry {
        chars.insert(0, '1');
    }

    Some(chars.iter().collect())
}

use sha2::{Digest, Sha256};

fn check_sha256_prefix(input: &str, prefix: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();

    let hex_result = hex::encode(result);

    // hex文字列が指定されたプレフィックスで始まるかどうかをチェック
    hex_result.starts_with(prefix)
}

fn main() {
    let mut s = "a".to_string();
    let mut count = 0u32;

    // const searchB32 = 'MUNISYSTEM'
    let prefix = "651a89625323";
    // let prefix = "abcd";

    loop {
        count += 1;
        if check_sha256_prefix(&s, prefix) {
            println!("{}", s);
            break;
        }

        s = increment_string(&s).unwrap();
        if count % 1000000 == 0 {
            println!("{}: {}", count, s);
        }
    }
}
