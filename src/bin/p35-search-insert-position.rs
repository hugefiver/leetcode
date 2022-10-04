impl Solution {
    pub fn search_insert(mut nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering::*;

        let len = nums.len();
        if len == 0 {
            return 0;
        }

        // nums.insert(0, i32::MIN);
        if target <= nums[0] {
            return 0;
        }
        nums.push(i32::MAX);
        let arr: Vec<_> = nums
            .windows(2)
            .map(|xs| (xs[0], xs[1]))
            .collect();
        arr.binary_search_by(|&(x, y)| {
            if x < target && target <= y {
                Equal
            } else {
                y.cmp(&target)
            }
        }).unwrap_or(len) as i32 + 1
    }
}

struct Solution;

fn main() {
    for (arr, t) in [
        (vec![1,3,5,6], 5),
        (vec![1,3,5,6], 2),
        (vec![1, 2, 3, 4], 1),
        (vec![1, 2, 3, 4], 4),
        (vec![1, 2, 3, 4], 5),
        (vec![1,3,5,6], 7),
        (vec![1, 4], 4),
        (vec![1, 2, 4], 4),
    ] {
        assert_eq!(Solution::search_insert(arr.clone(), t), foo(arr.clone(), t), "{arr:?} {t}");
    }
    println!("ok");

    fn foo(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        for (i, val) in nums.into_iter().enumerate() {
            if target <= val {
                return i as i32;
            }
        }
       len as i32
    }
}
