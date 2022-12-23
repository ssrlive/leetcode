#![allow(dead_code)]

// 32. Longest Valid Parentheses
// https://leetcode.com/problems/longest-valid-parentheses/
// https://leetcode.cn/problems/longest-valid-parentheses/
//
// Given a string containing just the characters '(' and ')', return the length of the longest valid (well-formed) parentheses substring.
//
// Example 1:
//
// Input: s = "(()"
// Output: 2
// Explanation: The longest valid parentheses substring is "()".
//
// Example 2:
//
// Input: s = ")()())"
// Output: 4
// Explanation: The longest valid parentheses substring is "()()".
//
// Example 3:
//
// Input: s = ""
// Output: 0
//
// Constraints:
//
// - 0 <= s.length <= 3 * 10^4
// - s[i] is '(', or ')'.
//

struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = vec![-1];
        let mut max_len = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                stack.push(i as i32);
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    max_len = max_len.max(i as i32 - stack[stack.len() - 1]);
                }
            }
        }
        max_len
    }
}

#[test]
fn test() {
    assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
    assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
    assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
}
