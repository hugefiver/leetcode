impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map_or(-1, |x| x as i32)
    }
}

#[test]
fn test() {}

struct Solution;

fn main() {}
