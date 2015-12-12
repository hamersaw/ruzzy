pub fn ngram(a: &str, b: &str, size: usize) -> f64 {
    if a == b {
        return 1.0;
    }

    //loop through first string add unique ngrams to vec
    let mut ngrams = Vec::new();
    for ngram in compute_ngram_tokens(a, size) {
        if !ngrams.contains(&ngram) {
            ngrams.push(ngram);
        }
    }

    //loop through second string
    let mut intersection = 0;
    let mut difference = 0;
    for ngram in compute_ngram_tokens(b, size) {
        if ngrams.contains(&ngram) {
            intersection += 1
        } else {
            difference += 1;
        }
    }

    intersection as f64 / ((ngrams.len() as i32 + difference) as f64)
}

pub fn compute_ngram_tokens(s: &str, size: usize) -> Vec<String> {
    let mut tokens = Vec::new();

    if s.len() < size {
        tokens.push(format!("{s:>width$}", s=s, width=size));
    } else {
        for i in 0..(s.len() - size + 1) {
            unsafe {
                tokens.push(s.slice_unchecked(i, i + size).to_string());
            }
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_empty() {
        assert_eq!(ngram("", "", 3), 1.0);
    }

    #[test]
    fn compare_eq_small() {
        assert_eq!(ngram("x", "x", 3), 1.0);
    }
    
    #[test]
    fn compare_ne_small() {
        assert!(ngram("x", "y", 3) < 1.0);
    }

    #[test]
    fn compare_eq_large() {
        assert_eq!(ngram("alphabet", "alphabet", 3), 1.0);
    }

    #[test]
    fn compare_ne_large() {
        assert!(ngram("alphabet", "alpha", 3) < 1.0);
    }
}
