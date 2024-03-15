use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        TimeMap {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let mut result = "".to_string();
        if let Some(v) = self.map.get(&key) {
            match v.binary_search_by(|&(t, _)| t.cmp(&timestamp)) {
                Ok(i) => {
                    result = v[i].1.clone();
                }
                Err(i) if i > 0 => {
                    result = v[i - 1].1.clone();
                }
                Err(_) => {}
            }
        }
        result
    }
}

// /**
//  * Your TimeMap object will be instantiated and called as such:
//  * let obj = TimeMap::new();
//  * obj.set(key, value, timestamp);
//  * let ret_2: String = obj.get(key, timestamp);
//  */
