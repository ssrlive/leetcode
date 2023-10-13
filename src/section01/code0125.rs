#![allow(dead_code)]

// 125. Valid Palindrome
// https://leetcode.com/problems/valid-palindrome/
// https://leetcode.cn/problems/valid-palindrome/
//
// A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.
//
// Given a string s, return true if it is a palindrome, or false otherwise.

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let iter = s.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_ascii_lowercase());

        iter.clone().eq(iter.rev())
    }
}

#[test]
fn test_is_palindrome() {
    assert!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()));
    assert!(!Solution::is_palindrome("race a car".to_string()));
    assert!(Solution::is_palindrome("   ".to_string()));
}
