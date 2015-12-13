pub fn soundex(a: &str, b: &str) -> bool {
    encode_soundex(a) == encode_soundex(b)
}

pub fn encode_soundex(s: &str) -> String {
    if s.len() == 0 {
        return "".to_string();
    }
    
    let first_char = s.to_lowercase().chars().nth(0).unwrap();
    let mut v : Vec<u8> = s[1..].to_lowercase().chars().map(|x| 
            match x {
                'b' | 'f' | 'p' | 'v' => '1',
                'c' | 'g' | 'j' | 'k' | 'q' | 's' | 'x' | 'z' => '2',
                'd' | 't' => '3',
                'l' => '4',
                'm' | 'n' => '5' ,
                'r' => '6',
                _ => ' ',
            }
        ).filter(|x| *x != ' ').map(|x|
            x as u8                            
        ).collect();

    v.dedup();

    let digits = String::from_utf8(v).unwrap();
    match digits.len() {
        0...2 => format!("{}{digits:>width$}", first_char, digits=digits, width=3).replace(" ","0"),
        3 => format!("{}{}", first_char, digits),
        _ => format!("{}{}", first_char, &digits[..2]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_empty() {
        assert!(soundex("", ""));
    }

    #[test]
    fn compare_eq_small() {
        assert!(soundex("x", "x"));
    }
    
    #[test]
    fn compare_ne_small() {
        assert!(!soundex("x", "y"));
    }

    #[test]
    fn compare_eq_large() {
        assert!(soundex("alphabet", "alphabet"));
    }

    #[test]
    fn compare_ne_large() {
        assert!(soundex("alphabet", "alphabte"));
    }
}
