#![allow(dead_code)]

// 34. Find First and Last Position of Element in Sorted Array
// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
// https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/
//
// Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target value.
//
// If target is not found in the array, return [-1, -1].
//
// You must write an algorithm with O(log n) runtime complexity.
//
// Example 1:
//
// Input: nums = [5,7,7,8,8,10], target = 8
// Output: [3,4]
//
// Example 2:
//
// Input: nums = [5,7,7,8,8,10], target = 6
// Output: [-1,-1]
//
// Example 3:
//
// Input: nums = [], target = 0
// Output: [-1,-1]
//
// Constraints:
//
// - 0 <= nums.length <= 10^5
// - -10^9 <= nums[i] <= 10^9
// - nums is a non-decreasing array.
// - -10^9 <= target <= 10^9
//

struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.binary_search(&target).is_ok() {
            vec![
                nums.partition_point(|&i| i < target) as i32,
                nums.partition_point(|&i| i <= target) as i32 - 1,
            ]
        } else {
            vec![-1, -1]
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![5, 7, 7, 8, 8, 10], 8, vec![3, 4]),
        (vec![5, 7, 7, 8, 8, 10], 6, vec![-1, -1]),
        (vec![], 0, vec![-1, -1]),
    ];
    for (nums, target, expected) in cases {
        assert_eq!(Solution::search_range(nums, target), expected);
    }
}
