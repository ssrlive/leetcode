#![allow(dead_code)]

// 205. Isomorphic Strings
// https://leetcode.com/problems/isomorphic-strings/
//
// Given two strings s and t, determine if they are isomorphic.
//
// Two strings are isomorphic if the characters in s can be replaced to get t.
//
// All occurrences of a character must be replaced with another character while
// preserving the order of characters. No two characters may map to the same
// character but a character may map to itself.

struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map = std::collections::HashMap::new();
        let mut set = std::collections::HashSet::new();
        for (c1, c2) in s.chars().zip(t.chars()) {
            if let Some(&c) = map.get(&c1) {
                if c != c2 {
                    return false;
                }
            } else {
                if set.contains(&c2) {
                    return false;
                }
                map.insert(c1, c2);
                set.insert(c2);
            }
        }
        true
    }
}

#[test]
fn test_is_isomorphic() {
    assert_eq!(Solution::is_isomorphic("egg".to_string(), "add".to_string()), true);
    assert_eq!(Solution::is_isomorphic("foo".to_string(), "bar".to_string()), false);
    assert_eq!(Solution::is_isomorphic("paper".to_string(), "title".to_string()), true);
    assert_eq!(Solution::is_isomorphic("ab".to_string(), "aa".to_string()), false);
}
