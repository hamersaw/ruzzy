use std;

pub fn levenshtein(a: &str, b: &str) -> u16 {
    if a == b {
        return 0;
    } else if a.len() == 0 {
        return b.len() as u16;
    } else if b.len() == 0 {
        return a.len() as u16;
    }

    let mut v0: Vec<u16> = (0..(b.len() + 1) as u16).collect();
    let mut v1: Vec<u16> = (0..(b.len() + 1) as u16).collect();
    for (i, ca) in a.chars().enumerate() {
        v1[0] = i as u16 + 1;

        for (j, cb) in b.chars().enumerate() {
            v1[j+1] = std::cmp::min(
                    std::cmp::min(
                        v1[j] + 1,
                        v0[j+1] + 1
                    ),
                    v0[j] + if ca == cb { 0 } else { 1 }
                );
        }

        for j in 0..v0.len() {
            v0[j] = v1[j];
        }
    }

    v1[b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_empty() {
        assert_eq!(levenshtein("", ""), 0);
    }

    #[test]
    fn compare_eq_small() {
        assert_eq!(levenshtein("x", "x"), 0);
    }
    
    #[test]
    fn compare_ne_small() {
        assert_eq!(levenshtein("x", "y"), 1);
    }

    #[test]
    fn compare_eq_large() {
        assert_eq!(levenshtein("alphabet", "alphabet"), 0);
    }

    #[test]
    fn compare_ne_large() {
        assert_eq!(levenshtein("alphabet", "alphabte"), 2);
    }
}
