#![allow(dead_code)]

// 1249. Minimum Remove to Make Valid Parentheses
// https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/
// https://leetcode.cn/problems/minimum-remove-to-make-valid-parentheses/
//
// Medium
// Given a string s of '(' , ')' and lowercase English characters.
//
// Your task is to remove the minimum number of parentheses ( '(' or ')', in any positions ) so that the resulting parentheses string is valid and return any valid string.
//
// Formally, a parentheses string is valid if and only if:
//
//     It is the empty string, contains only lowercase characters, or
//     It can be written as AB (A concatenated with B), where A and B are valid strings, or
//     It can be written as (A), where A is a valid string.
//
// Example 1:
//
// Input: s = "lee(t(c)o)de)"
// Output: "lee(t(c)o)de"
// Explanation: "lee(t(co)de)" , "lee(t(c)ode)" would also be accepted.
//
// Example 2:
//
// Input: s = "a)b(c)d"
// Output: "ab(c)d"
//
// Example 3:
//
// Input: s = "))(("
// Output: ""
// Explanation: An empty string is also valid.
//
// Constraints:
//
// -     1 <= s.length <= 10^5
// -     s[i] is either'(' , ')', or lowercase English letter.
//

struct Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut stack = vec![];
        let mut invalid = vec![false; s.len()];
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                stack.push(i);
            } else if c == ')' {
                if let Some(j) = stack.pop() {
                    invalid[j] = false;
                    invalid[i] = false;
                } else {
                    invalid[i] = true;
                }
            }
        }
        for i in stack {
            invalid[i] = true;
        }
        s.chars()
            .enumerate()
            .filter(|(i, _)| !invalid[*i])
            .map(|(_, c)| c)
            .collect()
    }
}

#[test]
fn test() {
    let cases = vec![
        ("lee(t(c)o)de)", "lee(t(c)o)de"),
        ("a)b(c)d", "ab(c)d"),
        ("))((", ""),
        ("(a(b(c)d)", "a(b(c)d)"),
    ];
    for (s, expect) in cases {
        assert_eq!(Solution::min_remove_to_make_valid(s.to_string()), expect);
    }
}
