impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        Self::travel(&digits)
    }

    fn travel(digits: &str) -> Vec<String> {
        if digits.len() == 0 { 
            return vec![];
        }
        
        let dig = digits.chars().next().unwrap();
        let chs = Self::letter_from_digit(dig);

        if digits.len() == 1 {
            chs.into_iter().map(|c| c.to_string()).collect()
        } else {
            let mut ret = vec![];
            for c in chs.into_iter() {
                for s in Self::travel(&digits[1..]).into_iter() {
                    ret.push(c.to_string() + &s);
                }
            }
            ret
        }
    }

    fn letter_from_digit(digit: char) -> Vec<char> {
        match digit {
            '2' => vec!['a', 'b', 'c'],
            '3' => vec!['d', 'e', 'f'],
            '4' => vec!['g', 'h', 'i'],
            '5' => vec!['j', 'k', 'l'],
            '6' => vec!['m', 'n', 'o'],
            '7' => vec!['p', 'q', 'r', 's'],
            '8' => vec!['t', 'u', 'v'],
            '9' => vec!['w', 'x', 'y', 'z'],
            _ => panic!(),
        }
    }
}

#[test]
fn test() {}

struct Solution {}

fn main() {}
