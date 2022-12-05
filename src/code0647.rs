#![allow(dead_code)]

// 647. Palindromic Substrings
// https://leetcode.com/problems/palindromic-substrings/
//
// Given a string s, return the number of palindromic substrings in it.
//
// A string is a palindrome when it reads the same backward as forward.
//
// A substring is a contiguous sequence of characters within the string.
//
// Example 1:
//
// Input: s = "abc"
// Output: 3
// Explanation: Three palindromic strings: "a", "b", "c".
//
// Example 2:
//
// Input: s = "aaa"
// Output: 6
// Explanation: Six palindromic strings: "a", "a", "a", "aa", "aa", "aaa".
//
// Constraints:
//
// - 1 <= s.length <= 1000
// - s consists of lowercase English letters.
//
struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        fn _count_parindrome(s: &[u8], i: usize, j: usize) -> i32 {
            if j == s.len() || s[i] != s[j] {
                return 0;
            }
            let (mut i, mut j) = (i, j);
            let mut ret = 1;
            while i > 0 && j < s.len() - 1 && s[i - 1] == s[j + 1] {
                ret += 1;
                i -= 1;
                j += 1;
            }
            ret
        }

        let s = s.as_bytes();
        (0..s.len())
            .map(|i| _count_parindrome(s, i, i) + _count_parindrome(s, i, i + 1))
            .sum()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_substrings("abc".to_string()), 3);
    assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
}
