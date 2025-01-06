#![allow(dead_code)]

// 3407. Substring Matching Pattern
// https://leetcode.com/problems/substring-matching-pattern/
// https://leetcode.cn/problems/substring-matching-pattern/
//
// Easy
//
// You are given a string s and a pattern string p, where p contains exactly one '*' character.
//
// The '*' in p can be replaced with any sequence of zero or more characters.
//
// Return true if p can be made a substring of s, and false otherwise.
//
// A substring is a contiguous non-empty sequence of characters within a string.
//
// Example 1:
//
// Input: s = "leetcode", p = "ee*e"
//
// Output: true
//
// Explanation:
//
// By replacing the '*' with "tcod", the substring "eetcode" matches the pattern.
//
// Example 2:
//
// Input: s = "car", p = "c*v"
//
// Output: false
//
// Explanation:
//
// There is no substring matching the pattern.
//
// Example 3:
//
// Input: s = "luck", p = "u*"
//
// Output: true
//
// Explanation:
//
// The substrings "u", "uc", and "uck" match the pattern.
//
// Constraints:
//
//     1 <= s.length <= 50
//     1 <= p.length <= 50
//     s contains only lowercase English letters.
//     p contains only lowercase English letters and exactly one '*'
//

struct Solution;

impl Solution {
    pub fn has_match(s: String, p: String) -> bool {
        let x = p.find('*').unwrap();
        let b = &p[0..x];
        let e = &p[x + 1..];
        let i = s.find(b);
        let j = s.rfind(e);
        i.is_some_and(|i| j.is_some_and(|j| i + b.len() <= j))
    }
}

#[test]
fn test() {
    let s = "leetcode".to_string();
    let p = "ee*e".to_string();
    assert!(Solution::has_match(s, p));

    let s = "car".to_string();
    let p = "c*v".to_string();
    assert!(!Solution::has_match(s, p));

    let s = "luck".to_string();
    let p = "u*".to_string();
    assert!(Solution::has_match(s, p));
}
