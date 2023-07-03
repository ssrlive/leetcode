#![allow(dead_code)]

// 1143. Longest Common Subsequence
// https://leetcode.com/problems/longest-common-subsequence/
// https://leetcode.cn/problems/longest-common-subsequence/
//
// Given two strings text1 and text2, return the length of their longest common subsequence. If there is no common subsequence, return 0.
//
// A subsequence of a string is a new string generated from the original string with some characters (can be none)
// deleted without changing the relative order of the remaining characters.
//
// For example, "ace" is a subsequence of "abcde".
// A common subsequence of two strings is a subsequence that is common to both strings.
//
// Example 1:
//
// Input: text1 = "abcde", text2 = "ace"
// Output: 3
// Explanation: The longest common subsequence is "ace" and its length is 3.
//
// Example 2:
//
// Input: text1 = "abc", text2 = "abc"
// Output: 3
// Explanation: The longest common subsequence is "abc" and its length is 3.
//
// Example 3:
//
// Input: text1 = "abc", text2 = "def"
// Output: 0
// Explanation: There is no such common subsequence, so the result is 0.
//
// Constraints:
//
// - 1 <= text1.length, text2.length <= 1000
// - text1 and text2 consist of only lowercase English characters.
//

struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (text1, text2) = (text1.as_bytes(), text2.as_bytes());
        let (n1, n2) = (text1.len(), text2.len());

        let mut dp_prev = vec![0; n2 + 1];
        let mut dp_curr = dp_prev.clone();

        for i in (0..n1).rev() {
            for j in (0..n2).rev() {
                dp_curr[j] = dp_prev[j].max(dp_curr[j + 1]).max(dp_prev[j + 1] + i32::from(text1[i] == text2[j]));
            }
            std::mem::swap(&mut dp_prev, &mut dp_curr);
        }

        dp_prev[0]
    }
}

#[test]
fn test() {
    let cases = vec![
        ("abcde", "ace", 3),
        ("abc", "abc", 3),
        ("abc", "def", 0),
        ("bsbininm", "jmjkbkjkv", 1),
        ("oxcpqrsvwf", "shmtulqrypy", 2),
    ];
    for (text1, text2, expected) in cases {
        let text1 = text1.to_string();
        let text2 = text2.to_string();
        assert_eq!(Solution::longest_common_subsequence(text1, text2), expected);
    }
}
