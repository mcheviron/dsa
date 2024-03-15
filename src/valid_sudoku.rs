use std::collections::HashSet;
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut row_set = vec![HashSet::new(); 9];
    let mut col_set = vec![HashSet::new(); 9];
    let mut box_set = vec![HashSet::new(); 9];

    for i in 0..9 {
        for j in 0..9 {
            let num = board[i][j];
            if num != '.' {
                let num = num.to_digit(10).unwrap() as usize;
                if row_set[i].contains(&num)
                    || col_set[j].contains(&num)
                    || box_set[i / 3 * 3 + j / 3].contains(&num)
                {
                    return false;
                }
                row_set[i].insert(num);
                col_set[j].insert(num);
                box_set[i / 3 * 3 + j / 3].insert(num);
            }
        }
    }
    true
}
