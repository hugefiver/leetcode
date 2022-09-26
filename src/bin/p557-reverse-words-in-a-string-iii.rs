impl Solution {
    // pub fn reverse_words(s: String) -> String {
    //     s.split(' ')
    //         .map(|s| Box::new(s.chars().rev()) as Box<dyn Iterator<Item = char>>)
    //         .reduce(|x, y| {
    //             Box::new(x.chain(" ".chars()).chain(y)) as Box<dyn Iterator<Item = char>>
    //         })
    //         .map_or("".to_string(), |x| x.collect())
    // }

    pub fn reverse_words(mut s: String) -> String {
        let mut ss: Vec<_> = s.into_bytes().into_iter().map(char::from).collect();
        for xs in ss.split_mut(char::is_ascii_whitespace) {
            // xs.swap_with_slice(xs.as_ref().into_iter().rev().cloned().collect::<Vec<_>>().as_mut_slice())
            xs.reverse()
        }
        ss.into_iter().collect()
    }
}

#[test]
fn test() {}

struct Solution;

fn main() {}
