#![allow(dead_code)]

// 761. Special Binary String
// https://leetcode.com/problems/special-binary-string/
// https://leetcode.cn/problems/special-binary-string/
//
// Special binary strings are binary strings with the following two properties:
//
// - The number of 0's is equal to the number of 1's.
// - Every prefix of the binary string has at least as many 1's as 0's.
//
// You are given a special binary string s.
//
// A move consists of choosing two consecutive, non-empty, special substrings of s, and swapping them. Two strings are consecutive
// if the last character of the first string is exactly one index before the first character of the second string.
//
// Return the lexicographically largest resulting string possible after applying the mentioned operations on the string.
//
// Example 1:
//
// Input: s = "11011000"
// Output: "11100100"
// Explanation: The strings "10" [occuring at s[1]] and "1100" [at s[3]] are swapped.
// This is the lexicographically largest string possible after some number of swaps.
//
// Example 2:
//
// Input: s = "10"
// Output: "10"
//
// Constraints:
//
// - 1 <= s.length <= 50
// - s[i] is either '0' or '1'.
// - s is a special binary string.
//

struct Solution;

impl Solution {
    pub fn make_largest_special(s: String) -> String {
        let mut count = 0;
        let mut i = 0;
        let mut res = vec![];

        for (j, c) in s.char_indices() {
            if c == '1' {
                count += 1;
            } else {
                count -= 1;
            }

            if count == 0 {
                res.push(format!("1{}0", Solution::make_largest_special(s[i + 1..j].to_string())));
                i = j + 1;
            }
        }

        res.sort_by(|a, b| b.cmp(a));
        res.join("")
    }
}

#[test]
fn test() {
    assert_eq!(Solution::make_largest_special("11011000".to_string()), "11100100".to_string());
    assert_eq!(Solution::make_largest_special("10".to_string()), "10".to_string());
}
