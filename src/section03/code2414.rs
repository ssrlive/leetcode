#![allow(dead_code)]

// 2414. Length of the Longest Alphabetical Continuous Substring
// https://leetcode.com/problems/length-of-the-longest-alphabetical-continuous-substring/
// https://leetcode.cn/problems/length-of-the-longest-alphabetical-continuous-substring/
//
// An alphabetical continuous string is a string consisting of consecutive letters in the alphabet.
// In other words, it is any substring of the string "abcdefghijklmnopqrstuvwxyz".
//
// For example, "abc" is an alphabetical continuous string, while "acb" and "za" are not.
// Given a string s consisting of lowercase letters only, return the length of the longest alphabetical continuous substring.
//
// Example 1:
//
// Input: s = "abacaba"
// Output: 2
// Explanation: There are 4 distinct continuous substrings: "a", "b", "c" and "ab".
// "ab" is the longest continuous substring.
//
// Example 2:
//
// Input: s = "abcde"
// Output: 5
// Explanation: "abcde" is the longest continuous substring.
//
// Constraints:
//
// - 1 <= s.length <= 10^5
// - s consists of only English lowercase letters.
//

struct Solution;

impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let f = |(l_max, l): (i32, i32), (b1, b2): (u8, u8)| match b2.saturating_sub(b1) {
            1 => (l_max.max(l + 1), l + 1),
            _ => (l_max, 1),
        };
        s.bytes().zip(s[1..].bytes()).fold((1, 1), f).0
    }
}

#[test]
fn test() {
    assert_eq!(Solution::longest_continuous_substring("abacaba".to_string()), 2);
    assert_eq!(Solution::longest_continuous_substring("abcde".to_string()), 5);
    // assert_eq!(Solution::longest_continuous_substring("".to_string()), 1);
    assert_eq!(Solution::longest_continuous_substring("a".to_string()), 1);
    assert_eq!(Solution::longest_continuous_substring("ab".to_string()), 2);
    assert_eq!(Solution::longest_continuous_substring("ba".to_string()), 1);
    assert_eq!(Solution::longest_continuous_substring("abc".to_string()), 3);
    assert_eq!(Solution::longest_continuous_substring("cba".to_string()), 1);
    assert_eq!(Solution::longest_continuous_substring("abca".to_string()), 3);
    assert_eq!(Solution::longest_continuous_substring("acba".to_string()), 1);
    assert_eq!(Solution::longest_continuous_substring("abcab".to_string()), 3);
    assert_eq!(Solution::longest_continuous_substring("acbab".to_string()), 2);
}
