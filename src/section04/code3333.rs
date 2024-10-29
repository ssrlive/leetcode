#![allow(dead_code)]

// 3333. Find the Original Typed String II
// https://leetcode.com/problems/find-the-original-typed-string-ii/
// https://leetcode.cn/problems/find-the-original-typed-string-ii/
//
// Hard
//
// Alice is attempting to type a specific string on her computer. However, she tends to be clumsy and may press
// a key for too long, resulting in a character being typed multiple times.
//
// You are given a string word, which represents the final output displayed on Alice's screen.
// You are also given a positive integer k.
//
// Return the total number of possible original strings that Alice might have intended to type,
// if she was trying to type a string of size at least k.
//
// Since the answer may be very large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: word = "aabbccdd", k = 7
//
// Output: 5
//
// Explanation:
//
// The possible strings are: "aabbccdd", "aabbccd", "aabbcdd", "aabccdd", and "abbccdd".
//
// Example 2:
//
// Input: word = "aabbccdd", k = 8
//
// Output: 1
//
// Explanation:
//
// The only possible string is "aabbccdd".
//
// Example 3:
//
// Input: word = "aaabbb", k = 3
//
// Output: 8
//
// Constraints:
//
//    1 <= word.length <= 5 * 10^5
//    word consists only of lowercase English letters.
//    1 <= k <= 2000
//

struct Solution;

impl Solution {
    pub fn possible_string_count(word: String, k: i32) -> i32 {
        const M: i64 = 1_000_000_007;
        let n = word.len();
        let mut res = 1;
        let mut w = Vec::new();
        let mut i = 0;
        let word = word.as_bytes();
        while i < n {
            let mut ctr = 1;
            while i < n - 1 && word[i] == word[i + 1] {
                i += 1;
                ctr += 1;
            }
            res = (res * ctr as i64) % M;
            w.push(ctr);
            i += 1;
        }
        let k = k as usize;
        let mut dp = vec![1; k];
        let mut ndp = vec![0; k];
        for i in (0..std::cmp::min(w.len(), k + 2)).rev() {
            let mut pf = vec![0; k + 1];
            for j in 0..k {
                pf[j + 1] = (pf[j] + dp[j]) % M;
            }
            for rem in 0..k {
                let x = std::cmp::min(w[i], rem);
                ndp[rem] = (pf[rem] - pf[rem - x] + M) % M;
            }
            dp = ndp.clone();
        }
        res = (res - dp[k - 1] + M) % M;
        res as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::possible_string_count("aabbccdd".to_string(), 7), 5);
    assert_eq!(Solution::possible_string_count("aabbccdd".to_string(), 8), 1);
    assert_eq!(Solution::possible_string_count("aaabbb".to_string(), 3), 8);
}
