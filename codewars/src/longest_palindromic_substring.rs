fn longest_palindrome(s: &str) -> String {
    // Using Manacher's with special characters
    let mut s_copy = vec!();
    s_copy.push('@');
    s_copy.push('|');
    for letter in s.chars() {
        s_copy.push(letter);
        s_copy.push('|')
    }
    s_copy.push('$');
    let str_length = s_copy.len();

    let mut start = 0;
    let mut center = 0;
    let mut max_right = 0;
    let mut max_length = 0;
    let mut max_radii = vec![0; str_length];

    for i in 1..(str_length - 1){
        if i < max_right {
            max_radii[i] = std::cmp::min(max_right - i, max_radii[2 * center - i]);
        }

        while s_copy[i + max_radii[i] + 1] == s_copy[i - max_radii[i] - 1] {
            max_radii[i] += 1;
        }

        if i + max_radii[i] > max_right {
            center = i;
            max_right = i + max_radii[i];
        }

        if max_radii[i] > max_length {
            start = (i - max_radii[i] - 1) / 2;
            max_length = max_radii[i];
        }
    }

    s[start..start + max_length].to_string()

}

fn longest_palindrome_1(s: &str) -> String {
    if s.len() <= 1 {
        return s.to_string();
    }
    let str: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut right = 0;

    // idx is usize, always >= 0!
    for idx in 0..str.len() {
        // ..aba..
        let mut l = idx;
        let mut r = idx;
        while r < str.len() {
            if str[l] == str[r] {
                if r - l > right - left {
                    left = l;
                    right = r;
                }
                // l is usize, cannot be -1!
                // l -= 1;
                if l == 0 {
                    break;
                }
                l -= 1;
                r += 1;
            } else {
                break;
            }
        }

        // ..abba..
        let mut l2 = idx;
        let mut r2 = idx + 1;
        while r2 < str.len() {
            if str[l2] == str[r2] {
                if r2 - l2 > right - left {
                    left = l2;
                    right = r2;
                }
                // l2 is usize, cannot be -1!
                // l2 -= 1;
                if l2 == 0 {
                    break;
                }
                l2 -= 1;
                r2 += 1;
            } else {
                break;
            }
        }
    }

    str[left..right + 1].iter().collect()
}

// ERROR: Cannot pass random test !!!!!!!!
fn longest_palindrome_2(s: &str) -> String {
    let len = s.len();
    if len <= 1 {
        return s.to_string();
    }
    let s: Vec<_> = s.chars().collect();
    let mut dp = vec![vec![0; s.len()]; s.len()];
    let (mut idx, mut val) = ((0, 0), 0);
    for (i, &x) in s.iter().enumerate() {
        for (j, &y) in s.iter().rev().enumerate() {
            if x == y {
                let k = if i == 0 || j == 0 {
                    1
                } else {
                    dp[i - 1][j - 1] + 1
                };
                if k > val {
                    val = k;
                    idx = (i, j);
                }
                dp[i][j] = k;
            }
        }
    }

    let end = idx.0;
    let front = end + 1 - val as usize;
    s[front..=end].into_iter().collect()
}

#[test]
fn test() {
    for (a, b) in [
        ("banana", "anana"),
        ("abba", "abba"),
        ("cbbd", "bb"),
        ("zz", "zz"),
        ("dddd", "dddd"),
        ("", ""),
        ("abcdefghijklmnopqrstuvwxyz", "a"),
        ("ttaaftffftfaafatf", "aaftffftfaa"),
        ("bbaaacc", "aaa"),
        ("m", "m"),
        ("babad", "bab"),
        ("madam", "madam"),
        ("dde", "dd"),
        ("ababbab", "babbab"),
        ("abababa", "abababa"),
    ] {
        assert_eq!(longest_palindrome_2(a), b, "test of {a} => {b}");
    }
}
