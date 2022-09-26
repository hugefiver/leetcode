impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for ch in s.chars() {
            match ch {
                '(' | '{' | '[' => stack.push(ch),
                ')' | '}' | ']' => {
                    if let Some(last) = stack.last() {
                        match (last, ch) {
                            ('(', ')') | ('{', '}') | ('[', ']') => {
                                stack.pop();
                            },
                            _ => return false,
                        }
                    } else {
                        return false;
                    }
                },
                _ => continue,
            }
        }
        stack.is_empty()
    }
}

#[test]
fn test() {}

struct Solution;

fn main() {}
