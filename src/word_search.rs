use std::collections::HashSet;

fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let rows = board.len();
    let cols = board[0].len();
    let mut path: HashSet<(usize, usize)> = HashSet::new();

    fn dfs(
        r: usize,
        c: usize,
        i: usize,
        rows: usize,
        cols: usize,
        word: &str,
        board: &Vec<Vec<char>>,
        path: &mut HashSet<(usize, usize)>,
    ) -> bool {
        if i == word.len() {
            return true;
        }

        if r >= rows
            || c >= cols
            || word.chars().nth(i) != Some(board[r][c])
            || path.contains(&(r, c))
        {
            return false;
        }

        path.insert((r, c));
        let res = dfs(r + 1, c, i + 1, rows, cols, word, board, path)
            || dfs(r.wrapping_sub(1), c, i + 1, rows, cols, word, board, path)
            || dfs(r, c + 1, i + 1, rows, cols, word, board, path)
            || dfs(
                r,
                c.wrapping_sub(1) - 1,
                i + 1,
                rows,
                cols,
                word,
                board,
                path,
            );
        path.remove(&(r, c));

        res
    }

    for r in 0..rows {
        for c in 0..cols {
            if dfs(r, c, 0, rows, cols, &word, &board, &mut path) {
                return true;
            }
        }
    }

    false
}
