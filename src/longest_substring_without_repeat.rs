use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max_len = 0;
    let mut start = 0;
    let mut map = HashMap::new();
    for (i, c) in s.chars().enumerate() {
        if let Some(&index) = map.get(&c) {
            if index >= start {
                max_len = max_len.max(i - start);
                start = index + 1;
            }
        }
        map.insert(c, i);
    }
    max_len.max(s.len() - start) as i32
}
