impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        if palindrome.len() <= 1 {
            return "".to_string();
        }

        let mut flag = false;
        let ret = if palindrome.len() % 2 == 0 {
            palindrome
                .chars()
                .map(|c| {
                    if !flag && c != 'a' {
                        flag = true;
                        'a'
                    } else {
                        c
                    }
                })
                .collect()
        } else {
            let mid = palindrome.len() / 2;
            let mut ret: Vec<_> = palindrome
                .chars()
                .take(mid)
                .map(|c| {
                    if !flag && c != 'a' {
                        flag = true;
                        'a'
                    } else {
                        c
                    }
                })
                .collect();
            if flag {
                ret.extend(palindrome.chars().skip(mid));
            } else {
                ret.extend(palindrome.chars().skip(mid).take(mid));
                ret.push('b');
            }
            ret.into_iter().collect()
        };

        if flag {
            ret
        } else {
            let len = ret.len();
            let mut ret = ret;
            unsafe { ret.as_bytes_mut()[len - 1] = b'b' };
            ret
        }
    }
}

struct Solution;

fn main() {}
