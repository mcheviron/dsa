fn encode(strs: Vec<String>) -> String {
    let mut res = vec![];
    for s in strs {
        let encoded_word = format!("{}:{}", s.len(), s);
        res.push(encoded_word);
    }
    res.join("")
}

fn decode(s: String) -> Vec<String> {
    let mut res = vec![];
    let mut i = 0;
    let s_chars: Vec<char> = s.chars().collect();
    while i < s.len() {
        let mut j = i + 1;
        while j < s.len() && s_chars[j] != ':' {
            j += 1;
        }
        let word_len = s[i..j].parse::<usize>().unwrap();
        let word = &s[j + 1..j + 1 + word_len];
        res.push(word.to_string());
        i = j + 1 + word_len;
    }
    res
}
