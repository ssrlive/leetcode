#![allow(dead_code)]

// 567. Permutation in String
// https://leetcode.com/problems/permutation-in-string/
//
// Given two strings s1 and s2, return true if s2 contains a permutation of s1, or false otherwise.
//
// In other words, return true if one of s1's permutations is the substring of s2.
//
// Example 1:
//
// Input: s1 = "ab", s2 = "eidbaooo"
// Output: true
// Explanation: s2 contains one permutation of s1 ("ba").
//
// Example 2:
//
// Input: s1 = "ab", s2 = "eidboaoo"
// Output: false
//
// Constraints:
//
// - 1 <= s1.length, s2.length <= 10^4
// - s1 and s2 consist of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        Self::_check_inclusion(s1, s2).unwrap_or(false)
    }

    pub fn _check_inclusion(s1: String, s2: String) -> Option<bool> {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut s1_count = [0; 26];
        let mut s2_count = [0; 26];

        for i in 0..s1.len() {
            s1_count[(s1[i] - b'a') as usize] += 1;
            s2_count[(s2.get(i)? - b'a') as usize] += 1;
        }

        if s1_count == s2_count {
            return Some(true);
        }

        for i in s1.len()..s2.len() {
            s2_count[(s2[i - s1.len()] - b'a') as usize] -= 1;
            s2_count[(s2[i] - b'a') as usize] += 1;

            if s1_count == s2_count {
                return Some(true);
            }
        }

        Some(false)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::check_inclusion("ab".to_string(), "a".to_string()), false);
    assert_eq!(
        Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()),
        true
    );
    assert_eq!(
        Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()),
        false
    );
}
