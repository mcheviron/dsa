use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut counts = HashMap::new();
    for num in nums {
        counts
            .entry(num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut heap = BinaryHeap::new();
    for (&num, &count) in counts.iter() {
        heap.push(Reverse((count, num)));
        if heap.len() > k as usize {
            heap.pop();
        }
    }

    heap.into_iter().map(|Reverse((_, num))| num).collect()
}

#[test]
fn test_top_k_frequent_case7() {
    let input = vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4];
    let k = 3;
    let mut result = top_k_frequent(input.clone(), k);
    result.sort();
    let result_set: HashSet<i32> = result.into_iter().collect();
    // Check if the result contains 'k' unique elements.
    assert_eq!(result_set.len(), k as usize);
    // Check every element in the result set is part of the input.
    for &num in result_set.iter() {
        assert!(input.contains(&num));
    }
}

#[test]
fn test_top_k_frequent_case8() {
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let k = 10;
    let mut result = top_k_frequent(input.clone(), k);
    result.sort();
    let result_set: HashSet<i32> = result.into_iter().collect();
    // Check if the result contains 'k' unique elements.
    assert_eq!(result_set.len(), k as usize);
    // Check every element in the result set is part of the input.
    for &num in result_set.iter() {
        assert!(input.contains(&num));
    }
}

#[test]
fn test_top_k_frequent_case9() {
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let k = 1;
    let mut result = top_k_frequent(input.clone(), k);
    result.sort();
    assert_eq!(result, vec![10]);
}

#[test]
fn test_top_k_frequent_case10() {
    let input = vec![1, 1, 1, 1, 1, 2, 2, 2, 2, 2];
    let k = 2;
    let mut result = top_k_frequent(input.clone(), k);
    result.sort();
    let result_set: HashSet<i32> = result.into_iter().collect();
    // Check if the result contains 'k' unique elements.
    assert_eq!(result_set.len(), k as usize);
    // Check every element in the result set is part of the input.
    for &num in result_set.iter() {
        assert!(input.contains(&num));
    }
}
