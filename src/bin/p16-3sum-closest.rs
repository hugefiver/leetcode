impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() <= 3 {
            return nums.into_iter().sum();
        }

        let mut nums = nums;
        nums.sort();

        let mut closest = nums[0] + nums[1] + nums[2];
        let mut min_diff = (closest - target).abs();
        
        let mut a = 0;
		
        while a < nums.len() - 2 {
		
            let mut b = nums.len() - 1;
            let mut c = a + 1;
			
            while c < b {
                let sum = nums[a] + nums[b] + nums[c];
                let diff = (sum - target).abs();

                if diff < min_diff {
                    closest = sum;
                    min_diff = diff;
                }

                if sum > target {
                    b -= 1;
                } else if sum < target {
                    c += 1;
                } else {
                    return closest
                }
                
            }
                
            a += 1;
        }

        closest
    }

    pub fn three_sum_closest_2(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        
        let mut closest: Option<i32> = None; 
        
        for (start, &a) in nums.iter().enumerate() {
            let mut i = start + 1;
            let mut j = nums.len() - 1;   
            
            while i < j {
                let sum = nums[i] + nums[j] + a;
                
                if sum == target {
                    return target;
                } else if sum < target {
                    i += 1;
                } else {
                    j -= 1;
                }
                
                if let Some(val) = closest {
                    if (target - sum).abs() < (target - val).abs() {
                        closest = Some(sum);
                    }
                } else {
                    closest = Some(sum);
                }
            }
        }
        closest.unwrap()
    }
}

struct Solution {}

fn main() {}
