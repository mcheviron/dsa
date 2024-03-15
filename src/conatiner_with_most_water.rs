use std::cmp::{max, min};
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut left = 0;
    let mut right = height.len() - 1;
    while left < right {
        max_area = max(
            max_area,
            min(height[left], height[right]) * (right - left) as i32,
        );
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    max_area
}
