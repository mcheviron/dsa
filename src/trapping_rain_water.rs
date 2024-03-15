pub fn trap(height: Vec<i32>) -> i32 {
    let mut left_max = vec![0; height.len()];
    let mut right_max = vec![0; height.len()];
    let mut max_left = 0;
    let mut max_right = 0;
    for (i, j) in (0..height.len()).zip((0..height.len()).rev()) {
        max_left = std::cmp::max(max_left, height[i]);
        left_max[i] = max_left;
        max_right = std::cmp::max(max_right, height[j]);
        right_max[j] = max_right;
    }
    let mut water = 0;
    for i in 0..height.len() {
        water += std::cmp::min(left_max[i], right_max[i]) - height[i];
    }
    water
}
