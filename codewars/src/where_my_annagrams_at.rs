use itertools::Itertools;

fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    let this: String = word.chars().sorted_unstable().collect();
    let ws = words.iter().map(|s| {
        let s: String = s.chars().sorted_unstable().collect();
        s
    });

    let mut ret = vec![];
    for (idx, s) in ws.into_iter().enumerate() {
        if s.len() == this.len() && s.eq(&this) {
            ret.push(words[idx].to_owned());
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        do_test("abba", &["aabb", "abcd", "bbaa", "dada"], &["aabb", "bbaa"]);
        
        do_test(
            "racer",
            &["crazer", "carer", "racar", "caers", "racer"],
            &["carer", "racer"],
        );
    }

    fn do_test(word: &str, words: &[&str], exp: &[&str]) {
        let words: Vec<String> = words.iter().map(|w| w.to_string()).collect();
        let expected: Vec<String> = exp.iter().map(|w| w.to_string()).collect();
        let got = anagrams(word, &words);
        assert_eq!(
            got, expected,
            "Failed with word: \"{}\"\nwords: {:#?}",
            word, words
        );
    }
}
