use std::cmp::Reverse;

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable_by_key(|x| Reverse(*x));
        nums.windows(3)
            .find(|xs| xs[1] + xs[2] > xs[0])
            .map_or(0, |xs| xs.into_iter().sum())
    }
}

struct Solution;

fn main() {}
