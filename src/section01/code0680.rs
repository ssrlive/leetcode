#![allow(dead_code)]

// 680. Valid Palindrome II
// https://leetcode.com/problems/valid-palindrome-ii/description/
// https://leetcode.cn/problems/valid-palindrome-ii/description/
//
// Given a string s, return true if the s can be palindrome after deleting at most one character from it.
//
// Example 1:
//
// Input: s = "aba"
// Output: true
//
// Example 2:
//
// Input: s = "abca"
// Output: true
// Explanation: You could delete the character 'c'.
//
// Example 3:
//
// Input: s = "abc"
// Output: false
//
// Constraints:
//
// - 1 <= s.length <= 10^5
// - s consists of lowercase English letters.
//
// Follow up: Could you solve it in O(n) time and O(1) space?

struct Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if s[i] != s[j] {
                return Solution::is_palindrome(&s[i + 1..j + 1]) || Solution::is_palindrome(&s[i..j]);
            }
            i += 1;
            j -= 1;
        }
        true
    }

    fn is_palindrome(s: &[u8]) -> bool {
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if s[i] != s[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

#[test]
fn test() {
    assert!(Solution::valid_palindrome("aba".to_string()));
    assert!(Solution::valid_palindrome("abca".to_string()));
    assert!(!Solution::valid_palindrome("abc".to_string()));
}
