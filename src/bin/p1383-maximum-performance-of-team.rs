use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let mut pairs = efficiency.iter().map(|x| *x as u64).zip(speed.iter().map(|x| *x as u64)).collect::<Vec<(u64,u64)>>();
        pairs.sort_unstable();
        let mut best:u64 = 0;
        let mut heap = BinaryHeap::new();
        let mut cur:u64 = 0;
        let k = (k as usize);
        for (e,s) in pairs.iter().rev() {
            heap.push(Reverse(s));
            cur += s;
            if heap.len() > k {
                cur -= heap.pop().unwrap().0;
            }
            if e * cur > best {
                best = e * cur;
            }
        }
        (best % 1_000_000_007) as i32
    }
}

// speed, eff
#[derive(Eq,PartialEq,PartialOrd,Ord)]
struct Engineer(i32, i32);

const ANSWER_MODULUS: u64 = 1_000_000_007;

impl Solution2 {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let mut engs: Vec<Engineer> = speed.into_iter()
            .zip(efficiency.into_iter())
            .map(|(s,e)| Engineer(s, e))
            .collect();
        // ORDER BY eff DESC, speed DESC
        engs.sort_unstable_by( |e1,e2|
            if e1.1 != e2.1 {  // faster than .then
                e1.1.cmp(&e2.1).reverse()
            } else {
                e1.0.cmp(&e2.0).reverse()
            }
        );
        let engs = engs;
        let K = k as usize;
        let mut fastest = BinaryHeap::new();
        let mut fastest_sum: u64 = 0;
        let mut best = 0_u64;
        for eng in engs {
            let Engineer(speed, eff) = eng;
            fastest.push(Reverse(eng));
            fastest_sum += speed as u64;
            if fastest.len() > K {
                let Reverse(too_slow) = fastest.pop().unwrap();
                fastest_sum -= too_slow.0 as u64;
            }
            // We now have the fastest `K` engineers with efficiency `>= eff`.
            let prod: u64 = fastest_sum * (eff as u64);
            best = best.max(prod);
        }
        (best % ANSWER_MODULUS) as i32
    }
}

impl Solution3 {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        use std::collections::{BinaryHeap, HashMap};

        let mut v: Vec<(i32, i32)> = efficiency
            .iter()
            .zip(speed.iter())
            .map(|(&e, &s)| (-e, -s))
            .collect();
        v.sort();
        //       v.sort_by_key(|x| { -x.0 });
        //        println!("{:?}", v);
        let mut heap = BinaryHeap::<i32>::new();
        let mut ans = 0i64;
        let mut sum = 0i64;
        for (e, s) in v {
            heap.push(s);
            sum += s as i64;
            if heap.len() as i32 > k {
                sum -= heap.pop().unwrap() as i64
            }
            ans = std::cmp::max(ans, sum * e as i64);
            //            println!("sum = {}, e = {}, s = {}", sum, e, s);
        }
        (ans % 1_000_000_007) as i32
    }
}

#[test]
fn test() {}

struct Solution {}
struct Solution2 {}
struct Solution3 {}

fn main() {}
