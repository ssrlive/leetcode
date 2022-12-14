#![allow(dead_code)]

// 747. Largest Number At Least Twice of Others
// https://leetcode.com/problems/largest-number-at-least-twice-of-others/
//
// You are given an integer array nums where the largest integer is unique.
//
// Determine whether the largest element in the array is at least twice as much as every other number in the array.
// If it is, return the index of the largest element, or return -1 otherwise.
//
// Example 1:
//
// Input: nums = [3,6,1,0]
// Output: 1
// Explanation: 6 is the largest integer.
// For every other number in the array x, 6 is at least twice as big as x.
// The index of value 6 is 1, so we return 1.
//
// Example 2:
//
// Input: nums = [1,2,3,4]
// Output: -1
// Explanation: 4 is less than twice the value of 3, so we return -1.
//
// Constraints:
//
// - 2 <= nums.length <= 50
// - 0 <= nums[i] <= 100
// - The largest element in nums is unique.
//

struct Solution;

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut max_index = 0;
        for (i, &n) in nums.iter().enumerate() {
            if n > max {
                max = n;
                max_index = i;
            }
        }
        for (i, &n) in nums.iter().enumerate() {
            if i != max_index && max < n * 2 {
                return -1;
            }
        }
        max_index as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::dominant_index(vec![3, 6, 1, 0]), 1);
    assert_eq!(Solution::dominant_index(vec![1, 2, 3, 4]), -1);
}
