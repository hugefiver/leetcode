impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if buildings.is_empty() {
            return vec![];
        }

        use std::collections::BTreeMap;
        let mut points = buildings
            .iter()
            .flat_map(|x| [(x[0], x[2]), (x[1], 0)])
            .fold(BTreeMap::<i32, i32>::new(), |mut acc, p| {
                if let Some(last) = acc.get_mut(&p.0) {
                    *last = (*last).max(p.1);
                } else {
                    acc.insert(p.0, p.1);
                }
                acc
            });
        for line in buildings.into_iter().map(|x| (x[0], x[1], x[2])) {
            for (_, val) in points.range_mut(line.0 .. line.1) {
                if *val < line.2 {
                    *val = line.2;
                }
            }
        }
        
        let mut ret: Vec<_> = points.into_iter().map(|(k, v)| vec![k, v]).collect();
        ret.dedup_by_key(|x| x[1]);
        ret
    }
}

struct Solution;

fn main() {}
