#![allow(dead_code)]

// 730. Count Different Palindromic Subsequences
// https://leetcode.com/problems/count-different-palindromic-subsequences/
// https://leetcode.cn/problems/count-different-palindromic-subsequences/
//
// Given a string s, return the number of different non-empty palindromic subsequences in s.
// Since the answer may be very large, return it modulo 10^9 + 7.
//
// A subsequence of a string is obtained by deleting zero or more characters from the string.
//
// A sequence is palindromic if it is equal to the sequence reversed.
//
// Two sequences a1, a2, ... and b1, b2, ... are different if there is some i for which a-i != b-i.
//
// Example 1:
//
// Input: s = "bccb"
// Output: 6
// Explanation: The 6 different non-empty palindromic subsequences are 'b', 'c', 'bb', 'cc', 'bcb', 'bccb'.
// Note that 'bcb' is counted only once, even though it occurs twice.
//
// Example 2:
//
// Input: s = "abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba"
// Output: 104860361
// Explanation: There are 3104860382 different non-empty palindromic subsequences, which is 104860361 modulo 109 + 7.
//
// Constraints:
//
// - 1 <= s.length <= 1000
// - s[i] is either 'a', 'b', 'c', or 'd'.
//

struct Solution;

impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![0; n]; n];
        for (i, item) in dp.iter_mut().enumerate().take(n) {
            item[i] = 1;
        }
        for i in (0..n).rev() {
            for j in i + 1..n {
                if s[i] == s[j] {
                    let mut left = i + 1;
                    let mut right = j - 1;
                    while left <= right && s[left] != s[i] {
                        left += 1;
                    }
                    while left <= right && s[right] != s[i] {
                        right -= 1;
                    }
                    match left.cmp(&right) {
                        std::cmp::Ordering::Less => {
                            dp[i][j] = 2 * dp[i + 1][j - 1] - dp[left + 1][right - 1];
                        }
                        std::cmp::Ordering::Equal => {
                            dp[i][j] = 2 * dp[i + 1][j - 1] + 1;
                        }
                        std::cmp::Ordering::Greater => {
                            dp[i][j] = 2 * dp[i + 1][j - 1] + 2;
                        }
                    }
                } else {
                    dp[i][j] = dp[i + 1][j] + dp[i][j - 1] - dp[i + 1][j - 1];
                }
                dp[i][j] = ((dp[i][j] as i64 + 1000000007_i64) % 1000000007_i64) as i32;
            }
        }
        dp[0][n - 1]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_palindromic_subsequences("bccb".to_string()), 6);
    assert_eq!(
        Solution::count_palindromic_subsequences(
            "abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba".to_string()
        ),
        104860361
    );
}
