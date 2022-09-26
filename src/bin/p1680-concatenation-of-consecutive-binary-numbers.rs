impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        fn bit_len(x: i32) -> u32 {
            32 - x.leading_zeros()
        }

        let m = i64::pow(10, 9) + 7;
        // let mut ret: i64 = 0;
        // for x in 1..=n {
        //     ret = (ret << bit_len(x)) + x as i64;
        //     ret %= m;
        // }
        // ret as i32
        (1..=n as i64).fold(0, |ret, x| ((ret << 64 - x.leading_zeros()) + x) % m) as i32
    }
}

#[test]
fn test() {}

struct Solution;

fn main() {
    Solution::concatenated_binary(5);
}
