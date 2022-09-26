impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let set = Self::all_num_sets(n, k as i8);
        set.into_iter()
            .map(|s| -> i32 {
                let mut r = 0;
                for i in s {
                    r *= 10;
                    r += i as i32;
                }
                r
            })
            .collect()
    }

    fn all_num_sets(n: i32, k: i8) -> Vec<Vec<i8>> {
        let mut num_sets: Vec<Vec<i8>> = vec![];
        for i in 1..10 {
            let r = Self::foo(n, k, vec![], i);
            num_sets.extend(r.into_iter());
        }
        num_sets
    }

    fn foo(n: i32, k: i8, set: Vec<i8>, digit: i8) -> Vec<Vec<i8>> {
        if k == 0 {
            return vec![vec![digit;n as usize]];
        }

        let mut set = set;
        set.push(digit);

        if n == 1 {
            return vec![set];
        }

        let mut num_sets: Vec<Vec<i8>> = vec![];
        if digit - k >= 0 {
            let r = Self::foo(n - 1, k, set.clone(), digit - k);
            num_sets.extend(r.into_iter());
        }
        if digit + k < 10 {
            let r = Self::foo(n - 1, k, set.clone(), digit + k);
            num_sets.extend(r.into_iter());
        }

        num_sets
    }
}

#[test]
fn test() {}
struct Solution {}

fn main() {}
