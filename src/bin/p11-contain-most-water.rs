impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut area = 0;

        let mut max_left_h = 0;
        for left in 0..height.len() {
            if height[left] <= max_left_h {
                continue;
            } else {
                max_left_h = height[left];
            }

            let mut max_right_h = 0;
            for right in (left+1..height.len()).rev() {
                if height[right] <= max_right_h {
                    continue;
                } else {
                    max_right_h = height[right];
                    let tmp =
                        Self::compute_area(height[left], height[right], (right - left) as i32);
                    if tmp > area {
                        area = tmp;
                    }
                }
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
fn test() {
}

struct Solution {}

fn main() {
    println!("{}", Solution::max_area(vec![1,1]));
}
