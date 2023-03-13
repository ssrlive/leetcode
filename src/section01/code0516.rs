#![allow(dead_code)]

// 516. Longest Palindromic Subsequence
// https://leetcode.com/problems/longest-palindromic-subsequence/
// https://leetcode.cn/problems/longest-palindromic-subsequence/
//
// Given a string s, find the longest palindromic subsequence's length in s.
//
// A subsequence is a sequence that can be derived from another sequence by deleting some or no elements without changing the order of the remaining elements.
//
// Example 1:
//
// Input: s = "bbbab"
// Output: 4
// Explanation: One possible longest palindromic subsequence is "bbbb".
//
// Example 2:
//
// Input: s = "cbbd"
// Output: 2
// Explanation: One possible longest palindromic subsequence is "bb".
//
// Constraints:
//
// - 1 <= s.length <= 1000
// - s consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![0; n]; n];
        for (i, item) in dp.iter_mut().enumerate() {
            item[i] = 1;
        }
        for i in (0..n).rev() {
            for j in i + 1..n {
                if s[i] == s[j] {
                    dp[i][j] = dp[i + 1][j - 1] + 2;
                } else {
                    dp[i][j] = std::cmp::max(dp[i + 1][j], dp[i][j - 1]);
                }
            }
        }
        dp[0][n - 1]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::longest_palindrome_subseq("bbbab".to_string()), 4);
    assert_eq!(Solution::longest_palindrome_subseq("cbbd".to_string()), 2);
}
