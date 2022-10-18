use std::collections::HashSet;

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        Self::foo(&candidates, target).into_iter().collect()
    }

    fn foo(arr: &[i32], target: i32) -> HashSet<Vec<i32>> {
        let mut ret = HashSet::new();
        for &i in arr.iter() {
            if i == target {
                ret.insert(vec![i]);
                break;
            } else if i < target {
                let rr = Self::foo(arr, target - i);
                if !rr.is_empty() {
                    rr.into_iter().for_each(|mut x| {
                        x.push(i);
                        x.sort_unstable();
                        ret.insert(x);
                    });
                }
            } else {
                break;
            }
        }
        ret
    }
}

struct Solution;

fn main() {
    let arr = vec![2, 3, 6, 7];
    let target = 7;
    let result = Solution::combination_sum(arr, target);
    println!("{:?}", result);
}
