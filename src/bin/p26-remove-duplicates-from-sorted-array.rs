use std::ops::Index;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // nums.dedup();
        // nums.len() as i32
        if nums.is_empty() {
            0
        } else {
            let mut last = nums[0];
            let mut cur = 1;
            for i in 1..nums.len() {
                let x = nums[i];
                if x != last {
                    last = x;
                    nums[cur] = x;
                    cur += 1;
                }
            }
            cur as i32
        }
    }
}

struct Solution;

fn main() {}
