#![allow(dead_code)]

// 389. Find the Difference
// https://leetcode.com/problems/find-the-difference/
// https://leetcode.cn/problems/find-the-difference/
//
// You are given two strings s and t.
//
// String t is generated by random shuffling string s and then add one more letter at a random position.
//
// Return the letter that was added to t.
//
// Example 1:
//
// Input: s = "abcd", t = "abcde"
// Output: "e"
// Explanation: 'e' is the letter that was added.
//
// Example 2:
//
// Input: s = "", t = "y"
// Output: "y"
//
// Constraints:
//
// - 0 <= s.length <= 1000
// - t.length == s.length + 1
// - s and t consist of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut s = s.chars().collect::<Vec<_>>();
        let mut t = t.chars().collect::<Vec<_>>();
        s.sort();
        t.sort();
        for i in 0..s.len() {
            if s[i] != t[i] {
                return t[i];
            }
        }
        t[t.len() - 1]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_the_difference("abcd".to_string(), "abcde".to_string()), 'e');
    assert_eq!(Solution::find_the_difference("".to_string(), "y".to_string()), 'y');
}
