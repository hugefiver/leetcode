impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::cmp::Ordering::*;

        if nums.is_empty() {
            return vec![-1, -1];
        }

        let (mut ll, mut rr) = (0, nums.len() - 1);
        let mut mid;
        while {
            mid = (ll + rr) / 2;
            ll < mid && mid < rr
        } {
            match nums[mid].cmp(&target) {
                Greater => rr = mid,
                Less => ll = mid,
                Equal => break,
            }
        }

        // if nums[mid] != target {
        //     mid = if nums[ll] == target {
        //         ll
        //     } else if nums[rr] == target {
        //         rr
        //     } else {
        //         return vec![-1, -1];
        //     };
        // }

        match rr - ll {
            0 => {
                return if nums[ll] == target {
                    vec![ll as i32, ll as i32]
                } else {
                    vec![-1, -1]
                }
            }
            1 => {
                let (l, r) = (ll as i32, rr as i32);
                let (x, y) = match (nums[ll], nums[rr]) {
                    (x, y) if x == target && y == target => (l, r),
                    (x, _) if x == target => (l, l),
                    (_, x) if x == target => (r, r),
                    _ => (-1, -1),
                };
                return vec![x, y];
            }
            _ => {}
        }

        if nums[mid] == target || nums[ll] == target || nums[rr] == target {
            let (mut lidx, mut ridx) = (
                if nums[ll] == target { ll } else { mid },
                if nums[rr] == target { rr } else { mid },
            );

            let mut mid;
            while {
                mid = (ll + lidx) / 2;
                ll < mid && mid < lidx
            } {
                match nums[mid].cmp(&target) {
                    Less => ll = mid,
                    _ => lidx = mid,
                }
            }

            while {
                mid = (rr + ridx) / 2;
                ridx < mid && mid < rr
            } {
                match nums[mid].cmp(&target) {
                    Greater => rr = mid,
                    _ => ridx = mid,
                }
            }

            vec![lidx as i32, ridx as i32]
        } else {
            vec![-1, -1]
        }
    }
}

struct Solution;

fn main() {
    for (arr, t) in [
        (vec![5, 7, 7, 8, 8, 10], 8),
        (vec![5, 7, 7, 8, 8, 10], 6),
        (vec![], 0),
        (vec![1, 1, 1, 1], 1),
        (vec![1, 2, 3, 3, 3], 3),
        (vec![1, 4], 4),
        (vec![1, 2, 4], 4),
    ] {
        assert_eq!(Solution::search_range(arr.clone(), t), search_range(arr, t));
    }
    println!("ok");

    fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        vec![
            nums.iter()
                .enumerate()
                .find(|(_, &x)| x == target)
                .map_or(-1, |(i, _)| i as i32),
            nums.iter()
                .enumerate()
                .rev()
                .find(|(_, &x)| x == target)
                .map_or(-1, |(i, _)| i as i32),
        ]
    }
}
