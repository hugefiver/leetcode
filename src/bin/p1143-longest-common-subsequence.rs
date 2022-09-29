impl Solution {
    pub fn longest_common_subsequence(s1: String, s2: String) -> i32 {
        if s1.len() == 0 || s2.len() == 0 {
            return 0;
        }

        let mut dp = vec![vec![0; s2.len()]; s1.len()];
        for (i, x) in s1.chars().enumerate() {
            for (j, y) in s2.chars().enumerate() {
                dp[i][j] = if x == y {
                    if i == 0 || j == 0 {
                        1
                    } else {
                        dp[i - 1][j - 1] + 1
                    }
                } else {
                    match (i > 0, j > 0) {
                        (true, true) => dp[i - 1][j].max(dp[i][j - 1]),
                        (true, false) => dp[i - 1][j],
                        (false, true) => dp[i][j - 1],
                        _ => 0,
                    }
                }
            }
        }
        *dp.last().unwrap().last().unwrap()
    }

    pub fn longest_common_subsequence_1(s1: String, s2: String) -> i32 {
        use std::collections::HashMap;

        if s1.len() == 0 || s2.len() == 0 {
            return 0;
        }

        let mut map: HashMap<(usize, usize), i32> = HashMap::new();
        return foo(&mut map, &s1, &s2, 0, 0);

        fn foo(
            map: &mut HashMap<(usize, usize), i32>,
            s1: &str,
            s2: &str,
            i: usize,
            j: usize,
        ) -> i32 {
            if i == s1.len() || j == s2.len() {
                return 0;
            }
            if let Some(ret) = map.get(&(i, j)) {
                *ret
            } else {
                let ret = if s1.as_bytes()[i] == s2.as_bytes()[j] {
                    1 + foo(map, s1, s2, i + 1, j + 1)
                } else {
                    let x = foo(map, s1, s2, i + 1, j);
                    let y = foo(map, s1, s2, i, j + 1);
                    x.max(y)
                };
                map.insert((i, j), ret);
                ret
            }
        }
    }
}

struct Solution;

fn main() {
    Solution::longest_common_subsequence_1("abcde".to_string(), "ace".to_string());
}
