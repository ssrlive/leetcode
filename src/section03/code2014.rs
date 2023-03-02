#![allow(dead_code)]

/*

// 2014. Longest Subsequence Repeated k Times
// https://leetcode.com/problems/longest-subsequence-repeated-k-times/
// https://leetcode.cn/problems/longest-subsequence-repeated-k-times/
//
// Hard
//
// You are given a string s of length n, and an integer k. You are tasked to find the longest subsequence repeated k times in string s.

A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.

A subsequence seq is repeated k times in the string s if seq * k is a subsequence of s, where seq * k represents a string constructed by concatenating seq k times.

    For example, "bba" is repeated 2 times in the string "bababcba", because the string "bbabba", constructed by concatenating "bba" 2 times, is a subsequence of the string "bababcba".

Return the longest subsequence repeated k times in string s. If multiple such subsequences are found, return the lexicographically largest one. If there is no such subsequence, return an empty string.

Example 1:
example 1

Input: s = "letsleetcode", k = 2
Output: "let"
Explanation: There are two longest subsequences repeated 2 times: "let" and "ete".
"let" is the lexicographically largest one.

Example 2:

Input: s = "bb", k = 2
Output: "b"
Explanation: The longest subsequence repeated 2 times is "b".

Example 3:

Input: s = "ab", k = 2
Output: ""
Explanation: There is no subsequence repeated 2 times. Empty string is returned.

Constraints:

    n == s.length
    2 <= n, k <= 2000
    2 <= n < k * 8
    s consists of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        fn check(s: &[u8], p: &str, k: i32) -> bool {
            let p = p.as_bytes();
            let mut i = 0;
            let mut j = 0;
            let mut k = k;
            while i < s.len() && k > 0 {
                j += (p[j] == s[i]) as usize;
                if j == p.len() {
                    k -= 1;
                    j = 0;
                }
                i += 1;
            }
            k == 0
        }

        fn generate(s: &[u8], chars: &str, cur: &str, best: &mut String, mask: i32, k: i32) {
            for (i, ch) in chars.bytes().enumerate() {
                if (mask & (1 << i)) == 0 {
                    let mut new_cur = cur.to_string();
                    new_cur.push(ch as char);
                    if check(s, &new_cur, k) {
                        if new_cur.len() > best.len() {
                            *best = new_cur.clone();
                        }
                        generate(s, chars, &new_cur, best, mask + (1 << i), k);
                    }
                }
            }
        }

        let s = s.as_bytes();
        let mut cnt = vec![0; 26];
        let mut chars = String::new();
        for &ch in s.iter() {
            cnt[(ch - b'a') as usize] += 1;
        }
        for i in (0..26).rev() {
            chars.push_str(&String::from_utf8(vec![b'a' + i as u8; cnt[i] / k as usize]).unwrap());
        }
        let mut best = String::new();
        generate(s, &chars, "", &mut best, 0, k);
        best
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("letsleetcode", 2, "let"),
        ("bb", 2, "b"),
        ("ab", 2, ""),
    ];
    for (s, k, expect) in cases {
        assert_eq!(Solution::longest_subsequence_repeated_k(s.to_string(), k), expect);
    }
}
