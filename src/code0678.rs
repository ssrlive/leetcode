#![allow(dead_code)]

// 678. Valid Parenthesis String
// https://leetcode.com/problems/valid-parenthesis-string/
//
// Given a string s containing only three types of characters: '(', ')' and '*', return true if s is valid.
//
// The following rules define a valid string:
//
// - Any left parenthesis '(' must have a corresponding right parenthesis ')'.
// - Any right parenthesis ')' must have a corresponding left parenthesis '('.
// - Left parenthesis '(' must go before the corresponding right parenthesis ')'.
// - '*' could be treated as a single right parenthesis ')' or a single left parenthesis '(' or an empty string "".
//
// Example 1:
//
// Input: s = "()"
// Output: true
//
// Example 2:
//
// Input: s = "(*)"
// Output: true
//
// Example 3:
//
// Input: s = "(*))"
// Output: true
//
// Constraints:
//
// - 1 <= s.length <= 100
// - s[i] is '(', ')' or '*'.
//

struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut left = 0;
        let mut right = 0;
        for c in s.chars() {
            if c == '(' {
                left += 1;
                right += 1;
            } else if c == ')' {
                if left > 0 {
                    left -= 1;
                }
                right -= 1;
            } else {
                if left > 0 {
                    left -= 1;
                }
                right += 1;
            }
            if right < 0 {
                return false;
            }
        }
        left == 0
    }
}

#[test]
fn test() {
    assert!(Solution::check_valid_string("()".to_string()));
    assert!(Solution::check_valid_string("(*)".to_string()));
    assert!(Solution::check_valid_string("(*))".to_string()));
}
