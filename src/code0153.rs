#![allow(dead_code)]

// 153. Find Minimum in Rotated Sorted Array
// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/
// https://leetcode.cn/problems/find-minimum-in-rotated-sorted-array/
//
// Suppose an array of length n sorted in ascending order is rotated between 1 and n times. For example, the array nums = [0,1,2,4,5,6,7] might become:
//
// [4,5,6,7,0,1,2] if it was rotated 4 times.
// [0,1,2,4,5,6,7] if it was rotated 7 times.
// Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 time results in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]].
//
// Given the sorted rotated array nums of unique elements, return the minimum element of this array.
//
// You must write an algorithm that runs in O(log n) time.
//

struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        *nums.iter().min().unwrap_or(&0)
    }
}

#[test]
fn test_find_min() {
    assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
    assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
}
