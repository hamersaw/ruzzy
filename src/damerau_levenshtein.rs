use std;

pub fn compare(a: &str, b: &str) -> u16 {
    if a == b {
        return 0;
    } else if a.len() == 0 {
        return b.len() as u16;
    } else if b.len() == 0 {
        return a.len() as u16;
    }

    let mut d = vec!();
    for _ in 0..(a.len() + 1) as u16 {
        d.push(vec!(0; a.len() + 1));
    }

    for i in 0..d.len() {
        d[i][0] = i as u16;
        d[0][i] = i as u16;
    }

    for i in 1..a.len() + 1 {
        let ca = a.chars().nth(i-1).unwrap();

        for j in 1..b.len() + 1 {
            let cb = b.chars().nth(j-1).unwrap();
            let cost = if ca == cb { 0 } else { 1 };

            d[i][j] = std::cmp::min(
                    std::cmp::min(
                        d[i-1][j] + 1,
                        d[i][j-1] + 1
                    ),
                    d[i-1][j-1] + cost
                );

            if i > 1 && j > 1 && ca == b.chars().nth(j-2).unwrap() && a.chars().nth(i-2).unwrap() == cb {
                d[i][j] = std::cmp::min(d[i][j], d[i-2][j-2] + cost);
            }
        }
    }

    d[a.len()][b.len()]
}

#[test]
fn compare_empty() {
    assert_eq!(compare("", ""), 0);
}

#[test]
fn compare_eq_small() {
    assert_eq!(compare("x", "x"), 0);
}

#[test]
fn compare_ne_small() {
    assert_eq!(compare("x", "y"), 1);
}

#[test]
fn compare_eq_large() {
    assert_eq!(compare("alphabet", "alphabet"), 0);
}

#[test]
fn compare_ne_large() {
    assert_eq!(compare("alphabet", "alphabte"), 1);
}
