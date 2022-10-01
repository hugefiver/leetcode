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
                if nums[idx] == 0 { 0 } else { 1 }
            } else {
                if dp[idx].is_some() {
                    dp[idx].unwrap()
                } else {
                    let ret = match (nums[idx], nums[idx + 1]) {
                        (1, _) => foo(nums, dp, idx + 1) + foo(nums, dp, idx + 2),
                        (2, 0..=6) => foo(nums, dp, idx+2) + foo(nums, dp, idx+1),
                        (1..=9, _) => foo(nums, dp, idx+1),
                        _ => 0,
                    };
                    dp[idx].replace(ret);
                    ret
                }   
            }
        }
        foo(&nums, &mut vec![None; nums.len()], 0)
    }
}

struct Solution;

fn main() {
    let ret = Solution::num_decodings("12".to_string());
    println!("{ret}");
}
