#![allow(dead_code)]

// 665. Non-decreasing Array
// https://leetcode.com/problems/non-decreasing-array/
// https://leetcode.cn/problems/non-decreasing-array/
//
// Given an array nums with n integers, your task is to check if it could become non-decreasing by modifying at most one element.
//
// We define an array is non-decreasing if nums[i] <= nums[i + 1] holds for every i (0-based) such that (0 <= i <= n - 2).
//
// Example 1:
//
// Input: nums = [4,2,3]
// Output: true
// Explanation: You could modify the first 4 to 1 to get a non-decreasing array.
//
// Example 2:
//
// Input: nums = [4,2,1]
// Output: false
// Explanation: You cannot get a non-decreasing array by modifying at most one element.
//
// Constraints:
//
// - n == nums.length
// - 1 <= n <= 10^4
// - -10^5 <= nums[i] <= 10^5
//
// Follow up: Can you solve it in O(n) time complexity?

struct Solution;

impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut count = 0;
        let mut prev = std::i32::MIN;
        for i in 0..nums.len() {
            if nums[i] < prev {
                count += 1;
                if count > 1 {
                    return false;
                }
                if i > 1 && nums[i - 2] > nums[i] {
                    continue;
                }
            }
            prev = nums[i];
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(Solution::check_possibility(vec![4, 2, 3]), true);
    assert_eq!(Solution::check_possibility(vec![4, 2, 1]), false);
    assert_eq!(Solution::check_possibility(vec![3, 4, 2, 3]), false);
}
