#![allow(dead_code)]

// 1016. Binary String With Substrings Representing 1 To N
// https://leetcode.com/problems/binary-string-with-substrings-representing-1-to-n/
// https://leetcode.cn/problems/binary-string-with-substrings-representing-1-to-n/
//
// Given a binary string s and a positive integer n, return true if the binary representation of all the integers in the range [1, n] are substrings of s, or false otherwise.
//
// A substring is a contiguous sequence of characters within a string.
//
// Example 1:
//
// Input: s = "0110", n = 3
// Output: true
//
// Example 2:
//
// Input: s = "0110", n = 4
// Output: false
//
// Constraints:
//
// - 1 <= s.length <= 1000
// - s[i] is either '0' or '1'.
// - 1 <= n <= 10^9
//

struct Solution;

impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        let n = n as usize;
        let limit = 1_000_010;
        if limit < n {
            return false;
        }

        let mut memo = vec![false; n + 1];
        let mut s = s.chars().collect::<Vec<char>>();
        s.reverse();
        let m = s.len();
        for i in 0..m {
            let mut temp = 0;
            for j in 0..=40 {
                let ni = i + j;
                if m <= ni {
                    break;
                }
                if s[ni] == '1' {
                    temp += 1 << j;
                }
                if n < temp {
                    break;
                }
                memo[temp] = true;
            }
        }

        memo[0] = true;
        for v in memo {
            if !v {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let cases = vec![
        ("0110", 3, true),
        ("0110", 4, false),
        ("0110", 5, false),
        ("0110", 6, false),
    ];
    for (s, n, expected) in cases {
        assert_eq!(Solution::query_string(s.to_string(), n), expected);
    }
}
