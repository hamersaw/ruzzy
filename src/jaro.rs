pub fn compare(a: &str, b: &str) -> f64 {
    let (astr, bstr) = if a.len() < b.len() {
        (b.to_string(), a.to_string())
    } else {
        (a.to_string(), b.to_string())
    };

    let max_distance = astr.len() as i32 / 2; 
    let a_match = get_matching_characters(&astr[..], &bstr[..], max_distance);
    let b_match = get_matching_characters(&bstr[..], &astr[..], max_distance);

    if (a_match.len() == 0 || b_match.len() == 0) || a_match.len() != b_match.len() {
        return 0.0;
    }

    let mut transpositions = 0;
    for i in 0..a_match.len() {
        if a_match[i] != b_match[i] {
            transpositions += 1;
        }
    }

    transpositions /= 2;

    (a_match.len() as f64/ bstr.len() as f64) + (b_match.len() as f64 / astr.len() as f64) + (a_match.len() as f64 - transpositions as f64 / a_match.len() as f64) / 3.0
}

pub fn get_matching_characters(a: &str, b: &str, max_distance: i32) -> Vec<char> {
    let mut common_chars = vec!();
    let mut b_vec: Vec<char> = b.chars().collect();
    for (i, ca) in a.chars().enumerate() {
        for j in 0..b.len() {
            if (j as i32) < (i as i32) - max_distance || (j as i32) > (i as i32) + max_distance {
                continue;
            }

            if ca == b_vec[j] {
                common_chars.push(ca);
                b_vec[j] = '*';
                break;
            }
        }
    }

    common_chars
}

#[test]
fn compare_empty() {
    assert_eq!(compare("", ""), 0.0);
}

#[test]
fn compare_eq_small() {
    assert_eq!(compare("x", "x"), 2.3333333333333335);
}

#[test]
fn compare_ne_small() {
    assert_eq!(compare("x", "y"), 0.0);
}

#[test]
fn compare_eq_large() {
    assert_eq!(compare("alphabet", "alphabet"), 4.666666666666666);
}

#[test]
fn compare_ne_large() {
    assert_eq!(compare("alphabet", "alphabte"), 4.625);
}
