impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut num = 0;
        let mut last_group = 0;
        let mut last_char = '_';

        for c in s.chars() {
            let x = Self::to_num(c);
            if c == last_char {
                last_group += x;
            } else if Self::is_one_tenx(last_char, c) {
                num += x - last_group;
                last_group = 0;
            } else {
                num += last_group;
                last_group = x;
            }
            last_char = c;
        }

        num + last_group
    }

    #[inline]
    fn to_num(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }

    #[inline]
    fn is_one_tenx(c: char, d: char) -> bool {
        match (c, d) {
            ('I', 'V') | ('I', 'X') |
            ('X', 'L') | ('X', 'C') |
            ('C', 'D') | ('C', 'M') => true,
            _ => false,
        }
    }
}

#[test]
fn test() {}

struct Solution {}

fn main() {}
