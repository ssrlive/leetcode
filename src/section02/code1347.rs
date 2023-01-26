#![allow(dead_code)]

// 1347. Minimum Number of Steps to Make Two Strings Anagram
// https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram/
// https://leetcode.cn/problems/minimum-number-of-steps-to-make-two-strings-anagram/
//
// Medium
//
// You are given two strings of the same length s and t. In one step you can choose any
// character of t and replace it with another character.
//
// Return the minimum number of steps to make t an anagram of s.
//
// An Anagram of a string is a string that contains the same characters with a different (or the same) ordering.
//
// Example 1:
//
// Input: s = "bab", t = "aba"
// Output: 1
// Explanation: Replace the first 'a' in t with b, t = "bba" which is anagram of s.
//
// Example 2:
//
// Input: s = "leetcode", t = "practice"
// Output: 5
// Explanation: Replace 'p', 'r', 'a', 'i' and 'c' from t with proper characters to make t anagram of s.
//
// Example 3:
//
// Input: s = "anagram", t = "mangaar"
// Output: 0
// Explanation: "anagram" and "mangaar" are anagrams.
//
// Constraints:
//
// -    1 <= s.length <= 5 * 10^4
// -    s.length == t.length
// -    s and t consist of lowercase English letters only.
//

struct Solution;

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut freqs = std::collections::HashMap::new();
        for ch in s.chars() {
            *freqs.entry(ch).or_insert(0) += 1;
        }

        for ch in t.chars() {
            *freqs.entry(ch).or_insert(0) -= 1;
        }

        freqs.values().into_iter().filter(|&&item| item > 0).sum()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_steps("bab".to_string(), "aba".to_string()), 1);
    assert_eq!(Solution::min_steps("leetcode".to_string(), "practice".to_string()), 5);
    assert_eq!(Solution::min_steps("anagram".to_string(), "mangaar".to_string()), 0);
}
