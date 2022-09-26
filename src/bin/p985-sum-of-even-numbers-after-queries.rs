impl Solution {
    pub fn sum_even_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;
        let mut sum_even: i32 = nums.iter().filter(|&x| x % 2 == 0).sum();
        let mut ret = vec![];

        queries.into_iter().for_each(|x| {
            let (i, val) = (x[1] as usize, x[0]);
            let x = &mut nums[i];
            match (*x % 2 == 0, val % 2 == 0) {
                (true, true) => sum_even += val,
                (true, false) => sum_even -= *x,
                (false, false) => sum_even += *x + val,
                _ => {}
            };
            *x += val;
            ret.push(sum_even);
        });

        ret
    }
}

#[test]
fn test() {}

struct Solution;

fn main() {}
