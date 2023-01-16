#![allow(dead_code)]

// 20. Valid Parentheses
// https://leetcode.com/problems/valid-parentheses/
// https://leetcode.cn/problems/valid-parentheses/
//
// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
//
// An input string is valid if:
//
// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Every close bracket has a corresponding open bracket of the same type.
//
// Example 1:
//
// Input: s = "()"
// Output: true
//
// Example 2:
//
// Input: s = "()[]{}"
// Output: true
//
// Example 3:
//
// Input: s = "(]"
// Output: false
//
// Constraints:
//
// - 1 <= s.length <= 104
// - s consists of parentheses only '()[]{}'.
//

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::with_capacity(s.len());

        let enc = std::collections::HashMap::from([('(', ')'), ('{', '}'), ('[', ']')]);

        for c in s.chars() {
            if enc.contains_key(&c) {
                stack.push(c);
            } else {
                // A right-side enclosure as the only entry means "game over"
                if stack.is_empty() {
                    return false;
                }

                // This is should be the left side of the enclosure
                let left = stack.pop().unwrap();
                if enc[&left] != c {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_valid("()".to_string()), true);
    assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    assert_eq!(Solution::is_valid("(]".to_string()), false);
    assert_eq!(Solution::is_valid("([)]".to_string()), false);
    assert_eq!(Solution::is_valid("{[]}".to_string()), true);
}
