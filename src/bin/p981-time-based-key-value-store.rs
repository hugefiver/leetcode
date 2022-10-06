use std::{collections::{BTreeSet, HashMap}, cmp::Reverse};

struct TimeMap {
    map: HashMap<String, BTreeSet<(Reverse<i32>, String)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        if let Some(val) = self.map.get_mut(&key) {
            val.insert((Reverse(timestamp), value));
        } else {
            self.map.insert(key, [(Reverse(timestamp), value)].into());
        }
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(sets) = self.map.get(&key) {
            sets.iter()
                .skip_while(|(t, _)| t.0 > timestamp)
                .next()
                .map_or("".to_string(), |(_, val)| val.clone())
        } else {
            "".to_string()
        }
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

fn main() {}
