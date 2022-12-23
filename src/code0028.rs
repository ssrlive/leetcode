#![allow(dead_code)]

// 28. Find the Index of the First Occurrence in a String
// https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/
// https://leetcode.cn/problems/find-the-index-of-the-first-occurrence-in-a-string/
//
// Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
//
// Example 1:
//
// Input: haystack = "sadbutsad", needle = "sad"
// Output: 0
// Explanation: "sad" occurs at index 0 and 6.
// The first occurrence is at index 0, so we return 0.
//
// Example 2:
//
// Input: haystack = "leetcode", needle = "leeto"
// Output: -1
// Explanation: "leeto" did not occur in "leetcode", so we return -1.
//
// Constraints:
//
// - 1 <= haystack.length, needle.length <= 104
// - haystack and needle consist of only lowercase English characters.
//

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map(|i| i as i32).unwrap_or(-1)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::str_str("sadbutsad".to_string(), "sad".to_string()), 0);
    assert_eq!(Solution::str_str("leetcode".to_string(), "leeto".to_string()), -1);
}
