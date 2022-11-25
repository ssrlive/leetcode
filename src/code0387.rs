#![allow(dead_code)]

// 387. First Unique Character in a String
// https://leetcode.com/problems/first-unique-character-in-a-string/
//
// Given a string s, return the first non-repeating character in it and return
// its index. If it does not exist, return -1.
//
// Example 1:
//
// Input: s = "leetcode"
// Output: 0
//
// Example 2:
//
// Input: s = "loveleetcode"
// Output: 2
//
// Example 3:
//
// Input: s = "aabb"
// Output: -1
//
// Constraints:
//
// 1 <= s.length <= 10^5
// s consists of only lowercase English letters.

struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map = std::collections::HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        for (i, c) in s.chars().enumerate() {
            if map.get(&c).unwrap() == &1 {
                return i as i32;
            }
        }
        -1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
    assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
    assert_eq!(Solution::first_uniq_char("aabb".to_string()), -1);
}
