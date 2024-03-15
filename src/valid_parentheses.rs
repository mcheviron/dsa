pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];
    for c in s.chars() {
        if c == '(' || c == '[' || c == '{' {
            stack.push(c);
        } else {
            if stack.is_empty() {
                return false;
            }
            let top = stack.pop().unwrap();
            if (top == '(' && c != ')') || (top == '[' && c != ']') || (top == '{' && c != '}') {
                return false;
            }
        }
    }
    stack.is_empty()
}
