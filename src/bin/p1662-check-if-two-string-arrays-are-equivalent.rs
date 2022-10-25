impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        if word1.iter().map(|s| s.len()).sum::<usize>()
            == word2.iter().map(String::len).sum::<usize>()
        {
            word1
                .iter()
                .flat_map(|s| s.chars())
                .zip(word2.iter().flat_map(|s| s.chars()))
                .all(|(x, y)| x == y)
        } else {
            false
        }
    }
}

struct Solution;

fn main() {}
