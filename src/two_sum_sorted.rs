pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut i = 0;
    let mut j = numbers.len() - 1;

    while i < j {
        match numbers[i] + numbers[j] {
            sum if sum == target => return vec![i as i32 + 1, j as i32 + 1],
            sum if sum < target => i += 1,
            _ => j -= 1,
        }
    }
    vec![]
}
