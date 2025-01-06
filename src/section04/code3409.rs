#![allow(dead_code)]

// 3409. Longest Subsequence With Decreasing Adjacent Difference
// https://leetcode.com/problems/longest-subsequence-with-decreasing-adjacent-difference/
// https://leetcode.cn/problems/longest-subsequence-with-decreasing-adjacent-difference/
//
// Medium
//
// You are given an array of integers nums.
//
// Your task is to find the length of the longest subsequence seq of nums, such that the absolute differences
// between consecutive elements form a non-increasing sequence of integers. In other words, for a subsequence
// seq0, seq1, seq2, ..., seqm of nums, |seq1 - seq0| >= |seq2 - seq1| >= ... >= |seqm - seqm - 1|.
//
// Return the length of such a subsequence.
//
// A subsequence is an non-empty array that can be derived from another array by deleting some
// or no elements without changing the order of the remaining elements.
//
// Example 1:
//
// Input: nums = [16,6,3]
//
// Output: 3
//
// Explanation:
//
// The longest subsequence is [16, 6, 3] with the absolute adjacent differences [10, 3].
//
// Example 2:
//
// Input: nums = [6,5,3,4,2,1]
//
// Output: 4
//
// Explanation:
//
// The longest subsequence is [6, 4, 2, 1] with the absolute adjacent differences [2, 2, 1].
//
// Example 3:
//
// Input: nums = [10,20,10,19,10,20]
//
// Output: 5
//
// Explanation:
//
// The longest subsequence is [10, 20, 10, 19, 10] with the absolute adjacent differences [10, 10, 9, 9].
//
// Constraints:
//
//     2 <= nums.length <= 10^4
//     1 <= nums[i] <= 300
//

struct Solution;

impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; 301]; 301];
        let mut arr = vec![false; 301];
        let mut ans = 0;

        for &num in &nums {
            for i in (0..=300).rev() {
                if arr[i] {
                    let diff = (num - i as i32).unsigned_abs() as usize;
                    dp[num as usize][diff] = dp[num as usize][diff].max(dp[i][diff] + 1);
                }
            }
            for i in (0..300).rev() {
                dp[num as usize][i] = dp[num as usize][i].max(dp[num as usize][i + 1]);
            }
            ans = ans.max(dp[num as usize][0]);
            arr[num as usize] = true;
        }

        ans + 1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::longest_subsequence(vec![16, 6, 3]), 3);
    assert_eq!(Solution::longest_subsequence(vec![6, 5, 3, 4, 2, 1]), 4);
    assert_eq!(Solution::longest_subsequence(vec![10, 20, 10, 19, 10, 20]), 5);
}
