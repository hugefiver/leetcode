impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut es = vec![false; nums.len()];
        let mut dup = None;
        nums.into_iter().for_each(|x| {
            let idx = x as usize - 1;
            if dup.is_none() && es[idx] {
                dup = Some(x);
            } else {
                es[idx] = true;
            }
        });
        vec![dup.unwrap(), es.into_iter().position(|x| !x).unwrap() as i32 + 1]
    }
}

struct Solution;

fn main() {}
