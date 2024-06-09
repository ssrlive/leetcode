#![allow(dead_code)]

// 3174. Clear Digits
// https://leetcode.com/problems/clear-digits/
// https://leetcode.cn/problems/clear-digits/
//
// Easy
//
// You are given a string s.
//
// Your task is to remove all digits by doing this operation repeatedly:
//
//     Delete the first digit and the closest non-digit character to its left.
//
// Return the resulting string after removing all digits.
//
// Example 1:
//
// Input: s = "abc"
//
// Output: "abc"
//
// Explanation:
//
// There is no digit in the string.
//
// Example 2:
//
// Input: s = "cb34"
//
// Output: ""
//
// Explanation:
//
// First, we apply the operation on s[2], and s becomes "c4".
//
// Then we apply the operation on s[1], and s becomes "".
//
// Constraints:
//
//     1 <= s.length <= 100
//     s consists only of lowercase English letters and digits.
//     The input is generated such that it is possible to delete all digits.
//

struct Solution;

impl Solution {
    fn clear_digits(s: String) -> String {
        s.chars().fold(String::with_capacity(s.len()), |mut s, c| {
            if c.is_ascii_digit() {
                s.pop();
            } else {
                s.push(c);
            }
            s
        })
    }
}

#[test]
fn test() {
    let s = "abc".to_string();
    let res = "abc".to_string();
    assert_eq!(Solution::clear_digits(s), res);

    let s = "cb34".to_string();
    let res = "".to_string();
    assert_eq!(Solution::clear_digits(s), res);
}
