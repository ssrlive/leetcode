#![allow(dead_code)]

// 10. Regular Expression Matching
// https://leetcode.com/problems/regular-expression-matching/
// https://leetcode.cn/problems/regular-expression-matching/
//
// Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:
//
// '.' Matches any single character.​​​​
// '*' Matches zero or more of the preceding element.
// The matching should cover the entire input string (not partial).
//
// Example 1:
//
// Input: s = "aa", p = "a"
// Output: false
// Explanation: "a" does not match the entire string "aa".
//
// Example 2:
//
// Input: s = "aa", p = "a*"
// Output: true
// Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
//
// Example 3:
//
// Input: s = "ab", p = ".*"
// Output: true
// Explanation: ".*" means "zero or more (*) of any character (.)".
//
// Constraints:
//
// - 1 <= s.length <= 20
// - 1 <= p.length <= 30
// - s contains only lowercase English letters.
// - p contains only lowercase English letters, '.', and '*'.
// - It is guaranteed for each appearance of the character '*', there will be a previous valid character to match.
//

struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        dp[0][0] = true;
        for i in 0..=s.len() {
            for j in 1..=p.len() {
                if p[j - 1] == b'*' {
                    dp[i][j] = dp[i][j - 2] || i > 0 && dp[i - 1][j] && (s[i - 1] == p[j - 2] || p[j - 2] == b'.');
                } else {
                    dp[i][j] = i > 0 && dp[i - 1][j - 1] && (s[i - 1] == p[j - 1] || p[j - 1] == b'.');
                }
            }
        }
        dp[s.len()][p.len()]
    }
}

#[test]
fn test() {
    let cases = vec![
        ("aa", "a", false),
        ("aa", "a*", true),
        ("ab", ".*", true),
        ("aab", "c*a*b", true),
        ("mississippi", "mis*is*p*.", false),
    ];
    for (s, p, expected) in cases {
        assert_eq!(Solution::is_match(s.to_string(), p.to_string()), expected);
    }
}
