#![allow(dead_code)]

// 2486. Append Characters to String to Make Subsequence
// https://leetcode.com/problems/append-characters-to-string-to-make-subsequence/
// https://leetcode.cn/problems/append-characters-to-string-to-make-subsequence/
//
// You are given two strings s and t consisting of only lowercase English letters.
//
// Return the minimum number of characters that need to be appended to the end of s so that t becomes a subsequence of s.
//
// A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.
//
// Example 1:
//
// Input: s = "coaching", t = "coding"
// Output: 4
// Explanation: Append the characters "ding" to the end of s so that s = "coachingding".
// Now, t is a subsequence of s ("coachingding").
// It can be shown that appending any 3 characters to the end of s will never make t a subsequence.
//
// Example 2:
//
// Input: s = "abcde", t = "a"
// Output: 0
// Explanation: t is already a subsequence of s ("abcde").
//
// Example 3:
//
// Input: s = "z", t = "abcde"
// Output: 5
// Explanation: Append the characters "abcde" to the end of s so that s = "zabcde".
// Now, t is a subsequence of s ("zabcde").
// It can be shown that appending any 4 characters to the end of s will never make t a subsequence.
//
// Constraints:
//
// - 1 <= s.length, t.length <= 10^5
// - s and t consist only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let sa: Vec<char> = s.chars().collect();
        let ta: Vec<char> = t.chars().collect();
        let n1 = s.len();
        let n2 = t.len();

        while i < n1 && j < n2 {
            if sa[i] == ta[j] {
                i += 1;
                j += 1;
            } else {
                i += 1;
            }
        }
        (n2 - j) as i32
    }
}

#[test]
fn test() {
    let s = "coaching".to_string();
    assert_eq!(Solution::append_characters(s, "coding".to_string()), 4);
    assert_eq!(Solution::append_characters("abcde".to_string(), "a".to_string()), 0);
    assert_eq!(Solution::append_characters("z".to_string(), "abcde".to_string()), 5);
}
