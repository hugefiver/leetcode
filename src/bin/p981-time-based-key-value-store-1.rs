use std::collections::{BTreeMap, HashMap};

struct TimeMap {
    map: HashMap<String, BTreeMap<i32, String>>,
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
            val.insert(timestamp, value);
        } else {
            self.map.insert(key, [(timestamp, value)].into());
        }
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(sets) = self.map.get(&key) {
            sets.range(..=timestamp)
                .last()
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
