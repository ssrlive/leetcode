#![allow(dead_code)]

// 856. Score of Parentheses
// https://leetcode.com/problems/score-of-parentheses/
// https://leetcode.cn/problems/score-of-parentheses/
//
// Given a balanced parentheses string s, return the score of the string.
//
// The score of a balanced parentheses string is based on the following rule:
//
// "()" has score 1.
// AB has score A + B, where A and B are balanced parentheses strings.
// (A) has score 2 * A, where A is a balanced parentheses string.
//
// Example 1:
//
// Input: s = "()"
// Output: 1
//
// Example 2:
//
// Input: s = "(())"
// Output: 2
//
// Example 3:
//
// Input: s = "()()"
// Output: 2
//
// Constraints:
//
// - 2 <= s.length <= 50
// - s consists of only '(' and ')'.
// - s is a balanced parentheses string.
//

struct Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut stack = vec![0];
        for ch in s.chars() {
            match ch {
                '(' => stack.push(0),
                _ => {
                    let v = stack.pop().unwrap();
                    *stack.last_mut().unwrap() += i32::max(2 * v, 1)
                }
            }
        }
        stack.pop().unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::score_of_parentheses("()".to_string()), 1);
    assert_eq!(Solution::score_of_parentheses("(())".to_string()), 2);
    assert_eq!(Solution::score_of_parentheses("()()".to_string()), 2);
    assert_eq!(Solution::score_of_parentheses("(()(()))".to_string()), 6);
}
