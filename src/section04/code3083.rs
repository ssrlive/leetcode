#![allow(dead_code)]

// 3083. Existence of a Substring in a String and Its Reverse
// https://leetcode.com/problems/existence-of-a-substring-in-a-string-and-its-reverse/
// https://leetcode.cn/problems/existence-of-a-substring-in-a-string-and-its-reverse/
//
// Easy
//
// Given a string s, find any substring of length 2 which is also present in the reverse of s.
//
// Return true if such a substring exists, and false otherwise.
//
// Example 1:
//
// Input: s = "leetcode"
//
// Output: true
//
// Explanation: Substring "ee" is of length 2 which is also present in reverse(s) == "edocteel".
//
// Example 2:
//
// Input: s = "abcba"
//
// Output: true
//
// Explanation: All of the substrings of length 2 "ab", "bc", "cb", "ba" are also present in reverse(s) == "abcba".
//
// Example 3:
//
// Input: s = "abcd"
//
// Output: false
//
// Explanation: There is no substring of length 2 in s, which is also present in the reverse of s.
//
// Constraints:
//
//     1 <= s.length <= 100
//     s consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn is_substring_present(s: String) -> bool {
        let mut occured = [[false; 26]; 26];
        let mut s = s.chars().map(|c| c as usize - 97);

        let mut p = s.next().unwrap();
        for q in s {
            if p == q || occured[q][p] {
                return true;
            }
            occured[p][q] = true;
            p = q;
        }

        false
    }
}

#[test]
fn test() {
    assert!(Solution::is_substring_present("leetcode".to_string()));
    assert!(Solution::is_substring_present("abcba".to_string()));
    assert!(!Solution::is_substring_present("abcd".to_string()));
}
