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
