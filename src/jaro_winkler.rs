use jaro;

pub fn compare(a: &str, b: &str, scaling_factor: f32) -> f64 {
    let jaro_score = jaro::compare(a, b);

    let mut prefix_length = 0;
    let (a_vec, b_vec) = (a.chars().collect::<Vec<char>>(), b.chars().collect::<Vec<char>>());
    for i in 0..a.len() {
        if a_vec[i] == b_vec[i] {
            prefix_length += 1;
        } else {
            break;
        }
    }

    jaro_score + (scaling_factor as f64 * ( if prefix_length > 4 { 4 } else { prefix_length} ) as f64 * (1.0 - jaro_score))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_empty() {
        assert_eq!(compare("", "", 0.1), 0.0);
    }

    #[test]
    fn compare_eq_small() {
        assert_eq!(compare("x", "x", 0.1), 2.199999998013179);
    }
    
    #[test]
    fn compare_ne_small() {
        assert!(compare("x", "y", 0.1) < 1.0);
    }

    #[test]
    fn compare_eq_large() {
        assert_eq!(compare("alphabet", "alphabet", 0.1), 3.1999999781449633);
    }

    #[test]
    fn compare_ne_large() {
        assert_eq!(compare("alphabet", "alphabte", 0.1), 3.1749999783933163);
    }
}
