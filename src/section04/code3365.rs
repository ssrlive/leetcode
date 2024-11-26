#![allow(dead_code)]

// 3365. Rearrange K Substrings to Form Target String
// https://leetcode.com/problems/rearrange-k-substrings-to-form-target-string/
// https://leetcode.cn/problems/rearrange-k-substrings-to-form-target-string/
//
// Medium
//
// You are given two strings s and t, both of which are anagrams of each other, and an integer k.
//
// Your task is to determine whether it is possible to split the string s into k equal-sized substrings, rearrange the substrings,
// and concatenate them in any order to create a new string that matches the given string t.
//
// Return true if this is possible, otherwise, return false.
//
// An anagram is a word or phrase formed by rearranging the letters of a different word or phrase, using all the original letters exactly once.
//
// A substring is a contiguous non-empty sequence of characters within a string.
//
// Example 1:
//
// Input: s = "abcd", t = "cdab", k = 2
//
// Output: true
//
// Explanation:
//
// Split s into 2 substrings of length 2: ["ab", "cd"].
// Rearranging these substrings as ["cd", "ab"], and then concatenating them results in "cdab", which matches t.
//
// Example 2:
//
// Input: s = "aabbcc", t = "bbaacc", k = 3
//
// Output: true
//
// Explanation:
//
// Split s into 3 substrings of length 2: ["aa", "bb", "cc"].
// Rearranging these substrings as ["bb", "aa", "cc"], and then concatenating them results in "bbaacc", which matches t.
//
// Example 3:
//
// Input: s = "aabbcc", t = "bbaacc", k = 2
//
// Output: false
//
// Explanation:
//
// Split s into 2 substrings of length 3: ["aab", "bcc"].
// These substrings cannot be rearranged to form t = "bbaacc", so the output is false.
//
// Constraints:
//
// 1 <= s.length == t.length <= 2 * 10^5
// 1 <= k <= s.length
// s.length is divisible by k.
// s and t consist only of lowercase English letters.
// The input is generated such that s and t are anagrams of each other.
//

struct Solution;

impl Solution {
    pub fn is_possible_to_rearrange(s1: String, s2: String, k: i32) -> bool {
        use std::collections::HashMap;
        let k = s1.len() / k as usize;

        let count_chunks = |s: &String| -> HashMap<String, usize> {
            let mut counter = HashMap::new();
            for i in (0..s.len()).step_by(k) {
                let chunk = s[i..i + k].to_string();
                *counter.entry(chunk).or_insert(0) += 1;
            }
            counter
        };

        count_chunks(&s1) == count_chunks(&s2)
    }
}

#[test]
fn test() {
    assert!(Solution::is_possible_to_rearrange("abcd".to_string(), "cdab".to_string(), 2));
    assert!(Solution::is_possible_to_rearrange("aabbcc".to_string(), "bbaacc".to_string(), 3));
    assert!(!Solution::is_possible_to_rearrange("aabbcc".to_string(), "bbaacc".to_string(), 2));
}
