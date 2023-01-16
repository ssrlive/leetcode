#![allow(dead_code)]

// 290. Word Pattern
// https://leetcode.com/problems/word-pattern/
// https://leetcode.cn/problems/word-pattern/
//
// Given a pattern and a string s, find if s follows the same pattern.
//
// Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in s.
//
// Example 1:
//
// Input: pattern = "abba", s = "dog cat cat dog"
// Output: true
//
// Example 2:
//
// Input: pattern = "abba", s = "dog cat cat fish"
// Output: false
//
// Example 3:
//
// Input: pattern = "aaaa", s = "dog cat cat dog"
// Output: false
//
// Example 4:
//
// Input: pattern = "abba", s = "dog dog dog dog"
// Output: false
//
// Constraints:
//
// - 1 <= pattern.length <= 300
// - pattern contains only lower-case English letters.
// - 1 <= s.length <= 3000
// - s contains only lower-case English letters and spaces ' '.
// - s does not contain any leading or trailing spaces.
// - All the words in s are separated by a single space.
//

struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut map = std::collections::HashMap::new();
        let mut set = std::collections::HashSet::new();
        let mut iter = s.split_whitespace();
        for c in pattern.chars() {
            if let Some(word) = iter.next() {
                if let Some(&p) = map.get(word) {
                    if p != c {
                        return false;
                    }
                } else {
                    if set.contains(&c) {
                        return false;
                    }
                    map.insert(word, c);
                    set.insert(c);
                }
            } else {
                return false;
            }
        }
        iter.next().is_none()
    }
}

#[test]
fn test_word_pattern() {
    assert!(Solution::word_pattern(
        "abba".to_string(),
        "dog cat cat dog".to_string()
    ));
    assert!(Solution::word_pattern(
        "abbc".to_string(),
        "dog cat cat fish".to_string()
    ));
    assert!(!Solution::word_pattern(
        "aaaa".to_string(),
        "dog cat cat dog".to_string()
    ));
    assert!(!Solution::word_pattern(
        "abba".to_string(),
        "dog dog dog dog".to_string()
    ));
}
