#![allow(dead_code)]

// 3146. Permutation Difference between Two Strings
// https://leetcode.com/problems/permutation-difference-between-two-strings/
// https://leetcode.cn/problems/permutation-difference-between-two-strings/
//
// Easy
//
// You are given two strings s and t such that every character occurs at most once in s and t is a permutation of s.
//
// The permutation difference between s and t is defined as the sum of the absolute difference between the index of the
// occurrence of each character in s and the index of the occurrence of the same character in t.
//
// Return the permutation difference between s and t.
//
// Example 1:
//
// Input: s = "abc", t = "bac"
//
// Output: 2
//
// Explanation:
//
// For s = "abc" and t = "bac", the permutation difference of s and t is equal to the sum of:
//
// The absolute difference between the index of the occurrence of "a" in s and the index of the occurrence of "a" in t.
// The absolute difference between the index of the occurrence of "b" in s and the index of the occurrence of "b" in t.
// The absolute difference between the index of the occurrence of "c" in s and the index of the occurrence of "c" in t.
// That is, the permutation difference between s and t is equal to |0 - 1| + |2 - 2| + |1 - 0| = 2.
//
// Example 2:
//
// Input: s = "abcde", t = "edbac"
//
// Output: 12
//
// Explanation: The permutation difference between s and t is equal to |0 - 3| + |1 - 2| + |2 - 4| + |3 - 1| + |4 - 0| = 12.
//
// Constraints:
//
// 1 <= s.length <= 26
// Each character occurs at most once in s.
// t is a permutation of s.
// s consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        (0..=0)
            .fold((0x45, [0x0; 0x1a]), |(_, mut m), _| {
                t.as_bytes().iter().enumerate().for_each(|(i, c)| m[*c as usize - 0x61] = i);
                (
                    s.as_bytes().iter().enumerate().fold(0, |mut ret, (i, c)| {
                        ret += m[*c as usize - 0x61].abs_diff(i);
                        ret
                    }),
                    m,
                )
            })
            .0 as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_permutation_difference("abc".to_string(), "bac".to_string()), 2);
    assert_eq!(Solution::find_permutation_difference("abcde".to_string(), "edbac".to_string()), 12);
}
