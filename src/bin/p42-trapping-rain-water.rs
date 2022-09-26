impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut lmax, mut rmax) = (0, 0);
        let (mut l, mut r) = (0, height.len()-1);
        let mut ret = 0;
        while l < r {
            let (lh, rh) = (height[l], height[r]);
            if lh <= rh {
                if lh >= lmax {
                    lmax = lh;
                } else {
                    ret += lmax - lh;
                }
                l += 1;
            } else {
                if rh >= rmax {
                    rmax = rh;
                } else {
                    ret += rmax - rh;
                }
                r -= 1;
            }
        }

        ret
    }
}

#[test]
fn test() {}

struct Solution;

fn main() {}
