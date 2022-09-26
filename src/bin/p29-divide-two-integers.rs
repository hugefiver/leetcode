impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        assert!(divisor != 0);
        let negative = (divisor > 0) ^ (dividend > 0);
        let (mut x, y) = ((dividend as i64).abs(), (divisor as i64).abs());
        let mut ret: i64 = 0;
        for shift in (0..=(y.leading_zeros() - 32)).rev() {
            let z = y << shift;
            if x >= z {
                ret += 1 << shift;
                x -= z;
            }
        }

        if negative {
            (-ret).max(i32::MIN as i64) as i32
        } else {
            (ret).min(i32::MAX as i64) as i32
        }
    }
}

#[test]
fn test() {}

struct Solution;

fn main() {}
