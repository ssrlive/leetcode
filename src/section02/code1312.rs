#![allow(dead_code)]

// 1312. Minimum Insertion Steps to Make a String Palindrome
// https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/
// https://leetcode.cn/problems/minimum-insertion-steps-to-make-a-string-palindrome/
//
// Hard
//
// Given a string s. In one step you can insert any character at any index of the string.
//
// Return the minimum number of steps to make s palindrome.
//
// A Palindrome String is one that reads the same backward as well as forward.
//
// Example 1:
//
// Input: s = "zzazz"
// Output: 0
// Explanation: The string "zzazz" is already palindrome we do not need any insertions.
//
// Example 2:
//
// Input: s = "mbadm"
// Output: 2
// Explanation: String can be "mbdadbm" or "mdbabdm".
//
// Example 3:
//
// Input: s = "leetcode"
// Output: 5
// Explanation: Inserting 5 characters the string becomes "leetcodocteel".
//
// Constraints:
//
// -    1 <= s.length <= 500
// -    s consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![0; n]; n];
        for i in (0..n).rev() {
            for j in i + 1..n {
                if s[i] == s[j] {
                    dp[i][j] = dp[i + 1][j - 1];
                } else {
                    dp[i][j] = 1 + dp[i + 1][j].min(dp[i][j - 1]);
                }
            }
        }
        dp[0][n - 1]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_insertions("zzazz".to_string()), 0);
    assert_eq!(Solution::min_insertions("mbadm".to_string()), 2);
    assert_eq!(Solution::min_insertions("leetcode".to_string()), 5);
}
