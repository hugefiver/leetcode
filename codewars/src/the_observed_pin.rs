use std::collections::HashMap;

fn get_pins(observed: &str) -> Vec<String> {
    let key_map: HashMap<_, _> = [
        ('1', vec!['1', '2', '4']),
        ('2', vec!['1', '2', '3', '5']),
        ('3', vec!['2', '3', '6']),
        ('4', vec!['1', '4', '5', '7']),
        ('5', vec!['2', '4', '5', '6', '8']),
        ('6', vec!['3', '5', '6', '9']),
        ('7', vec!['4', '7', '8']),
        ('8', vec!['5', '7', '8', '9', '0']),
        ('9', vec!['6', '8', '9']),
        ('0', vec!['8', '0']),
    ]
    .into();

    let mut ret = vec![];
    for ch in observed.chars() {
        if ret.is_empty() {
            ret = key_map
                .get(&ch)
                .unwrap()
                .iter()
                .map(char::to_string)
                .collect();
        } else {
            ret = ret
                .into_iter()
                .flat_map(|s| {
                    key_map.get(&ch).unwrap().iter().map(move |c| {
                        let mut s = s.to_owned();
                        s.push(*c);
                        s
                    })
                })
                .collect();
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::get_pins;
    use itertools::Itertools;

    #[test]
    fn sample_tests() {
        assert_eq!(
            get_pins("8").iter().sorted().collect::<Vec<&String>>(),
            vec!["0", "5", "7", "8", "9"]
        );
        assert_eq!(
            get_pins("11").iter().sorted().collect::<Vec<&String>>(),
            vec!["11", "12", "14", "21", "22", "24", "41", "42", "44"]
        );
        assert_eq!(
            get_pins("369").iter().sorted().collect::<Vec<&String>>(),
            vec![
                "236", "238", "239", "256", "258", "259", "266", "268", "269", "296", "298", "299",
                "336", "338", "339", "356", "358", "359", "366", "368", "369", "396", "398", "399",
                "636", "638", "639", "656", "658", "659", "666", "668", "669", "696", "698", "699"
            ]
        );
    }
}
