fn lcs(x: &str, y: &str) -> String {
    let mut dp = vec![vec![0usize; y.len()]; x.len()];

    if x.len() == 0 || y.len() == 0 {
        return "".to_string();
    }

    let s1: Vec<_> = x.chars().collect();
    let s2: Vec<_> = y.chars().collect();
    let (mut idx, mut max_val) = ((0, 0), 0);
    for (i, &x) in s1.iter().enumerate() {
        for (j, &y) in s2.iter().enumerate() {
            if x == y {
                let k = if i == 0 || j == 0 {
                    1
                } else {
                    dp[i-1][j-1] + 1
                };
                if k > max_val {
                    max_val = k;
                    idx = (i, j);
                }
                dp[i][j] = k;
            } else {
                dp[i][j] = if i > 0 && j > 0 {
                    dp[i-1][j].max(dp[i][j-1])
                } else if i > 0 {
                    dp[i-1][j]
                } else if j > 0{
                    dp[i][j-1]
                } else {
                    0
                }
            }
        }
    }

    if max_val == 0 {
        "".to_string()
    } else {
        let mut seq = Vec::with_capacity(max_val);
        let (mut i, mut j) = idx;
        while seq.len() < max_val {
            if s1[i] == s2[j] {
                seq.push(s1[i]);
                if i == 0 || j == 0 {
                    break;
                }
                i -= 1;
                j -= 1;
            } else {
                if i > 0 && dp[i][j] == dp[i-1][j] {
                    i -= 1;
                } else {
                    j -= 1;
                }
            }
        }
        seq.into_iter().rev().collect()
    }
}


fn lcs_2(xx: &str, yy: &str) -> String {
    use std::cmp;
    let x = xx.chars().collect::<Vec<_>>();
    let y = yy.chars().collect::<Vec<_>>();
    let l = x.len();
    let m = y.len();
    let mut v = vec![vec!["".to_owned(); m + 1]; l + 1];
    for (i, j) in itertools::iproduct!(1..=l, 1..=m) {
        v[i][j] =   if x[i - 1] == y[j - 1] { 
                        format!("{}{}", v[i - 1][j - 1], x[i - 1]) 
                    } else { 
                        cmp::max_by_key(v[i - 1][j].to_owned(), v[i][j - 1].to_owned(), |x| x.len())
                    }
    }
    v[l][m].to_owned()
}

fn lcs_3(x: &str, y: &str) -> String {
    let a: &[u8] = x.as_bytes(); let b: &[u8] = y.as_bytes();
    let mut table: Vec<Vec<String>> = vec![vec![String::new(); b.len() + 1]; a.len() + 1];
    for i in 1..table.len() {
        for j in 1..table[i].len() {
            table[i][j] = if a[i - 1] == b[j - 1] {
                let mut s = table[i - 1][j - 1].clone();
                s.push(a[i - 1] as char);
                s
            } else {
                match (&table[i - 1][j], &table[i][j - 1]) {
                    (a, b) if a.len() > b.len() => a.clone(),
                    (_, b) => b.clone()
                }
            }
        }
    }

    table.last().unwrap().last().unwrap().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_tests() {
        assert_eq!(lcs("", ""), "");
        assert_eq!(lcs("abc", ""), "");
        assert_eq!(lcs("", "abc"), "");
        assert_eq!(lcs("a", "b"), "");
        assert_eq!(lcs("a", "a"), "a");
        assert_eq!(lcs("abc", "a"), "a");
        assert_eq!(lcs("abc", "ac"), "ac");
        assert_eq!(lcs("abcdef", "abc"), "abc");
        assert_eq!(lcs("abcdef", "acf"), "acf");
        assert_eq!(lcs("anothertest", "notatest"), "nottest");
        assert_eq!(lcs("132535365", "123456789"), "12356");
        assert_eq!(lcs("nothardlythefinaltest", "zzzfinallyzzz"), "final");
        assert_eq!(lcs("abcdefghijklmnopq", "apcdefghijklmnobq"), "acdefghijklmnoq");
    }

    #[test]
    fn fixed_tests_2() {
        assert_eq!(lcs_2("", ""), "");
        assert_eq!(lcs_2("abc", ""), "");
        assert_eq!(lcs_2("", "abc"), "");
        assert_eq!(lcs_2("a", "b"), "");
        assert_eq!(lcs_2("a", "a"), "a");
        assert_eq!(lcs_2("abc", "a"), "a");
        assert_eq!(lcs_2("abc", "ac"), "ac");
        assert_eq!(lcs_2("abcdef", "abc"), "abc");
        assert_eq!(lcs_2("abcdef", "acf"), "acf");
        assert_eq!(lcs_2("anothertest", "notatest"), "nottest");
        assert_eq!(lcs_2("132535365", "123456789"), "12356");
        assert_eq!(lcs_2("nothardlythefinaltest", "zzzfinallyzzz"), "final");
        assert_eq!(lcs_2("abcdefghijklmnopq", "apcdefghijklmnobq"), "acdefghijklmnoq");
    }
}

