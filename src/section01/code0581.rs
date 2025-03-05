#![allow(dead_code)]

// 581. Shortest Unsorted Continuous Subarray
// https://leetcode.com/problems/shortest-unsorted-continuous-subarray/
// https://leetcode.cn/problems/shortest-unsorted-continuous-subarray/
//
// Given an integer array nums, you need to find one continuous subarray that if you only sort this subarray
// in ascending order, then the whole array will be sorted in ascending order.
//
// Return the shortest such subarray and output its length.
//
// Example 1:
//
// Input: nums = [2,6,4,8,10,9,15]
// Output: 5
// Explanation: You need to sort [6, 4, 8, 10, 9] in ascending order to make the whole array sorted in ascending order.
//
// Example 2:
//
// Input: nums = [1,2,3,4]
// Output: 0
//
// Example 3:
//
// Input: nums = [1]
// Output: 0
//
// Constraints:
//
// - 1 <= nums.length <= 10^4
// - -10^5 <= nums[i] <= 10^5
//
// Follow up: Can you solve it in O(n) time complexity?
//

struct Solution;

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut sorted = nums.clone();
        sorted.sort();

        let mut start = 0;
        let mut end = 0;

        for i in 0..nums.len() {
            if nums[i] != sorted[i] {
                start = i;
                break;
            }
        }

        for i in (0..nums.len()).rev() {
            if nums[i] != sorted[i] {
                end = i;
                break;
            }
        }

        if start == end { 0 } else { (end - start + 1) as i32 }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]), 5);
    assert_eq!(Solution::find_unsorted_subarray(vec![1, 2, 3, 4]), 0);
    assert_eq!(Solution::find_unsorted_subarray(vec![1]), 0);
}
