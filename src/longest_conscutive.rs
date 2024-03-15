use std::cmp::max;
use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let nums_set: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());
    let mut longest_consecutive_len = 0;
    for num in &nums {
        if !nums_set.contains(&(*num - 1)) {
            let mut current_num = *num;
            let mut current_num_len = 1;
            while nums_set.contains(&(current_num + 1)) {
                current_num += 1;
                current_num_len += 1;
            }
            longest_consecutive_len = max(longest_consecutive_len, current_num_len);
        }
    }
    longest_consecutive_len
}
