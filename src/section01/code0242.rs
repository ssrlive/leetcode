#![allow(dead_code)]

// 242. Valid Anagram
// https://leetcode.com/problems/valid-anagram/
// https://leetcode.cn/problems/valid-anagram/
//
// Given two strings s and t, return true if t is an anagram of s, and false otherwise.
//
// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
// typically using all the original letters exactly once.
//
// Example 1:
//
// Input: s = "anagram", t = "nagaram"
// Output: true
//
// Example 2:
//
// Input: s = "rat", t = "car"
// Output: false
//
// Constraints:
//
// 1 <= s.length, t.length <= 5 * 104
// s and t consist of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map = std::collections::HashMap::new();
        s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
        t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);
        map.into_values().all(|v| v == 0)
    }

    pub fn is_anagram_2(s: String, t: String) -> bool {
        let mut s = s.chars().collect::<Vec<_>>();
        let mut t = t.chars().collect::<Vec<_>>();
        s.sort();
        t.sort();
        s == t
    }
}

#[test]
fn test_is_anagram() {
    assert!(Solution::is_anagram("anagram".to_string(), "nagaram".to_string()));
    assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));
}
