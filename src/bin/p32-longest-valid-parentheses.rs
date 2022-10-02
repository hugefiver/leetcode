impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        if s.len() < 2 {
            return 0;
        }

        let mut dp = vec![0; s.len()];
        let mut stack = Vec::new();
        let s: Vec<_> = s.chars().collect();

        // left to right
        for (idx, &ch) in s.iter().enumerate() {
            match ch {
                '(' => {
                    stack.push(ch);
                    // if idx == 0 {
                    //     dp[idx] = 1;
                    // } else {
                    //     dp[idx] = dp[idx - 1] + 1;
                    // }
                    dp[idx] = stack.len() as i32;
                }
                ')' => {
                    if !stack.is_empty() && stack.last().unwrap().clone() == '(' {
                        dp[idx] = stack.len() as i32;
                        stack.pop();
                    } else {
                        dp[idx] = 0;
                        stack.clear();
                    }
                }
                _ => unreachable!(),
            }
        }

        // right to left
        for (idx, &ch) in s.iter().enumerate().rev() {
            if dp[idx] != 0 {
                match ch {
                    ')' => {
                        stack.push(ch);
                        // if idx == dp.len() - 1 {
                        //     dp[idx] = 1;
                        // } else {
                        //     dp[idx] = dp[idx + 1] + 1;
                        // }

                        dp[idx] = stack.len() as i32;
                    }
                    '(' => {
                        if !stack.is_empty() && stack.last().unwrap().clone() == ')' {
                            dp[idx] = stack.len() as i32;
                            stack.pop();
                        } else {
                            dp[idx] = 0;
                            stack.clear();
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                stack.clear();
            }
        }

        dp.split(|&x| x == 0)
            .into_iter()
            .map(|xs| xs.len() as i32)
            .max()
            .unwrap_or(0)
    }

    fn longest_valid_parentheses_1(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut stack: Vec<i32> = vec![-1];
        let mut res = 0;

        for i in 0..s.len() {
            if s[i] == '(' {
                stack.push(i as i32);
            } else if s[i] == ')' {
                stack.pop();
                if stack.len() == 0 {
                    stack.push(i as i32);
                } else {
                    res = res.max(i as i32 - stack[stack.len() - 1])
                }
            } else {
                unreachable!();
            }
        }

        res
    }

    fn longest_valid_parentheses_2(s: String) -> i32 {
        // dp[i]: longest valid parentheses that ends with s[i]
        // dp[i] = max(2 + dp[i-2] if s[i-1] == '(' and s[i] == ')',
        //             2 + dp[i-1] + dp[i - 1 - dp[i-1] - 1]
        //             if s[i - 1 - dp[i-1]] == '(' and s[i] == ')' and s[i-1] == ')'
        //            )
        let n: usize = s.len();
        let s = s.into_bytes();
        let mut dp: Vec<i32> = vec![0; n];

        for i in 1..n {
            if s[i] == b'(' {
                continue;
            }
            // then bytes[i] must be ')'
            // if the last two char is '()'
            if s[i - 1] == b'(' {
                dp[i] = 2 + {
                    // checked access to dp
                    if i >= 2 {
                        dp[i - 2]
                    } else {
                        0
                    }
                }
            // if the char before「the valid parentheses which end with the second last ')'」 is '('
            } else if s.get(i - 1 - dp[i - 1] as usize) == Some(&b'(') {
                // chained with the valid parentheses before
                dp[i] = 2 + dp[i - 1] + dp.get(i - 1 - dp[i - 1] as usize - 1).unwrap_or(&0);
            }
        }

        dp.into_iter().max().unwrap_or(0)
    }

    fn longest_valid_parentheses_3(s: String) -> i32 {
        /*
            Time Complexity: O(n), Single traversal of String
            Space Complexity: O(n), DP  array of size of n is used
        */
        let mut stack = vec![-1];
        let mut ans = 0;

        for (i, ch) in s.char_indices() {
            match ch {
                '(' => stack.push(i as i32),
                _ => {
                    stack.pop();
                    if stack.is_empty() {
                        stack.push(i as i32);
                    } else {
                        if let Some(peek) = stack.last() {
                            ans = i32::max(ans, i as i32 - peek);
                            // println!("{stack:?}")
                        }
                    }
                }
            }
        }
        ans
    }

    fn longest_valid_parentheses_4(s: String) -> i32 {
        let mut valid = vec![0; s.len() + 1];
        let s = s.as_bytes();
        for (i, &c) in s.iter().enumerate() {
            if c == b'(' {
                continue;
            }
            if let Some(idx) = i.checked_sub(valid[i] + 1) {
                if s[idx] == b'(' {
                    valid[i + 1] = valid[i] + 2 + valid[idx];
                }
            }
        }
        valid.into_iter().max().unwrap_or(0) as i32
    }
}

impl Solution2 {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = vec![-1];
        let mut res = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                stack.push(i as i32)
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(i as i32)
                }
                res = res.max(i as i32 - stack.last().unwrap());
            }
        }
        res
    }
}

struct Solution;
struct Solution2;

fn main() {}
