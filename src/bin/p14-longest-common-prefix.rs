impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.into_iter()
            .map(|s| -> Box<dyn Iterator<Item = char>> {Box::new(Vec::from(s).into_iter().map(char::from))})
            .reduce(|s: Box<dyn Iterator<Item = char>>, x: Box<dyn Iterator<Item = char>>| {
                Box::new(s.into_iter()
                    .zip(x.into_iter())
                    .take_while(|(n, m)| n == m)
                    .map(|(c, _)| c)
                    .into_iter())
            })
            .unwrap()
            .collect()
    }
}

#[test]
fn test() {}

struct Solution {}

fn main() {}
