impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for s in paths.into_iter() {
            let mut xs = s.split_whitespace();
            if let Some(path) = xs.next() {
                for f in xs {
                    if let (Some(l), Some(r)) = (f.find('('), f.find(')')) {
                        let file = &f[..l];
                        let content = &f[l + 1..r];
                        let file_path = path.to_string() + "/" + file;
                        if let Some(v) = map.get_mut(content) {
                            v.push(file_path);
                        } else {
                            map.insert(content.to_owned(), vec![file_path]);
                        }
                    }
                }
            }
        }

        // map.into_values().fold(vec![], |mut xs, x| {
        //     if x.len() > 1 {
        //         xs.push(x);
        //         xs
        //     } else {
        //         xs
        //     }
        // })

        map.into_values().filter(|x| x.len() > 1).collect()
    }
}

#[test]
fn test() {}

struct Solution;

fn main() {}
