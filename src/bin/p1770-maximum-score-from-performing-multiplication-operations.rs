
struct Solution1;
impl Solution1 {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let (n, m) = (nums.len(), multipliers.len());
        let mut dp = vec![vec![0; m + 1]; m + 1];
        for left in (0..m).rev() {
            for right in (0..m - left).rev() {
                let multiplier = multipliers[left + right];
                dp[left][right] = (dp[left + 1][right] + multiplier * nums[left])
                    .max(dp[left][right + 1] + multiplier * nums[n - right - 1]);
            }
        }
        dp[0][0]
    }
}

struct Solution2;
impl Solution2 {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let (n, m) = (nums.len(), multipliers.len());
        let mut dp_prev = vec![0; m + 1];
        let mut dp_curr = dp_prev.clone();
        for left in (0..m).rev() {
            for right in (0..m - left).rev() {
                let multiplier = multipliers[left + right];
                dp_curr[right] = (dp_prev[right] + multiplier * nums[left])
                    .max(dp_curr[right + 1] + multiplier * nums[n - right - 1]);
            }
            std::mem::swap(&mut dp_prev, &mut dp_curr);
        }
        dp_prev[0]
    }
}

use std::collections::HashMap;

struct Solution3;
impl Solution3 {
    fn dp(nums: &[i32], multipliers: &[i32], left: usize, right: usize, memo: &mut HashMap<(usize, usize), i32>) -> i32 {
        let (n, m) = (nums.len(), multipliers.len());

        if left + right == m {
            0
        } else if let Some(additional_score) = memo.get(&(left, right)) {
            *additional_score
        } else {
            let rez = (nums[left] * multipliers[left + right] + Self::dp(nums, multipliers, left + 1, right, memo))
                .max(nums[n - right - 1] * multipliers[left + right] + Self::dp(nums, multipliers, left, right + 1, memo));
            memo.insert((left, right), rez);
            rez
        }
    }

    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        Self::dp(&nums, &multipliers, 0, 0, &mut HashMap::new())
    }
}

struct Solution4;
impl Solution4 {
    fn dp(nums: &[i32], multipliers: &[i32], left: usize, right: usize, memo: &mut [Vec<Option<i32>>]) -> i32 {
        let (n, m) = (nums.len(), multipliers.len());

        if left + right == m {
            0
        } else if let Some(additional_score) = memo[left][right] {
            additional_score
        } else {
            let rez = (nums[left] * multipliers[left + right] + Self::dp(nums, multipliers, left + 1, right, memo))
                .max(nums[n - right - 1] * multipliers[left + right] + Self::dp(nums, multipliers, left, right + 1, memo));
            memo[left][right] = Some(rez);
            rez
        }
    }

    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        Self::dp(&nums, &multipliers, 0, 0, &mut vec![vec![None; multipliers.len()]; multipliers.len()])
    }
}

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, muls: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = muls.len();
        let mut memo = vec![vec![std::i32::MIN; m]; n];
        Self::dp(0, 0, n, m, &nums, &muls, &mut memo)
    }
    fn dp(
        k: usize,
        i: usize,
        n: usize,
        m: usize,
        nums: &Vec<i32>,
        muls: &Vec<i32>,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if i == m {
            return 0;
        }
        if memo[k][i] != std::i32::MIN {
            return memo[k][i];
        }
        let left = Self::dp(k + 1, i + 1, n, m, nums, muls, memo) + nums[k] * muls[i];
        let right = Self::dp(k, i + 1, n, m, nums, muls, memo) + nums[n - i + k - 1] * muls[i];
        memo[k][i] = std::cmp::max(left, right);
        memo[k][i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let nums = vec![1, 2, 3];
        let muls = vec![3, 2, 1];
        assert_eq!(14, Solution::maximum_score(nums, muls));
        let nums = vec![-5, -3, -3, -2, 7, 1];
        let muls = vec![-10, -5, 3, 4, 6];
        assert_eq!(102, Solution::maximum_score(nums, muls));
    }
}

struct Solution5;
impl Solution5 {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let mut d = vec![0; multipliers.len() + 1];
        for (i, mi) in multipliers.into_iter().enumerate().rev() {
            for (j, (l, r)) in nums.iter().zip(&nums[nums.len() - 1 - i..]).enumerate() {
                d[j] = std::cmp::max(mi * l + d[j + 1], mi * r + d[j]);
            }
            d.truncate(i + 1);
        }
        d[0]
    }
}

fn main() {}
