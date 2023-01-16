#![allow(dead_code)]

// 2401. Longest Nice Subarray
// https://leetcode.com/problems/longest-nice-subarray/
// https://leetcode.cn/problems/longest-nice-subarray/
//
// You are given an array nums consisting of positive integers.
//
// We call a subarray of nums nice if the bitwise AND of every pair of elements that are in different positions in the subarray is equal to 0.
//
// Return the length of the longest nice subarray.
//
// A subarray is a contiguous part of an array.
//
// Note that subarrays of length 1 are always considered nice.
//
// Example 1:
//
// Input: nums = [1,3,8,48,10]
// Output: 3
// Explanation: The longest nice subarray is [3,8,48]. This subarray satisfies the conditions:
// - 3 AND 8 = 0.
// - 3 AND 48 = 0.
// - 8 AND 48 = 0.
// It can be proven that no longer nice subarray can be obtained, so we return 3.
//
// Example 2:
//
// Input: nums = [3,1,5,11,13]
// Output: 1
// Explanation: The length of the longest nice subarray is 1. Any subarray of length 1 can be chosen.
//
// Constraints:
//
// - 1 <= nums.length <= 10^5
// - 1 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        let mut b_xor = 0;
        let mut l = 0;
        let mut ans = 1;
        for (i, v) in nums.iter().enumerate() {
            while b_xor & v > 0 {
                b_xor ^= nums[l];
                l += 1;
            }
            ans = max(ans, i - l + 1);
            b_xor |= v;
        }
        ans as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::longest_nice_subarray(vec![1, 3, 8, 48, 10]), 3);
    assert_eq!(Solution::longest_nice_subarray(vec![3, 1, 5, 11, 13]), 1);
}
