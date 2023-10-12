#![allow(dead_code)]

/*

// 1930. Unique Length-3 Palindromic Subsequences
// https://leetcode.com/problems/unique-length-3-palindromic-subsequences/
// https://leetcode.cn/problems/unique-length-3-palindromic-subsequences/
//
// Medium
//
// Given a string s, return the number of unique palindromes of length three that are a subsequence of s.

Note that even if there are multiple ways to obtain the same subsequence, it is still only counted once.

A palindrome is a string that reads the same forwards and backwards.

A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.

    For example, "ace" is a subsequence of "abcde".

Example 1:

Input: s = "aabca"
Output: 3
Explanation: The 3 palindromic subsequences of length 3 are:
- "aba" (subsequence of "aabca")
- "aaa" (subsequence of "aabca")
- "aca" (subsequence of "aabca")

Example 2:

Input: s = "adc"
Output: 0
Explanation: There are no palindromic subsequences of length 3 in "adc".

Example 3:

Input: s = "bbcbaba"
Output: 4
Explanation: The 4 palindromic subsequences of length 3 are:
- "bbb" (subsequence of "bbcbaba")
- "bcb" (subsequence of "bbcbaba")
- "bab" (subsequence of "bbcbaba")
- "aba" (subsequence of "bbcbaba")

Constraints:

    3 <= s.length <= 10^5
    s consists of only lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut result = 0;
        let mut seen = [false; 26];
        for i in 0..n {
            let ti = (s[i] as u8 - b'a') as usize;
            if seen[ti] {
                continue;
            }
            seen[ti] = true;
            let mut ti = 0;
            for j in (i + 1..n).rev() {
                if s[i] == s[j] {
                    ti = j;
                    break;
                }
            }
            if ti == 0 {
                continue;
            }
            let mut exists = vec![false; 26];
            for j in i + 1..ti {
                exists[(s[j] as u8 - b'a') as usize] = true;
            }
            for v in exists {
                if v {
                    result += 1;
                }
            }
        }
        result
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("aabca", 3),
        ("adc", 0),
        ("bbcbaba", 4),
        ("bbcbabab", 4),
    ];
    for (s, expected) in cases {
        assert_eq!(Solution::count_palindromic_subsequence(s.to_string()), expected);
    }
}
