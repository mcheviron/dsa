use std::collections::HashMap;
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for s in strs {
        let mut chars = s.chars().collect::<Vec<char>>();
        chars.sort();
        let key = chars.iter().collect::<String>();
        map.entry(key).or_default().push(s);
    }
    map.values().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sort_nested_vector(mut vec: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for inner_vec in &mut vec {
            inner_vec.sort();
        }
        vec.sort();
        vec
    }

    #[test]
    fn test_group_anagrams_with_multiple_groups() {
        let result = group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ]);
        let expected = vec![
            vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
            vec!["tan".to_string(), "nat".to_string()],
            vec!["bat".to_string()],
        ];
        assert_eq!(sort_nested_vector(result), sort_nested_vector(expected));
    }

    #[test]
    fn test_group_anagrams_with_single_group() {
        let result = group_anagrams(vec!["".to_string()]);
        let expected = vec![vec!["".to_string()]];
        assert_eq!(sort_nested_vector(result), sort_nested_vector(expected));
    }

    #[test]
    fn test_group_anagrams_with_single_word() {
        let result = group_anagrams(vec!["a".to_string()]);
        let expected = vec![vec!["a".to_string()]];
        assert_eq!(sort_nested_vector(result), sort_nested_vector(expected));
    }

    #[test]
    fn test_group_anagrams_with_additional_groups() {
        let result = group_anagrams(vec![
            "abc".to_string(),
            "cba".to_string(),
            "bca".to_string(),
            "xyz".to_string(),
            "yzx".to_string(),
            "zxy".to_string(),
        ]);
        let expected = vec![
            vec!["abc".to_string(), "cba".to_string(), "bca".to_string()],
            vec!["xyz".to_string(), "yzx".to_string(), "zxy".to_string()],
        ];
        assert_eq!(sort_nested_vector(result), sort_nested_vector(expected));
    }

    #[test]
    fn test_group_anagrams_with_multiple_groups_and_words() {
        let result = group_anagrams(vec![
            "race".to_string(),
            "care".to_string(),
            "acre".to_string(),
            "ear".to_string(),
            "era".to_string(),
        ]);
        let expected = vec![
            vec!["race".to_string(), "care".to_string(), "acre".to_string()],
            vec!["ear".to_string(), "era".to_string()],
        ];
        assert_eq!(sort_nested_vector(result), sort_nested_vector(expected));
    }
}
