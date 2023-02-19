#![allow(dead_code)]

/*

// 1745. Palindrome Partitioning IV
// https://leetcode.com/problems/palindrome-partitioning-iv/
// https://leetcode.cn/problems/palindrome-partitioning-iv/
//
// Hard
//
// Given a string s, return true if it is possible to split the string s into three non-empty palindromic substrings. Otherwise, return false.​​​​​

A string is said to be palindrome if it the same string when reversed.

Example 1:

Input: s = "abcbdd"
Output: true
Explanation: "abcbdd" = "a" + "bcb" + "dd", and all three substrings are palindromes.

Example 2:

Input: s = "bcbddxy"
Output: false
Explanation: s cannot be split into 3 palindromes.

Constraints:

    3 <= s.length <= 2000
    s​​​​​​ consists only of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn check_partitioning(s: String) -> bool {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![false; n]; n];
        for (i, dp_i) in dp.iter_mut().enumerate() {
            dp_i[i] = true;
        }
        for i in 0..n - 1 {
            dp[i][i + 1] = s[i] == s[i + 1];
        }
        for i in (0..n - 2).rev() {
            for j in i + 2..n {
                dp[i][j] = s[i] == s[j] && dp[i + 1][j - 1];
            }
        }
        for i in 0..n - 2 {
            for j in i + 1..n - 1 {
                if dp[0][i] && dp[i + 1][j] && dp[j + 1][n - 1] {
                    return true;
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    assert_eq!(Solution::check_partitioning("abcbdd".to_string()), true);
    assert_eq!(Solution::check_partitioning("bcbddxy".to_string()), false);
}
