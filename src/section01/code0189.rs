#![allow(dead_code)]

// 189. Rotate Array
// https://leetcode.com/problems/rotate-array/
// https://leetcode.cn/problems/rotate-array/
//
// Given an array, rotate the array to the right by k steps, where k is non-negative.
//

struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        nums.rotate_right(k)
    }
}

#[test]
fn test_rotate() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut nums, 3);
    assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    let mut nums = vec![-1, -100, 3, 99];
    Solution::rotate(&mut nums, 2);
    assert_eq!(nums, vec![3, 99, -1, -100]);
}
