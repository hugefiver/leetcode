use std::iter::Sum;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let mut nums = nums;
        nums.sort_unstable();

        let mut xx = i32::MIN;
        for (idx, &x) in nums.iter().enumerate() {
            if x == xx {
                continue;
            } else {
                xx = x;
            }
            let mut yy = i32::MIN;
            for (idx, &y) in nums.iter().enumerate().skip(idx+1) {
                if y == yy {
                    continue;
                } else {
                    yy = y;
                }
                let mut zz = i32::MIN;
                for (idx, &z) in nums.iter().enumerate().skip(idx+1) {
                    if z == zz {
                        continue;
                    } else {
                        zz = z;
                    }
                    let mut ww = i32::MIN;
                    for &w in nums.iter().skip(idx+1) {
                        if w == ww {
                            continue;
                        } else {
                            ww = w;
                        }
                        let sum = [x, y, z, w].into_iter().map(i64::from).sum::<i64>();
                        if sum == target as i64 {
                            ret.push(vec![x, y, z, w]);
                            break;
                        } else if sum > target as i64 {
                            break;
                        }
                    }
                }
            }
        }

        ret
    }
}

struct Solution;

fn main() {}
