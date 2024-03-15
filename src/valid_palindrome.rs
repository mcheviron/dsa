pub fn is_palindrome(s: String) -> bool {
    let cleaned = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<Vec<char>>();

    for (left, right) in cleaned.iter().zip(cleaned.iter().rev()) {
        if left != right {
            return false;
        }
    }
    true
}
