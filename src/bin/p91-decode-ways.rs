impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let nums: Vec<_> = s.chars().filter_map(|x| x.to_digit(10)).collect();
        fn foo(nums: &Vec<u32>, dp: &mut Vec<Option<i32>>, idx: usize) -> i32 {
            if idx == nums.len() {
                1
            } else if idx == nums.len() - 1 {
                if nums[idx] == 0 {
                    0
                } else {
                    1
                }
            } else {
                if dp[idx].is_some() {
                    dp[idx].unwrap()
                } else {
                    let ret = match (nums[idx], nums[idx + 1]) {
                        (1, _) => foo(nums, dp, idx + 1) + foo(nums, dp, idx + 2),
                        (2, 0..=6) => foo(nums, dp, idx + 2) + foo(nums, dp, idx + 1),
                        (1..=9, _) => foo(nums, dp, idx + 1),
                        _ => 0,
                    };
                    dp[idx].replace(ret);
                    ret
                }
            }
        }
        foo(&nums, &mut vec![None; nums.len()], 0)
    }

    pub fn num_decodings_dp(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let nums: Vec<_> = s.chars().filter_map(|x| x.to_digit(10)).collect();
        let mut dp = vec![0; nums.len()];

        if s.len() == 1 {
            if nums[0] == 0 {
                return 0;
            } else {
                return 1;
            }
        } else {
            match (nums[0], nums[1]) {
                (0, _) => return 0,
                (1, 1..=9) | (2, 1..=6) => {
                    dp[0] = 1;
                    dp[1] = 2;
                },
                (_, 1..=9) => {
                    dp[0] = 1;
                    dp[1] = 1;
                }
                (1..=2, 0) => {
                    dp[0] = 1;
                    dp[1] = 1;
                },
                _ => {},
            }
        }

        for i in 2..nums.len() {
            dp[i] = match (nums[i - 1], nums[i]) {
                (1, 1..=9) | (2, 1..=6) => dp[i - 1] + dp[i - 2],
                (1, 0) | (2, 0) => dp[i - 2],
                (_, 1..=9) => dp[i - 1],
                _ => 0,
            }
        }
        dp.last().unwrap().clone()
    }
}

struct Solution;

fn main() {
    Solution::num_decodings_dp("2101".to_string());
    for s in ["12", "226", "910382283", "27", "2101"] {
        assert_eq!(Solution::num_decodings(s.to_string()), Solution::num_decodings_dp(s.to_string()));
    }
}
