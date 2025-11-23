#![allow(dead_code)]

// 3702. Longest Subsequence With Non-Zero Bitwise XOR
// https://leetcode.com/problems/longest-subsequence-with-non-zero-bitwise-xor/
// https://leetcode.cn/problems/longest-subsequence-with-non-zero-bitwise-xor/
//
// Medium
//
// You are given an integer array nums.
//
// Return the length of the longest subsequence in nums whose bitwise XOR is non-zero. If no such subsequence exists, return 0.
//
// Example 1:
//
// Input: nums = [1,2,3]
//
// Output: 2
//
// Explanation:
//
// One longest subsequence is [2, 3]. The bitwise XOR is computed as 2 XOR 3 = 1, which is non-zero.
//
// Example 2:
//
// Input: nums = [2,3,4]
//
// Output: 3
//
// Explanation:
//
// The longest subsequence is [2, 3, 4]. The bitwise XOR is computed as 2 XOR 3 XOR 4 = 5, which is non-zero.
//
// Constraints:
//
// 1 <= nums.length <= 10^5
// 0 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().fold(0, |x, acc| acc ^ x);

        if sum == 0 {
            for &v in nums.iter() {
                let x = sum ^ v;
                if x != 0 {
                    return (nums.len() - 1) as i32;
                }
            }

            0
        } else {
            nums.len() as i32
        }
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3];
    assert_eq!(Solution::longest_subsequence(nums), 2);

    let nums = vec![2, 3, 4];
    assert_eq!(Solution::longest_subsequence(nums), 3);
}
