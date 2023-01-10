#![allow(dead_code)]

// 1021. Remove Outermost Parentheses
// https://leetcode.com/problems/remove-outermost-parentheses/
// https://leetcode.cn/problems/remove-outermost-parentheses/
//
// A valid parentheses string is either empty "", "(" + A + ")", or A + B, where A and B are valid parentheses strings, and + represents string concatenation.
//
// For example, "", "()", "(())()", and "(()(()))" are all valid parentheses strings.
// A valid parentheses string s is primitive if it is nonempty, and there does not exist a way to split it into s = A + B, with A and B nonempty valid parentheses strings.
//
// Given a valid parentheses string s, consider its primitive decomposition: s = P1 + P2 + ... + Pk, where Pi are primitive valid parentheses strings.
//
// Return s after removing the outermost parentheses of every primitive string in the primitive decomposition of s.
//
// Example 1:
//
// Input: s = "(()())(())"
// Output: "()()()"
// Explanation:
// The input string is "(()())(())", with primitive decomposition "(()())" + "(())".
// After removing outer parentheses of each part, this is "()()" + "()" = "()()()".
//
// Example 2:
//
// Input: s = "(()())(())(()(()))"
// Output: "()()()()(())"
// Explanation:
// The input string is "(()())(())(()(()))", with primitive decomposition "(()())" + "(())" + "(()(()))".
// After removing outer parentheses of each part, this is "()()" + "()" + "()(())" = "()()()()(())".
//
// Example 3:
//
// Input: s = "()()"
// Output: ""
// Explanation:
// The input string is "()()", with primitive decomposition "()" + "()".
// After removing outer parentheses of each part, this is "" + "" = "".
//
// Constraints:
//
// - 1 <= s.length <= 10^5
// - s[i] is either '(' or ')'.
// - s is a valid parentheses string.
//

struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut res = String::with_capacity(s.len());
        let mut counter = 0;

        for c in s.chars() {
            let prev_counter = counter;
            counter += if c == '(' { 1 } else { -1 };

            if prev_counter | counter != 1 {
                // not 0 -> 1, 1 -> 0
                res.push(c);
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        ("(()())(())", "()()()"),
        ("(()())(())(()(()))", "()()()()(())"),
        ("()()", ""),
    ];
    for (s, expected) in cases {
        assert_eq!(Solution::remove_outer_parentheses(s.to_string()), expected);
    }
}
