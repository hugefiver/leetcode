impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
        if tokens.len() == 0 {
            return 0;
        }
        let mut tokens = tokens;
        let mut power = power;
        let mut score = 0;

        tokens.sort_unstable();

        let mut i = 0;
        let mut j = tokens.len() - 1;
        while i < j {
            let token = tokens[i];
            let earn = tokens[j];

            if power >= token {
                power -= token;
                score += 1;
                i += 1;
            } else if score > 0 {
                power += earn;
                score -= 1;
                j -= 1;
            } else {
                break;
            }
        }

        if tokens[i] <= power {
            score += 1;
        }
        score
    }
}

#[test]
fn test() {}

struct Solution {}

fn main() {}
