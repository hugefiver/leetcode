impl Solution {
    pub fn number_of_weak_characters1(properties: Vec<Vec<i32>>) -> i32 {
        // let mut chars: Vec<_> = properties.into_iter().map(|x| (x[0], x[1])).collect();
        // chars.sort_by_key(|&(att, _)| att);

        // let mut count = 0;
        // for (i, (att1, def1)) in chars[..chars.len() - 1].iter().enumerate() {
        //     if chars[i + 1..]
        //         .iter()
        //         .rev()
        //         .any(|(att2, def2)| att2 > att1 && def2 > def1)
        //     {
        //         count += 1;
        //     }
        // }
        // count
        let mut chars: Vec<_> = properties;
        chars.sort_by_key(|x| (x[0], x[1]));

        let mut count = 0;
        let mut max_idx = chars.len() - 1;
        let mut search_idx = max_idx;
        for (i, (att, def)) in chars[..chars.len() - 1].iter().map(|x| (x[0], x[1])).enumerate().rev() {
            for idx in (i+1..search_idx).rev() {
                if chars[idx][0] <= att {
                    search_idx = idx + 1;
                    break;
                }
                if chars[max_idx][1] < chars[idx][1] {
                    max_idx = idx;
                }
            }
            search_idx = std::cmp::min(search_idx, max_idx);
            
            if att < chars[max_idx][0] && def < chars[max_idx][1] {
                count += 1;
            }
        }
        count
    }

    pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        let mut max_att = properties.iter().map(|prop| prop[0]).max().unwrap();
        let mut max_def = vec![0; max_att as usize + 2];
        
        for (att, def) in properties.iter().map(|prop| (prop[0] as usize, prop[1])) {
            max_def[att] = max_def[att].max(def);
        }
        
        for i in (0..max_att as usize).rev() {
            max_def[i] = max_def[i].max(max_def[i + 1]);
        }
        
        properties
            .iter()
            .filter(|prop| prop[1] < max_def[prop[0] as usize + 1])
            .count() as _
    }

    pub fn number_of_weak_characters_fold(mut properties: Vec<Vec<i32>>) -> i32 {
        properties.sort_unstable_by_key(|pair| (-pair[0], pair[1]));
        properties
            .iter()
            .fold((0, 0), |(res, max_def), pair| {
                (res + (pair[1] < max_def) as i32, (max_def.max(pair[1])))
            })
            .0
    }

    pub fn number_of_weak_characters2(properties: Vec<Vec<i32>>) -> i32 {
        let mut max_defs = [0; 100_002];

        properties.iter().for_each(|pair| {
            if let &[attack, deffense] = pair.as_slice() {
                let max_def_per_attack = &mut max_defs[attack as usize];
                *max_def_per_attack = deffense.max(*max_def_per_attack);
            }
        });

        let mut prev_max_def = i32::MIN;
        max_defs.iter_mut().rev().for_each(|def| {
            *def = prev_max_def.max(*def);
            prev_max_def = *def;
        });

        properties
            .iter()
            .filter(|pair| pair[1] < max_defs[pair[0] as usize + 1])
            .count() as _
    }
}

#[test]
fn test() {}

struct Solution {}

fn main() {
    Solution::number_of_weak_characters([[7,9],[10,7],[6,9],[10,4],[7,5],[7,10]]
        .map(|x| x.to_vec())
        .to_vec());
}
