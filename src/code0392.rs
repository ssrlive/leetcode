#![allow(dead_code)]

// 392. Is Subsequence
// https://leetcode.com/problems/is-subsequence/
//
// Given two strings s and t, return true if s is a subsequence of t, or false otherwise.
//
// A subsequence of a string is a new string that is formed from the original string by
// deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters.
// (i.e., "ace" is a subsequence of "abcde" while "aec" is not).
//
// Example 1:
//
// Input: s = "abc", t = "ahbgdc"
// Output: true
//
// Example 2:
//
// Input: s = "axc", t = "ahbgdc"
// Output: false
//
// Constraints:
//
// - 0 <= s.length <= 100
// - 0 <= t.length <= 104
// - s and t consist only of lowercase English letters.
//
// Follow up: Suppose there are lots of incoming s, say s1, s2, ..., sk where k >= 109,
// and you want to check one by one to see if t has its subsequence. In this scenario, how would you change your code?
//

struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut iter = t.chars();
        for c in s.chars() {
            match iter.find(|&p| p == c) {
                Some(_) => (),
                None => return false,
            }
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()), true);
    assert_eq!(Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string()), false);
}
