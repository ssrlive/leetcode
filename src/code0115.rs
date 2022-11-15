#![allow(dead_code)]

// 115. Distinct Subsequences
// https://leetcode.com/problems/distinct-subsequences/

struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
        for item in dp.iter_mut().take(s.len() + 1) {
            item[0] = 1;
        }
        for i in 1..=s.len() {
            for j in 1..=t.len() {
                dp[i][j] = dp[i - 1][j];
                if s[i - 1] == t[j - 1] {
                    dp[i][j] += dp[i - 1][j - 1];
                }
            }
        }
        dp[s.len()][t.len()]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()), 3);
    assert_eq!(Solution::num_distinct("babgbag".to_string(), "bag".to_string()), 5);
}
