impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return String::from("");
        }

        let mut start = 0;
        let mut end = 0;
        let chs: Vec<char> = s.chars().collect();
        let mut f = vec![vec![false; chs.len()]; chs.len()];

        for j in 0..chs.len() {
            let mut i = 0;
            f[j][j] = true;
            while i <= j {
                f[i][j] = chs[j] == chs[i] && (j - i < 2 || j > 0 && f[i + 1][j - 1]);
                if f[i][j] {
                    if (j - i + 1) > (end - start) {
                        start = i;
                        end = j + 1;
                    }
                }
                i += 1;
            }
        }

        return chs[start..end].iter().collect::<String>();
    }

    // 2. DP ~ 206ms
    pub fn longest_palindrome_2(str: String) -> String {
        const NOTSET: (i64, i64) = (-1, -1);
        let s = str.as_bytes();
        let n: usize = s.len();
        if n == 0 {
            return "".to_string();
        }
        let mut dp = vec![vec![false; n]; n];
        let mut res: (i64, i64) = NOTSET;
        for j in 0..n {
            for i in (0..=j).rev() {
                let diff = (j - i) as i64;
                if s[i] == s[j] {
                    dp[i][j] = (diff < 2) || dp[i + 1][j - 1];
                }
                if dp[i][j] && (res == NOTSET || res.1 - res.0 < diff) {
                    res = (i as i64, j as i64);
                }
            }
        }
        return str[res.0 as usize..(res.1 + 1) as usize].to_string();
    }

    // 3. Expand from the middle of each element ~13ms
    pub fn longest_palindrome_3(str: String) -> String {
        let s = str.as_bytes();
        let n: i64 = s.len() as i64;
        if n == 0 {
            return str;
        }
        let mut res: &[u8] = &s[0..1];

        // check for palindromes with center in m with odd length
        for m in 0..n {
            let mut l: i64 = m as i64;
            let mut r: i64 = m as i64;
            while l >= 0 && r < n && s[l as usize] == s[r as usize] {
                l -= 1;
                r += 1;
            }
            l += 1;
            r -= 1;
            if (res.len() as i64) < (r + 1 - l) {
                res = &s[l as usize..(r + 1) as usize];
            }
        }

        // check for palindromes with center in m with even length
        for m in 0..(n - 1) {
            let mut l = m as i64;
            let mut r = (m + 1) as i64;
            while l >= 0 && r < n && s[l as usize] == s[r as usize] {
                l -= 1;
                r += 1;
            }
            l += 1;
            r -= 1;
            if (res.len() as i64) < r - l + 1 {
                res = &s[l as usize..(r + 1) as usize];
            }
        }

        String::from_utf8(res.to_vec()).unwrap()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_longest() {
        assert_eq!(
            "aba".to_string(),
            Solution::longest_palindrome("aba".to_string())
        );
        assert_eq!(
            "abba".to_string(),
            Solution::longest_palindrome("abba".to_string())
        );
        assert_eq!("".to_string(), Solution::longest_palindrome("".to_string()));
        assert_eq!(
            "a".to_string(),
            Solution::longest_palindrome("a".to_string())
        );
        assert_eq!(
            "bb".to_string(),
            Solution::longest_palindrome("cbbd".to_string())
        );
        assert_eq!(
            "aca".to_string(),
            Solution::longest_palindrome("aacabdkacaa".to_string())
        );
    }
}

fn main() {}
