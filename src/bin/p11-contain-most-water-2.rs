impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp::max;
        let mut area = 0;

        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_left_h = 0;
        let mut max_right_h = 0;

        while left < right {
            if height[left] > max_left_h || height[right] > max_right_h {
                let tmp = Self::compute_area(height[left], height[right], (right - left) as i32);
                if tmp > area {
                    area = tmp;
                }
                max_left_h = max(max_left_h, height[left]);
                max_right_h = max(max_right_h, height[right]);
            }
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        area
    }

    #[inline]
    fn compute_area(lh: i32, rh: i32, w: i32) -> i32 {
        use std::cmp::min;
        min(lh, rh) * w
    }
}

#[test]
fn test() {}

struct Solution {}

fn main() {
    println!("{}", Solution::max_area(vec![1, 1]));
}
