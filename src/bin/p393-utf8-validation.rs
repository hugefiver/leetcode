impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut idx = 0;
        while idx < data.len() {
            if let Some(rest) = Self::utf8_rest_byte(data[idx]) {
                let ok = data[idx + 1..]
                    .iter()
                    .take(rest)
                    .all(|&x| Self::is_utf8_rest_byte(x));
                if !ok || idx + rest >= data.len() {
                    return false;
                }
                idx += rest + 1;
            } else {
                return false;
            }
        }
        true
    }

    pub fn valid_utf8_2(data: Vec<i32>) -> bool {
        let mut idx = 0;
        let mut rest = 0;
        while idx < data.len() {
            if rest == 0 {
                if let Some(r) = Self::utf8_rest_byte(data[idx]) {
                    rest = r;
                } else {
                    return false;
                }
            } else {
                if Self::is_utf8_rest_byte(data[idx]) {
                    rest -= 1;
                } else {
                    return false;
                }
            }
            idx += 1;
        }
        rest == 0
    }

    #[inline]
    fn utf8_rest_byte(x: i32) -> Option<usize> {
        if x >> 3 == 0b11110 {
            3.into()
        } else if x >> 4 == 0b1110 {
            2.into()
        } else if x >> 5 == 0b110 {
            1.into()
        } else if x >> 7 == 0 {
            0.into()
        } else {
            None
        }
    }

    #[inline]
    fn is_utf8_rest_byte(x: i32) -> bool {
        x & 0b10000000 == 0b10000000
    }
}

#[test]
fn test() {}

struct Solution {}

fn main() {}
