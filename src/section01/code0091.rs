#![allow(dead_code)]

// 91. Decode Ways
// https://leetcode.com/problems/decode-ways/
// https://leetcode.cn/problems/decode-ways/
//
// A message containing letters from A-Z can be encoded into numbers using the following mapping:
//
// 'A' -> "1"
// 'B' -> "2"
// ...
// 'Z' -> "26"
// To decode an encoded message, all the digits must be grouped then mapped back into letters using the reverse of
// the mapping above (there may be multiple ways). For example, "11106" can be mapped into:
//
// "AAJF" with the grouping (1 1 10 6)
// "KJF" with the grouping (11 10 6)
// Note that the grouping (1 11 06) is invalid because "06" cannot be mapped into 'F' since "6" is different from "06".
//
// Given a string s containing only digits, return the number of ways to decode it.
//
// The test cases are generated so that the answer fits in a 32-bit integer.
//
// Example 1:
//
// Input: s = "12"
// Output: 2
// Explanation: "12" could be decoded as "AB" (1 2) or "L" (12).
//
// Example 2:
//
// Input: s = "226"
// Output: 3
// Explanation: "226" could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).
//
// Example 3:
//
// Input: s = "06"
// Output: 0
// Explanation: "06" cannot be mapped to "F" because of the leading zero ("6" is different from "06").
//
// Constraints:
//
// - 1 <= s.length <= 100
// - s contains only digits and may contain leading zero(s).
//

struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        fn is_valid_code(code: u8) -> bool {
            (1..=26).contains(&code)
        }

        let mut prev_prev = 0;
        let mut prev = 0;
        let mut prev_digit = 0;
        // because we deal with digits (0-9) they are encoded by one byte
        // - so it's ok to use bytes iterator
        for digit in s.bytes().map(|b| b - b'0') {
            let mut num = 0;

            if prev_digit != 0 && is_valid_code(prev_digit * 10 + digit) {
                num += prev_prev.max(1);
            }

            if is_valid_code(digit) {
                num += prev.max(1);
            }

            if num == 0 {
                return 0;
            }

            prev_prev = prev;
            prev = num;
            prev_digit = digit;
        }
        prev
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_decodings("12".to_string()), 2);
    assert_eq!(Solution::num_decodings("226".to_string()), 3);
    assert_eq!(Solution::num_decodings("06".to_string()), 0);
    assert_eq!(Solution::num_decodings("0".to_string()), 0);
    assert_eq!(Solution::num_decodings("1".to_string()), 1);
    assert_eq!(Solution::num_decodings("10".to_string()), 1);
    assert_eq!(Solution::num_decodings("27".to_string()), 1);
    assert_eq!(Solution::num_decodings("100".to_string()), 0);
    assert_eq!(Solution::num_decodings("101".to_string()), 1);
    assert_eq!(Solution::num_decodings("110".to_string()), 1);
    assert_eq!(Solution::num_decodings("111".to_string()), 3);
    assert_eq!(Solution::num_decodings("1111".to_string()), 5);
}
