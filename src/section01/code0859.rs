#![allow(dead_code)]

// 859. Buddy Strings
// https://leetcode.com/problems/buddy-strings/
// https://leetcode.cn/problems/buddy-strings/
//
// Given two strings s and goal, return true if you can swap two letters in s so the result is equal to goal, otherwise, return false.
//
// Swapping letters is defined as taking two indices i and j (0-indexed) such that i != j and swapping the characters at s[i] and s[j].
//
// For example, swapping at indices 0 and 2 in "abcd" results in "cbad".
//
// Example 1:
//
// Input: s = "ab", goal = "ba"
// Output: true
// Explanation: You can swap s[0] = 'a' and s[1] = 'b' to get "ba", which is equal to goal.
//
// Example 2:
//
// Input: s = "ab", goal = "ab"
// Output: false
// Explanation: The only letters you can swap are s[0] = 'a' and s[1] = 'b', which results in "ba" != goal.
//
// Example 3:
//
// Input: s = "aa", goal = "aa"
// Output: true
// Explanation: You can swap s[0] = 'a' and s[1] = 'a' to get "aa", which is equal to goal.
//
// Constraints:
//
// - 1 <= s.length, goal.length <= 2 * 10^4
// - s and goal consist of lowercase letters.
//

struct Solution;

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        let s1 = s.as_bytes();
        let s2 = goal.as_bytes();
        if s1.len() != s2.len() {
            return false;
        }
        let mut s = std::collections::HashSet::new();
        let mut not_same = Vec::new();
        for i in 0..s1.len() {
            if s1[i] != s2[i] {
                not_same.push(i);
                if not_same.len() > 2 {
                    return false;
                }
            }
            s.insert(s1[i]);
        }
        (not_same.is_empty() && s.len() < s1.len())
            || (not_same.len() == 2 && s1[not_same[0]] == s2[not_same[1]] && s1[not_same[1]] == s2[not_same[0]])
    }
}

#[test]
fn test() {
    assert!(Solution::buddy_strings("ab".to_string(), "ba".to_string()));
    assert!(!Solution::buddy_strings("ab".to_string(), "ab".to_string()));
    assert!(Solution::buddy_strings("aa".to_string(), "aa".to_string()));
    assert!(Solution::buddy_strings("aaaaaaabc".to_string(), "aaaaaaacb".to_string()));
    assert!(!Solution::buddy_strings("".to_string(), "aa".to_string()));
}
