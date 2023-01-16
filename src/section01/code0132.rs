#![allow(dead_code)]

// 132. Palindrome Partitioning II
// https://leetcode.com/problems/palindrome-partitioning-ii/
// https://leetcode.cn/problems/palindrome-partitioning-ii/
//
// Given a string s, partition s such that every substring of the partition is a palindrome.
//
// Return the minimum cuts needed for a palindrome partitioning of s.
//

struct Solution;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s = s.as_bytes();
        let mut p = vec![vec![false; s.len()]; s.len()];
        let mut dp = vec![0; s.len()];
        for i in 0..s.len() {
            let mut min = i as i32;
            for j in 0..=i {
                if (j + 1 >= i || p[j + 1][i - 1]) && s[j] == s[i] {
                    p[j][i] = true;
                    min = if j == 0 { 0 } else { min.min(dp[j - 1] + 1) };
                }
            }
            dp[i] = min;
        }
        dp[s.len() - 1]
    }
}

#[test]
fn test_min_cut() {
    assert_eq!(Solution::min_cut("aab".to_string()), 1);
    assert_eq!(Solution::min_cut("a".to_string()), 0);
    assert_eq!(Solution::min_cut("ab".to_string()), 1);
    assert_eq!(Solution::min_cut("aa".to_string()), 0);
    assert_eq!(
        Solution::min_cut("ababababababababababababcbabababababababababababa".to_string()),
        0
    );
}
