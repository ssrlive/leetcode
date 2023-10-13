#![allow(dead_code)]

// 459. Repeated Substring Pattern
// https://leetcode.com/problems/repeated-substring-pattern/
// https://leetcode.cn/problems/repeated-substring-pattern/
//
// Given a string s, check if it can be constructed by taking a substring of it and appending multiple copies of the substring together.
//
// Example 1:
//
// Input: s = "abab"
// Output: true
// Explanation: It is the substring "ab" twice.
//
// Example 2:
//
// Input: s = "aba"
// Output: false
//
// Example 3:
//
// Input: s = "abcabcabcabc"
// Output: true
// Explanation: It is the substring "abc" four times or the substring "abcabc" twice.
//
// Constraints:
//
// - 1 <= s.length <= 10^4
// - s consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s = s.as_bytes();
        let n = s.len();
        for i in 1..=n / 2 {
            if n % i == 0 {
                let mut j = i;
                while j < n && s[j] == s[j % i] {
                    j += 1;
                }
                if j == n {
                    return true;
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    assert!(Solution::repeated_substring_pattern("abab".to_string()));
    assert!(!Solution::repeated_substring_pattern("aba".to_string()));
    assert!(Solution::repeated_substring_pattern("abcabcabcabc".to_string()));
}
