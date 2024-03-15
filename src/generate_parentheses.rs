pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = vec![];
    let mut stack = vec![];

    fn backtrack(
        n: i32,
        opening_count: i32,
        closing_count: i32,
        res: &mut Vec<String>,
        stack: &mut Vec<&str>,
    ) {
        if opening_count == n && closing_count == n {
            res.push(stack.join(""))
        }
        if opening_count < n {
            stack.push("(");
            backtrack(n, opening_count + 1, closing_count, res, stack);
            stack.pop();
        }
        if closing_count < opening_count {
            stack.push(")");
            backtrack(n, opening_count, closing_count + 1, res, stack);
            stack.pop();
        }
    }
    backtrack(n, 0, 0, &mut res, &mut stack);
    res
}
