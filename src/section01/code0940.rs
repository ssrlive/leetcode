#![allow(dead_code)]

// 940. Distinct Subsequences II
// https://leetcode.com/problems/distinct-subsequences-ii/
// https://leetcode.cn/problems/distinct-subsequences-ii/
//
// Given a string s, return the number of distinct non-empty subsequences of s. Since the answer may be very large, return it modulo 109 + 7.
//
// A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without
// disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not.
//
// Example 1:
//
// Input: s = "abc"
// Output: 7
// Explanation: The 7 distinct subsequences are "a", "b", "c", "ab", "ac", "bc", and "abc".
//
// Example 2:
//
// Input: s = "aba"
// Output: 6
// Explanation: The 6 distinct subsequences are "a", "b", "ab", "aa", "ba", and "aba".
//
// Example 3:
//
// Input: s = "aaa"
// Output: 3
// Explanation: The 3 distinct subsequences are "a", "aa" and "aaa".
//
// Constraints:
//
// - 1 <= s.length <= 2000
// - s consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        let mut end = [0; 26];
        let mut res = 0;
        const MOD: i32 = 1_000_000_007;
        for c in s.chars() {
            let added = (res + 1 - end[(c as u8 - b'a') as usize]) % MOD;
            res = (res + added) % MOD;
            end[(c as u8 - b'a') as usize] = (end[(c as u8 - b'a') as usize] + added) % MOD;
        }
        (res + MOD) % MOD
    }
}

#[test]
fn test() {
    let s = "abc".to_string();
    assert_eq!(Solution::distinct_subseq_ii(s), 7);
    let s = "aba".to_string();
    assert_eq!(Solution::distinct_subseq_ii(s), 6);
    let s = "aaa".to_string();
    assert_eq!(Solution::distinct_subseq_ii(s), 3);
}
