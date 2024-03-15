use std::collections::HashMap;

pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }

    let mut freq_map = HashMap::new();
    for c in s1.chars() {
        *freq_map.entry(c).or_insert(0) += 1;
    }

    let mut left = 0;
    let mut count = 0;

    for right in 0..s2.len() {
        let c = s2.chars().nth(right).unwrap();
        if let Some(freq) = freq_map.get_mut(&c) {
            *freq -= 1;
            if *freq >= 0 {
                count += 1;
            }
        }

        while count == s1.len() {
            if right - left + 1 == s1.len() {
                return true;
            }

            let left_char = s2.chars().nth(left).unwrap();
            if let Some(freq) = freq_map.get_mut(&left_char) {
                *freq += 1;
                if *freq > 0 {
                    count -= 1;
                }
            }
            left += 1;
        }
    }

    false
}
