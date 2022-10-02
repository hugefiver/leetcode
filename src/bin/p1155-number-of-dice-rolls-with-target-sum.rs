impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        const MOD: i64 = 10i64.pow(9) + 7;

        if target > k * n {
            return 0;
        }
        let mut dp = vec![0i64; target as usize + 1];
        dp[0] = 1;
        (0..n).for_each(|_| {
            let mut sum = dp
                .iter()
                .rev()
                .take(k as usize)
                .fold(0, |acc, x| (acc + *x) % MOD) as i64;
            for i in (0..=target).rev() {
                if i >= k {
                    sum += dp[(i - k) as usize];
                }
                sum = (sum + MOD - dp[i as usize]) % MOD;
                dp[i as usize] = sum;
            }
        });
        dp.last().unwrap().clone() as i32
    }
}

struct Solution;

#[test]
fn test() {
    for (n, k, t, ret) in [(2, 6, 7, 6), (1, 6, 3, 1), (30, 30, 500, 222616187)] {
        assert_eq!(Solution::num_rolls_to_target(n, k, t), ret);
    }
}

fn main() {}
