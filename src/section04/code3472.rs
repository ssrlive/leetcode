#![allow(dead_code)]

// 3472. Longest Palindromic Subsequence After at Most K Operations
// https://leetcode.com/problems/longest-palindromic-subsequence-after-at-most-k-operations/
// https://leetcode.cn/problems/longest-palindromic-subsequence-after-at-most-k-operations/
//
// Medium
//
// You are given a string s and an integer k.
//
// In one operation, you can replace the character at any position with the next or previous letter in the alphabet (wrapping around so that 'a' is after 'z'). For example, replacing 'a' with the next letter results in 'b', and replacing 'a' with the previous letter results in 'z'. Similarly, replacing 'z' with the next letter results in 'a', and replacing 'z' with the previous letter results in 'y'.
//
// Return the length of the longest palindromic subsequence of s that can be obtained after performing at most k operations.
//
// Example 1:
//
// Input: s = "abced", k = 2
//
// Output: 3
//
// Explanation:
//
// Replace s[1] with the next letter, and s becomes "acced".
// Replace s[4] with the previous letter, and s becomes "accec".
// The subsequence "ccc" forms a palindrome of length 3, which is the maximum.
//
// Example 2:
//
// Input: s = "aaazzz", k = 4
//
// Output: 6
//
// Explanation:
//
// Replace s[0] with the previous letter, and s becomes "zaazzz".
// Replace s[4] with the next letter, and s becomes "zaazaz".
// Replace s[3] with the next letter, and s becomes "zaaaaz".
// The entire string forms a palindrome of length 6.
//
// Constraints:
//
// 1 <= s.length <= 200
// 1 <= k <= 200
// s consists of only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn longest_palindromic_subsequence(s: String, k: i32) -> i32 {
        fn solve(i: usize, j: usize, k: i32, s: &[u8], dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
            if i > j {
                return 0;
            }
            if i == j {
                return 1;
            }

            if dp[i][j][k as usize] != -1 {
                return dp[i][j][k as usize];
            }

            let mut res = solve(i + 1, j, k, s, dp);
            res = res.max(solve(i, j - 1, k, s, dp));

            let cost = (s[i] as i32 - s[j] as i32).abs().min(26 - (s[i] as i32 - s[j] as i32).abs());
            if k >= cost {
                res = res.max(2 + solve(i + 1, j - 1, k - cost, s, dp));
            }

            dp[i][j][k as usize] = res;
            res
        }

        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![vec![-1; k as usize + 1]; n]; n];
        solve(0, n - 1, k, s, &mut dp)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::longest_palindromic_subsequence("abced".to_string(), 2), 3);
    assert_eq!(Solution::longest_palindromic_subsequence("aaazzz".to_string(), 4), 6);
}
