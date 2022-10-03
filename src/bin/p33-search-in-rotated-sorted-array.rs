impl Solution {
    pub fn search(mut nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() <= 3 {
            return match nums.into_iter().position(|x| x == target) {
                Some(x) => x as i32,
                None => -1,
            };
        }

        let mut mid = nums
            .binary_search_by(|x| {
                if *x > nums[0] {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            })
            .unwrap_or_else(|i| i);
        if mid == 0 {
            mid += 1;
        }

        nums.rotate_left(mid);
        match nums.binary_search(&target) {
            Ok(x) => ((x + mid) % nums.len()) as i32,
            Err(_) => -1,
        }
    }

    fn search_2(nums: Vec<i32>, target: i32) -> i32 {
        let (mut start, mut end) = (0usize, nums.len());
        while start < end {
            let mid = start + (end - start) / 2;
            let mid_val = nums[mid];
            let start_val = nums[start];
            let end_val = nums[end - 1];
            if mid_val == target {
                return mid as i32;
            }
            if mid_val > start_val {
                if start_val <= target && target < mid_val {
                    end = mid;
                } else {
                    start = mid + 1;
                }
            } else {
                if mid_val < target && target <= end_val {
                    start = mid + 1;
                } else {
                    end = mid;
                }
            }
        }
        -1
    }
}

struct Solution;

fn main() {
    for (arr, n) in [
        (vec![9, 1, 2, 3, 4, 5, 6, 7, 8], 9),
        (vec![4, 5, 6, 7, 0, 1, 2], 0),
        (vec![4, 5, 6, 7, 0, 1, 2], 3),
        (vec![1], 0),
        (vec![4, 5, 6, -1, 0, 1, 2], 0),
        (vec![4, 5, 6, 7, 8, 0, 1, 2], 0),
        (vec![4, 5, 6, 7, 0, 1, 2], 4),
        (vec![4, 5, 6, 7, 0, 1, 2], 7),
    ] {
        println!("{:?} {} => {}", arr.clone(), n, Solution::search(arr, n));
    }
}
