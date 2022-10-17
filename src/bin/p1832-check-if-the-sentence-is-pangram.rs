impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut arr = [0; (b'z' - b'a' + 1) as usize];

        for c in sentence.bytes() {
            arr[(c - b'a') as usize] += 1;
        }

        arr.into_iter().all(|x| x.gt(&0))
    }
}

struct Solution;

fn main() {}
