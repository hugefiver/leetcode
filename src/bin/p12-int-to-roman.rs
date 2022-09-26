impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let mut s = String::new();

        if num >= 1000 {
            for _ in 0..num / 1000 {
                s.push('M');
            }
            num %= 1000;
        }

        if num >= 100 {
            let n = num / 100;
            s.push_str(&Self::build(n, 'C', 'D', 'M'));
            num %= 100;
        }

        if num >= 10 {
            let n = num / 10;
            s.push_str(&Self::build(num, 'X', 'L', 'C'));
            num = num % 10;
        }

        if num > 0 {
            s.push_str(&Self::build(num, 'I', 'V', 'X'));
        }

        s.to_string()
    }

    fn build(n: i32, one: char, five: char, uplevel: char) -> String {
        let mut s = String::new();
        if n == 9 {
            s.push(one);
            s.push(uplevel);
        } else if n >= 5 {
            s.push(five);
            for _ in 0..n - 5 {
                s.push(one);
            }
        } else if n == 4 {
            s.push(one);
            s.push(five);
        } else {
            for _ in 0..n {
                s.push(one);
            }
        }
        s
    }
}

impl Solution2 {
    pub fn int_to_roman(num: i32) -> String {
        const ONES: [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
        const TENS: [&str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
        const HUNDREDS: [&str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
        const THOUSANDS: [&str; 4] = ["", "M", "MM", "MMM"];

        let num = num as usize;
        format!(
            "{}{}{}{}",
            THOUSANDS[num / 1000],
            HUNDREDS[num / 100 % 10],
            TENS[num / 10 % 10],
            ONES[num % 10]
        )
    }
}

#[test]
fn test() {}

struct Solution {}
struct Solution2 {}
fn main() {}
