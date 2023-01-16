#![allow(dead_code)]

// 5. Longest Palindromic Substring
// https://leetcode.com/problems/longest-palindromic-substring/
// https://leetcode.cn/problems/longest-palindromic-substring/
//
// Given a string s, return the longest palindromic substring in s.
//
// Example 1:
//
// Input: s = "babad"
// Output: "bab"
// Explanation: "aba" is also a valid answer.
//
// Example 2:
//
// Input: s = "cbbd"
// Output: "bb"
//
// Constraints:
//
// - 1 <= s.length <= 1000
// - s consist of only digits and English letters.
//

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut max = 0;
        let mut start = 0;
        let mut end = 0;
        let mut dp = vec![vec![false; s.len()]; s.len()];
        for i in 0..s.len() {
            for j in 0..=i {
                if s.as_bytes()[i] == s.as_bytes()[j] && (i - j <= 2 || dp[j + 1][i - 1]) {
                    dp[j][i] = true;
                    if i - j + 1 > max {
                        max = i - j + 1;
                        start = j;
                        end = i;
                    }
                }
            }
        }
        s[start..=end].to_string()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab".to_string());
    assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb".to_string());
}
