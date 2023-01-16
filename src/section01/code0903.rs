#![allow(dead_code)]

// 903. Valid Permutations for DI Sequence
// https://leetcode.com/problems/valid-permutations-for-di-sequence/
// https://leetcode.cn/problems/valid-permutations-for-di-sequence/
//
// You are given a string s of length n where s[i] is either:
//
// 'D' means decreasing, or
// 'I' means increasing.
// A permutation perm of n + 1 integers of all the integers in the range [0, n] is called a valid permutation if for all valid i:
//
// If s[i] == 'D', then perm[i] > perm[i + 1], and
// If s[i] == 'I', then perm[i] < perm[i + 1].
//
// Return the number of valid permutations perm. Since the answer may be large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: s = "DID"
// Output: 5
// Explanation: The 5 valid permutations of (0, 1, 2, 3) are:
// (1, 0, 3, 2)
// (2, 0, 3, 1)
// (2, 1, 3, 0)
// (3, 0, 2, 1)
// (3, 1, 2, 0)
//
// Example 2:
//
// Input: s = "D"
// Output: 1
//
// Constraints:
//
// - n == s.length
// - 1 <= n <= 200
// - s[i] is either 'I' or 'D'.
//

struct Solution;

impl Solution {
    pub fn num_perms_di_sequence(s: String) -> i32 {
        let n = s.len();
        let m = 1_000_000_007;
        let mut dp = vec![vec![0; n + 1]; n + 1];
        dp[0][0] = 1;
        for i in 1..=n {
            for j in 0..=i {
                if s.chars().nth(i - 1).unwrap() == 'D' {
                    for k in j..=i - 1 {
                        dp[i][j] = dp[i][j] % m + dp[i - 1][k] % m;
                    }
                } else if j > 0 {
                    for k in 0..=j - 1 {
                        dp[i][j] = dp[i][j] % m + dp[i - 1][k] % m;
                    }
                }
            }
        }
        let mut res = 0;
        for i in 0..=n {
            res = res % m + dp[n][i] % m;
        }
        (res % m) as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_perms_di_sequence("DID".to_string()), 5);
    assert_eq!(Solution::num_perms_di_sequence("D".to_string()), 1);

    let s = "IIIIIDIIIIIIDIDIDDDIIDIDDIDDIDDDDDIDDDDDIDIIDDIDDI".to_string();
    assert_eq!(Solution::num_perms_di_sequence(s), 118230849);
}
