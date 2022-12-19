#![allow(dead_code)]

// 167. Two Sum II - Input array is sorted
// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
// https://leetcode.cn/problems/two-sum-ii-input-array-is-sorted/
//
// Given an array of integers numbers that is already sorted in non-decreasing order,
// find two numbers such that they add up to a specific target number.
//
// Return the indices of the two numbers (1-indexed) as an integer array answer of size 2,
// where 1 <= answer[0] < answer[1] <= numbers.length.
//
// The tests are generated such that there is exactly one solution. You may not use the same element twice.
//

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = numbers.len() - 1;
        while i < j {
            let sum = numbers[i] + numbers[j];
            match sum.cmp(&target) {
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Greater => j -= 1,
                std::cmp::Ordering::Equal => return vec![i as i32 + 1, j as i32 + 1],
            }
        }
        vec![]
    }
}

#[test]
fn test_two_sum() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
}
