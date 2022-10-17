impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut arr = [false; 26];

        for c in sentence.bytes() {
            arr[(c - b'a') as usize] = true;
        }

        arr.into_iter().fold(true, |acc, x| acc && x)
    }
}

impl Solution2 {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut arr = [0; (b'z' - b'a' + 1) as usize];

        for c in sentence.bytes() {
            arr[(c - b'a') as usize] += 1;
        }

        arr.into_iter().all(|x| x.gt(&0))
    }
}

struct Solution;
struct Solution2;

fn main() {}
