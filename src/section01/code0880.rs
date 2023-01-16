#![allow(dead_code)]

// 880. Decoded String at Index
// https://leetcode.com/problems/decoded-string-at-index/
// https://leetcode.cn/problems/decoded-string-at-index/
//
// You are given an encoded string s. To decode the string to a tape, the encoded string is read one character at a time and the following steps are taken:
//
// If the character read is a letter, that letter is written onto the tape.
// If the character read is a digit d, the entire current tape is repeatedly written d - 1 more times in total.
//
// Given an integer k, return the kth letter (1-indexed) in the decoded string.
//
// Example 1:
//
// Input: s = "leet2code3", k = 10
// Output: "o"
// Explanation: The decoded string is "leetleetcodeleetleetcodeleetleetcode".
// The 10th letter in the string is "o".
//
// Example 2:
//
// Input: s = "ha22", k = 5
// Output: "h"
// Explanation: The decoded string is "hahahaha".
// The 5th letter is "h".
//
// Example 3:
//
// Input: s = "a2345678999999999999999", k = 1
// Output: "a"
// Explanation: The decoded string is "a" repeated 8301530446056247680 times.
// The 1st letter is "a".
//
// Constraints:
//
// - 2 <= s.length <= 100
// - s consists of lowercase English letters and digits 2 through 9.
// - s starts with a letter.
// - 1 <= k <= 10^9
// - It is guaranteed that k is less than or equal to the length of the decoded string.
// The decoded string is guaranteed to have less than 263 letters.
//

struct Solution;

impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let len = s.len();
        let mut i = 0;
        let mut stk = vec![];
        let mut left = 1;
        let s = s.as_bytes();
        let mut k = k as usize;
        while i < len && left <= k {
            let mut part = vec![];
            while i < len && s[i].is_ascii_lowercase() {
                part.push(s[i]);
                i += 1;
            }
            let right = left + part.len() - 1;
            stk.push((part, left, right));
            let mut mul = 1;
            while i < len && s[i].is_ascii_digit() {
                mul *= usize::from(s[i] - b'0');
                i += 1;
            }
            left = right * mul + 1;
            stk.push((vec![], right, left - 1));
        }
        for (part, start, end) in stk.iter().rev() {
            if part.is_empty() {
                k %= start;
                k = if k == 0 { *start } else { k };
            } else if k >= *start && k <= *end {
                return String::from_utf8(part[(k - start)..=(k - start)].to_vec()).unwrap();
            }
        }
        String::new()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::decode_at_index(String::from("leet2code3"), 10),
        String::from("o")
    );
    assert_eq!(Solution::decode_at_index(String::from("ha22"), 5), String::from("h"));
    assert_eq!(
        Solution::decode_at_index(String::from("a2345678999999999999999"), 1),
        String::from("a")
    );
}
