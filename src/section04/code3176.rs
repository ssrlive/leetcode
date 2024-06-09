#![allow(dead_code)]

// 3176. Find the Maximum Length of a Good Subsequence I
// https://leetcode.com/problems/find-the-maximum-length-of-a-good-subsequence-i/
// https://leetcode.cn/problems/find-the-maximum-length-of-a-good-subsequence-i/
//
// Medium
//
// You are given an integer array nums and a non-negative integer k. A sequence of integers seq is called good
// if there are at most k indices i in the range [0, seq.length - 2] such that seq[i] != seq[i + 1].
//
// Return the maximum possible length of a good subsequence of nums.
//
// Example 1:
//
// Input: nums = [1,2,1,1,3], k = 2
//
// Output: 4
//
// Explanation:
//
// The maximum length subsequence is [1,2,1,1,3].
//
// Example 2:
//
// Input: nums = [1,2,3,4,5,1], k = 0
//
// Output: 2
//
// Explanation:
//
// The maximum length subsequence is [1,2,3,4,5,1].
//
// Constraints:
//
//     1 <= nums.length <= 500
//     1 <= nums[i] <= 10^9
//     0 <= k <= min(nums.length, 25)
//

struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![1; (k + 1) as usize]; n];
        let mut max_length = 1;
        for i in 1..n {
            for j in 0..=k {
                for m in 0..i {
                    if nums[m] == nums[i] {
                        dp[i][j as usize] = dp[i][j as usize].max(dp[m][j as usize] + 1);
                    } else if j > 0 {
                        dp[i][j as usize] = dp[i][j as usize].max(dp[m][j as usize - 1] + 1);
                    }
                }
                max_length = max_length.max(dp[i][j as usize]);
            }
        }
        max_length
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 1, 1, 3];
    let k = 2;
    let output = 4;
    assert_eq!(Solution::maximum_length(nums, k), output);

    let nums = vec![1, 2, 3, 4, 5, 1];
    let k = 0;
    let output = 2;
    assert_eq!(Solution::maximum_length(nums, k), output);

    let nums = vec![59, 59, 60, 60, 60, 59];
    let k = 2;
    let output = 6;
    assert_eq!(Solution::maximum_length(nums, k), output);

    let nums = vec![7, 13, 13, 29, 41];
    let k = 4;
    let output = 5;
    assert_eq!(Solution::maximum_length(nums, k), output);
}
