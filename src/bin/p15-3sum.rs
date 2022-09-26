impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();

        if nums.len() < 3 {
            return ret;
        }

        let mut nums = nums;
        nums.sort_unstable();

        for i in 0..(nums.len() - 2) {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            if nums[i] > 0 {
                return ret;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let x = nums[left];
                let y = nums[right];
                let z = nums[i];
                let total = x + y + z;
                if total > 0 {
                    right -= 1;
                } else if total < 0 {
                    left += 1;
                } else {
                    ret.push(vec![x, y, z]);
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }

                    left += 1;
                    right -= 1;
                }
            }
        }
        ret
    }

    pub fn three_sum_2(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        
        let mut res: Vec<Vec<i32>> = Vec::with_capacity(nums.len() / 3 + 1);
        for (i, &num) in nums.iter().enumerate() {
            if i > 0 && nums[i - 1] == num {
                continue;
            }
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while (l < r) {
                let s = nums[l] + nums[r] + num;
                match s {
                    d if d < 0 => l += 1,
                    d if d > 0 => r -= 1,
                    _ => {
                        res.push(vec![num, nums[l], nums[r]]);
                        l += 1;
                        r -= 1;
                        while (l < r && nums[l] == nums[l - 1]) {
                            l += 1;
                        }
                        while (l < r && nums[r] == nums[r + 1]) {
                            r -= 1;
                        }
                    }
                }
            }
        }
        
        res
    }
}

#[test]
fn test() {}

struct Solution {}

fn main() {}
