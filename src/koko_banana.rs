pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let max = *piles.iter().max().unwrap();

    let mut left = 1;
    let mut right = max;

    while left < right {
        let mid = left + (right - left) / 2;
        if can_eat_all(&piles, mid, h) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn can_eat_all(piles: &[i32], k: i32, h: i32) -> bool {
    let mut hours = 0;
    for &pile in piles {
        hours += (pile + k - 1) / k;
        if hours > h {
            return false;
        }
    }
    true
}
