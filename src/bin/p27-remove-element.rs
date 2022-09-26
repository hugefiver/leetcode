#![feature(drain_filter)]
#![feature(iter_partition_in_place)]


impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32
    }

    pub fn remove_element_1(nums: &mut Vec<i32>, val: i32) -> i32 {
        (nums.len() - nums.drain_filter(|x| *x == val).count()) as i32
    }

    pub fn remove_element_2(nums: &mut Vec<i32>, val: i32) -> i32 {
        // nums.iter_mut().partition_in_place(|&x| x != val) as i32
        
        let (ret, _) = nums.to_owned().into_iter().partition(|x| *x != val);
        *nums = ret;
        nums.len() as i32
    }

    pub fn remove_element_3(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            0
        } else {
            let mut i = 1;
            for j in 1..nums.len() {
                if nums[j] != val {
                    nums[i] = nums[j];
                    i += 1;
                }
            }
            i as i32
        }
    }
}

struct Solution;

fn main() {}
