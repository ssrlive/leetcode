#![allow(dead_code)]

// 3456. Find Special Substring of Length K
// https://leetcode.com/problems/find-special-substring-of-length-k/
// https://leetcode.cn/problems/find-special-substring-of-length-k/
//
// Easy
//
// You are given a string s and an integer k.
//
// Determine if there exists a substring of length exactly k in s that satisfies the following conditions:
//
//     The substring consists of only one distinct character (e.g., "aaa" or "bbb").
//     If there is a character immediately before the substring, it must be different from the character in the substring.
//     If there is a character immediately after the substring, it must also be different from the character in the substring.
//
// Return true if such a substring exists. Otherwise, return false.
//
// Example 1:
//
// Input: s = "aaabaaa", k = 3
//
// Output: true
//
// Explanation:
//
// The substring s[4..6] == "aaa" satisfies the conditions.
//
//     It has a length of 3.
//     All characters are the same.
//     The character before "aaa" is 'b', which is different from 'a'.
//     There is no character after "aaa".
//
// Example 2:
//
// Input: s = "abc", k = 2
//
// Output: false
//
// Explanation:
//
// There is no substring of length 2 that consists of one distinct character and satisfies the conditions.
//
// Constraints:
//
//     1 <= k <= s.length <= 100
//     s consists of lowercase English letters only.
//

struct Solution;

impl Solution {
    pub fn has_special_substring(s: String, k: i32) -> bool {
        let k = k as usize;
        let mut last = s.chars().next().unwrap();
        let mut m = 1;
        for x in s.chars().skip(1) {
            if x != last {
                if m == k {
                    return true;
                }
                m = 1;
            } else {
                m += 1;
            }
            last = x;
        }
        m == k
    }
}

#[test]
fn test() {
    assert!(Solution::has_special_substring("aaabaaa".to_string(), 3));
    assert!(!Solution::has_special_substring("abc".to_string(), 2));
}
