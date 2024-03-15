pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }

    let m = matrix.len();
    let n = matrix[0].len();
    let mut left = 0;
    let mut right = m * n - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        let mid_value = match matrix.get(mid / n) {
            Some(row) => match row.get(mid % n) {
                Some(value) => value,
                None => return false,
            },
            None => return false,
        };

        match mid_value.cmp(&target) {
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid - 1,
            std::cmp::Ordering::Equal => return true,
        }
    }

    false
}
