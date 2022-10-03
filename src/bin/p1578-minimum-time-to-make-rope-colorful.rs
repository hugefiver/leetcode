impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut arr: Vec<_> = colors.chars().zip(needed_time.into_iter()).collect();
        let mut ret = 0;
        (0..arr.len() - 1).zip(1usize..).for_each(|(i, j)| {
            let (x, xx) = arr[i];
            let (y, yy) = arr[j];
            if x == y {
                if xx <= yy {
                    ret += xx;
                } else {
                    ret += yy;
                    arr.swap(i, j);
                }
            }
        });
        ret
    }
}

struct Solution;

fn main() {}
