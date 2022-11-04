impl Solution {
    pub fn reverse_vowels(mut s: String) -> String {
        let arr = unsafe { s.as_bytes_mut() };
        let is_vowels = |x: &u8| "aeiouAEIOU".contains(*x as char);

        let (i, j) = (
            arr.iter().position(is_vowels),
            arr.iter().rev().position(is_vowels),
        );
        if i.is_none() || j.is_none() {
            s
        } else {
            let (mut i, mut j) = (i.unwrap(), arr.len() - j.unwrap() - 1);
            while i < j {
                match (is_vowels(&arr[i]), is_vowels(&arr[j])) {
                    (true, true) => {
                        arr.swap(i, j);
                        i += 1;
                        j -= 1;
                    }
                    (true, false) => j -= 1,
                    (false, true) => i += 1,
                    (false, false) => {
                        i += 1;
                        j -= 1;
                    }
                }
            }
            s
        }
    }
}

struct Solution;

fn main() {}
