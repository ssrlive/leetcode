#![allow(dead_code)]

// 65. Valid Number
// https://leetcode.com/problems/valid-number/
// https://leetcode.cn/problems/valid-number/
//
// A valid number can be split up into these components (in order):
//
// A decimal number or an integer.
// (Optional) An 'e' or 'E', followed by an integer.
// A decimal number can be split up into these components (in order):
//
// (Optional) A sign character (either '+' or '-').
// One of the following formats:
// One or more digits, followed by a dot '.'.
// One or more digits, followed by a dot '.', followed by one or more digits.
// A dot '.', followed by one or more digits.
// An integer can be split up into these components (in order):
//
// (Optional) A sign character (either '+' or '-').
// One or more digits.
// For example, all the following are valid numbers: ["2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"],
// while the following are not valid numbers: ["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"].
//
// Given a string s, return true if s is a valid number.
//
// Example 1:
//
// Input: s = "0"
// Output: true
//
// Example 2:
//
// Input: s = "e"
// Output: false
//
// Example 3:
//
// Input: s = "."
// Output: false
//
// Constraints:
//
// - 1 <= s.length <= 20
// - s consists of only English letters (both uppercase and lowercase), digits (0-9), plus '+', minus '-', or dot '.'.
//

struct Solution;

impl Solution {
    pub fn is_number(s: String) -> bool {
        if let Some(e) = s.chars().position(|c| c == 'e' || c == 'E') {
            Self::is_decimal(&s[0..e]) && Self::is_integer(&s[e + 1..])
        } else {
            Self::is_decimal(&s)
        }
    }
    fn is_decimal(s: &str) -> bool {
        let (mut num, mut dot) = (false, false);
        for (i, c) in s.chars().enumerate() {
            match c {
                '0'..='9' => num = true,
                '+' | '-' if i == 0 => {}
                '.' if !dot => dot = true,
                _ => return false,
            }
        }
        num
    }
    fn is_integer(s: &str) -> bool {
        let mut num = false;
        for (i, c) in s.chars().enumerate() {
            match c {
                '0'..='9' => num = true,
                '+' | '-' if i == 0 => {}
                _ => return false,
            }
        }
        num
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_number("0".to_string()), true);
    assert_eq!(Solution::is_number("e".to_string()), false);
    assert_eq!(Solution::is_number(".".to_string()), false);
    assert_eq!(Solution::is_number(".".to_string()), false);
    assert_eq!(Solution::is_number("2".to_string()), true);
    assert_eq!(Solution::is_number("0089".to_string()), true);
    assert_eq!(Solution::is_number("-0.1".to_string()), true);
    assert_eq!(Solution::is_number("+3.14".to_string()), true);
    assert_eq!(Solution::is_number("4.".to_string()), true);
    assert_eq!(Solution::is_number("-.9".to_string()), true);
    assert_eq!(Solution::is_number("2e10".to_string()), true);
    assert_eq!(Solution::is_number("-90E3".to_string()), true);
    assert_eq!(Solution::is_number("3e+7".to_string()), true);
    assert_eq!(Solution::is_number("+6e-1".to_string()), true);
    assert_eq!(Solution::is_number("53.5e93".to_string()), true);
    assert_eq!(Solution::is_number("-123.456e789".to_string()), true);
    assert_eq!(Solution::is_number("abc".to_string()), false);
    assert_eq!(Solution::is_number("1a".to_string()), false);
    assert_eq!(Solution::is_number("1e".to_string()), false);
    assert_eq!(Solution::is_number("e3".to_string()), false);
    assert_eq!(Solution::is_number("99e2.5".to_string()), false);
    assert_eq!(Solution::is_number("--6".to_string()), false);
    assert_eq!(Solution::is_number("-+3".to_string()), false);
    assert_eq!(Solution::is_number("95a54e53".to_string()), false);
}
