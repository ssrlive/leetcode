#![allow(dead_code)]

/*

// 2116. Check if a Parentheses String Can Be Valid
// https://leetcode.com/problems/check-if-a-parentheses-string-can-be-valid/
// https://leetcode.cn/problems/check-if-a-parentheses-string-can-be-valid/
//
// Medium
//
// A parentheses string is a non-empty string consisting only of '(' and ')'. It is valid if any of the following conditions is true:

    It is ().
    It can be written as AB (A concatenated with B), where A and B are valid parentheses strings.
    It can be written as (A), where A is a valid parentheses string.

You are given a parentheses string s and a string locked, both of length n. locked is a binary string consisting only of '0's and '1's. For each index i of locked,

    If locked[i] is '1', you cannot change s[i].
    But if locked[i] is '0', you can change s[i] to either '(' or ')'.

Return true if you can make s a valid parentheses string. Otherwise, return false.

Example 1:

Input: s = "))()))", locked = "010100"
Output: true
Explanation: locked[1] == '1' and locked[3] == '1', so we cannot change s[1] or s[3].
We change s[0] and s[4] to '(' while leaving s[2] and s[5] unchanged to make s valid.

Example 2:

Input: s = "()()", locked = "0000"
Output: true
Explanation: We do not need to make any changes because s is already valid.

Example 3:

Input: s = ")", locked = "0"
Output: false
Explanation: locked permits us to change s[0].
Changing s[0] to either '(' or ')' will not make s valid.

Constraints:

    n == s.length == locked.length
    1 <= n <= 10^5
    s[i] is either '(' or ')'.
    locked[i] is either '0' or '1'.
*/

struct Solution;

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        fn scan(s: &str, locked: &str, ch1: char, ch2: char) -> bool {
            let s = if ch1 == '(' {
                s.chars().collect::<Vec<char>>()
            } else {
                s.chars().rev().collect::<Vec<char>>()
            };
            let locked = if ch1 == '(' {
                locked.chars().collect::<Vec<char>>()
            } else {
                locked.chars().rev().collect::<Vec<char>>()
            };
            let (mut balance, mut available) = (0i32, 0i32);
            for i in 0..s.len() {
                if locked[i] == '0' {
                    available += 1;
                    continue;
                }
                if s[i] == ch1 {
                    balance += 1;
                }
                if s[i] == ch2 {
                    balance -= 1;
                }
                if balance >= 0 {
                    continue;
                }
                if available == 0 {
                    return false;
                }
                available -= 1;
                balance += 1;
            }
            true
        }

        if !s.len().is_multiple_of(2) {
            return false;
        }
        scan(&s, &locked, '(', ')') && scan(&s, &locked, ')', '(')
    }
}

#[test]
fn test() {
    let cases = vec![
        (")()())", "010101", true),
        ("()()", "0000", true),
        (")", "0", false),
        ("((())", "00000", false),
    ];
    for (s, locked, expected) in cases {
        assert_eq!(Solution::can_be_valid(s.to_string(), locked.to_string()), expected);
    }
}
