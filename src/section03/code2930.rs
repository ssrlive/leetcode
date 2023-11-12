#![allow(dead_code)]

// 2930. Number of Strings Which Can Be Rearranged to Contain Substring
// https://leetcode.com/problems/number-of-strings-which-can-be-rearranged-to-contain-substring/
// https://leetcode.cn/problems/number-of-strings-which-can-be-rearranged-to-contain-substring/
//
// Medium
//
// You are given an integer n.
//
// A string s is called good if it contains only lowercase English characters and
// it is possible to rearrange the characters of s such that the new string contains "leet" as a substring.
//
// For example:
//
//     The string "lteer" is good because we can rearrange it to form "leetr" .
//     "letl" is not good because we cannot rearrange it to contain "leet" as a substring.
//
// Return the total number of good strings of length n.
//
// Since the answer may be large, return it modulo 109 + 7.
//
// A substring is a contiguous sequence of characters within a string.
//
// Example 1:
//
// Input: n = 4
// Output: 12
// Explanation: The 12 strings which can be rearranged to have "leet" as a substring are:
// "eelt", "eetl", "elet", "elte", "etel", "etle", "leet", "lete", "ltee", "teel", "tele", and "tlee".
//
// Example 2:
//
// Input: n = 10
// Output: 83943898
// Explanation: The number of strings with length 10 which can be rearranged to have "leet"
// as a substring is 526083947580. Hence the answer is 526083947580 % (109 + 7) = 83943898.
//
// Constraints:
//
//     1 <= n <= 10^5
//

struct Solution;

impl Solution {
    pub fn string_count(n: i32) -> i32 {
        let mut dp = vec![vec![-1; 16]; n as usize + 1];
        Self::func(n, 0, &mut dp)
    }

    fn func(n: i32, mask: i32, dp: &mut Vec<Vec<i32>>) -> i32 {
        const MOD: i32 = 1000000007;
        if n == 0 {
            if mask == 15 {
                return 1;
            }
            return 0;
        }
        if dp[n as usize][mask as usize] != -1 {
            return dp[n as usize][mask as usize];
        }
        let mut ans = 0;
        for i in 0..26 {
            if i == 4 {
                if mask & 1 != 0 {
                    ans = (ans + Self::func(n - 1, mask | 2, dp)) % MOD;
                } else {
                    ans = (ans + Self::func(n - 1, mask | 1, dp)) % MOD;
                }
            } else if i == 11 {
                ans = (ans + Self::func(n - 1, mask | 4, dp)) % MOD;
            } else if i == 19 {
                ans = (ans + Self::func(n - 1, mask | 8, dp)) % MOD;
            } else {
                ans = (ans + Self::func(n - 1, mask, dp)) % MOD;
            }
        }
        dp[n as usize][mask as usize] = ans;
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::string_count(4), 12);
    assert_eq!(Solution::string_count(10), 83943898);
}
