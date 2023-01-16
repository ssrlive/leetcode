#![allow(dead_code)]

// 712. Minimum ASCII Delete Sum for Two Strings
// https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/
// https://leetcode.cn/problems/minimum-ascii-delete-sum-for-two-strings/
//
// Given two strings s1 and s2, return the lowest ASCII sum of deleted characters to make two strings equal.
//
// Example 1:
//
// Input: s1 = "sea", s2 = "eat"
// Output: 231
// Explanation: Deleting "s" from "sea" adds the ASCII value of "s" (115) to the sum.
// Deleting "t" from "eat" adds 116 to the sum.
// At the end, both strings are equal, and 115 + 116 = 231 is the minimum sum possible to achieve this.
//
// Example 2:
//
// Input: s1 = "delete", s2 = "leet"
// Output: 403
// Explanation: Deleting "dee" from "delete" to turn the string into "let",
// adds 100[d] + 101[e] + 101[e] to the sum.
// Deleting "e" from "leet" adds 101[e] to the sum.
// At the end, both strings are equal to "let", and the answer is 100+101+101+101 = 403.
// If instead we turned both strings into "lee" or "eet", we would get answers of 433 or 417, which are higher.
//
// Constraints:
//
// - 1 <= s1.length, s2.length <= 1000
// - s1 and s2 consist of lowercase English letters.
//

struct Solution;

use std::cmp::min;

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let s1v: Vec<i32> = s1.chars().map(|x| x as i32).collect();
        let s2v: Vec<i32> = s2.chars().map(|x| x as i32).collect();
        let m = s1.len();
        let n = s2.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];
        for i in (0..=m - 1).rev() {
            dp[i][n] = dp[i + 1][n] + s1v[i];
        }
        for i in (0..=n - 1).rev() {
            dp[m][i] = dp[m][i + 1] + s2v[i];
        }
        for i in (0..=m - 1).rev() {
            for j in (0..=n - 1).rev() {
                if s1v[i] == s2v[j] {
                    dp[i][j] = dp[i + 1][j + 1];
                } else {
                    dp[i][j] = i32::min(dp[i + 1][j] + s1v[i], dp[i][j + 1] + s2v[j]);
                }
            }
        }
        dp[0][0]
    }

    pub fn minimum_delete_sum_2(s1: String, s2: String) -> i32 {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];

        for i in 1..=s1.len() {
            dp[i][0] = dp[i - 1][0] + s1[i - 1] as i32;
        }

        for j in 1..=s2.len() {
            dp[0][j] = dp[0][j - 1] + s2[j - 1] as i32;
        }

        for i in 1..=s1.len() {
            for j in 1..=s2.len() {
                if s1[i - 1] == s2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = min(dp[i - 1][j] + s1[i - 1] as i32, dp[i][j - 1] + s2[j - 1] as i32);
                }
            }
        }

        dp[s1.len()][s2.len()]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_delete_sum("sea".to_string(), "eat".to_string()), 231);
    assert_eq!(
        Solution::minimum_delete_sum("delete".to_string(), "leet".to_string()),
        403
    );
}
