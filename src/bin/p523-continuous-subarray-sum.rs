impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashMap;

        let mut seen = HashMap::<i32, i32>::from([(0, -1)]);

        nums.into_iter()
            .scan(0, |acc, num| {
                *acc += num;
                Some(*acc)
            })
            .enumerate()
            .any(|(idx, sum)| {
                let idx = idx as _;
                *seen.entry(sum % k).or_insert(idx) + 1 < idx
            })
    }

    pub fn check_subarray_sum_2(nums: Vec<i32>, k: i32) -> bool {
        if nums.is_empty() {
            return false;
        } else if nums.len() == 1 {
            return nums[0] == k;
        }

        let nums = nums.into_iter().map(|x| (x % k) as u32).collect::<Vec<_>>();
        let k = k as u32;

        for i in 0..nums.len() {
            let mut sum = 0;
            for j in i..nums.len() {
                sum += nums[j];
                if sum >= k {
                    sum -= k;
                }
                if sum == 0 {
                    return true;
                }
            }
        }

        false
    }
}

struct Solution;

fn main() {}
