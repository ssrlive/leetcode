#![allow(dead_code)]

// 664. Strange Printer
// https://leetcode.com/problems/strange-printer/
// https://leetcode.cn/problems/strange-printer/
//
// There is a strange printer with the following two special properties:
//
// The printer can only print a sequence of the same character each time.
// At each turn, the printer can print new characters starting from and ending at any place and will cover the original existing characters.
// Given a string s, return the minimum number of turns the printer needed to print it.
//
// Example 1:
//
// Input: s = "aaabbb"
// Output: 2
// Explanation: Print "aaa" first and then print "bbb".
//
// Example 2:
//
// Input: s = "aba"
// Output: 2
// Explanation: Print "aaa" first and then print "b" from the second place of the string, which will cover the existing character 'a'.
//
// Constraints:
//
// - 1 <= s.length <= 100
// - s consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![0; n]; n];
        for i in (0..n).rev() {
            dp[i][i] = 1;
            for j in i + 1..n {
                if s[i] == s[j] {
                    dp[i][j] = dp[i][j - 1];
                } else {
                    let mut min_val = i32::MAX;
                    for k in i..j {
                        min_val = min_val.min(dp[i][k] + dp[k + 1][j]);
                    }
                    dp[i][j] = min_val;
                }
            }
        }
        dp[0][n - 1]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::strange_printer("aaabbb".to_string()), 2);
    assert_eq!(Solution::strange_printer("aba".to_string()), 2);
}
