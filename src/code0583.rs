#![allow(dead_code)]

// 583. Delete Operation for Two Strings
// https://leetcode.com/problems/delete-operation-for-two-strings/
//
// Given two strings word1 and word2, return the minimum number of steps required to make word1 and word2 the same.
//
// In one step, you can delete exactly one character in either string.
//
// Example 1:
//
// Input: word1 = "sea", word2 = "eat"
// Output: 2
// Explanation: You need one step to make "sea" to "ea" and another step to make "eat" to "ea".
//
// Example 2:
//
// Input: word1 = "leetcode", word2 = "etco"
// Output: 4
//
// Constraints:
//
// - 1 <= word1.length, word2.length <= 500
// - word1 and word2 consist of only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let mut dp = vec![vec![0; word2.len() + 1]; word1.len() + 1];

        for i in 1..=word1.len() {
            for j in 1..=word2.len() {
                if word1[i - 1] == word2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }

        (word1.len() + word2.len() - 2 * dp[word1.len()][word2.len()]) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_distance("sea".to_string(), "eat".to_string()), 2);
    assert_eq!(Solution::min_distance("leetcode".to_string(), "etco".to_string()), 4);
}
