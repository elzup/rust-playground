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

fn main() {
    let mut s = "ZZZY".to_string();
    for _ in 0..10 {
        println!("{}", s);
        s = increment_string(&s).unwrap();
    }
}
