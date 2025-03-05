#![allow(dead_code)]

// 209. Minimum Size Subarray Sum
// https://leetcode.com/problems/minimum-size-subarray-sum/
// https://leetcode.cn/problems/minimum-size-subarray-sum/
//
// Given an array of n positive integers and a positive integer s, find the
// minimal length of a contiguous subarray of which the sum ≥ s. If there isn't
// one, return 0 instead.
//
// Example:
//
// Input: s = 7, nums = [2,3,1,2,4,3]
// Output: 2
// Explanation: the subarray [4,3] has the minimal length under the problem
// constraint.
//
// Follow up:
//
// If you have figured out the O(n) solution, try coding another solution of
// which the time complexity is O(n log n).
//

struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut min_len = nums.len() + 1;
        let mut left = 0;
        for (right, &num) in nums.iter().enumerate() {
            sum += num;
            while sum >= target {
                min_len = std::cmp::min(min_len, right - left + 1);
                sum -= nums[left];
                left += 1;
            }
        }
        if min_len == nums.len() + 1 { 0 } else { min_len as i32 }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
    assert_eq!(Solution::min_sub_array_len(11, vec![1, 2, 3, 4, 5]), 3);
}
