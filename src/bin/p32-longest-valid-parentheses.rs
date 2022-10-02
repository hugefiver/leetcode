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
}

struct Solution;

fn main() {}
