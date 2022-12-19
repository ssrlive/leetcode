#![allow(dead_code)]

// 446. Arithmetic Slices II - Subsequence
// https://leetcode.com/problems/arithmetic-slices-ii-subsequence/
// https://leetcode.cn/problems/arithmetic-slices-ii-subsequence/
//
// Given an integer array nums, return the number of all the arithmetic subsequences of nums.
//
// A sequence of numbers is called arithmetic if it consists of at least three elements and if the difference
// between any two consecutive elements is the same.
//
// For example, [1, 3, 5, 7, 9], [7, 7, 7, 7], and [3, -1, -5, -9] are arithmetic sequences.
// For example, [1, 1, 2, 5, 7] is not an arithmetic sequence.
// A subsequence of an array is a sequence that can be formed by removing some elements (possibly none) of the array.
//
// For example, [2,5,10] is a subsequence of [1,2,1,2,4,1,5,10].
// The test cases are generated so that the answer fits in 32-bit integer.
//
// Example 1:
//
// Input: nums = [2,4,6,8,10]
// Output: 7
// Explanation: All arithmetic subsequence slices are:
// [2,4,6]
// [4,6,8]
// [6,8,10]
// [2,4,6,8]
// [4,6,8,10]
// [2,4,6,8,10]
// [2,6,10]
//
// Example 2:
//
// Input: nums = [7,7,7,7,7]
// Output: 16
// Explanation: Any subsequence of this array is arithmetic.
//
// Constraints:
//
// - 1  <= nums.length <= 1000
// - -2^31 <= nums[i] <= 2^31 - 1
//

struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut dp = vec![std::collections::HashMap::new(); nums.len()];
        let mut ans = 0;
        for i in 1..nums.len() {
            for j in 0..i {
                let diff = nums[i] as i64 - nums[j] as i64;
                if diff < std::i32::MIN as i64 || diff > std::i32::MAX as i64 {
                    continue;
                }
                let diff = diff as i32;
                let count = *dp[j].get(&diff).unwrap_or(&0);
                ans += count;
                *dp[i].entry(diff).or_insert(0) += count + 1;
            }
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]), 7);
    assert_eq!(Solution::number_of_arithmetic_slices(vec![7, 7, 7, 7, 7]), 16);
}
