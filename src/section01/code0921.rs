#![allow(dead_code)]

// 921. Minimum Add to Make Parentheses Valid
// https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/
// https://leetcode.cn/problems/minimum-add-to-make-parentheses-valid/
//
// A parentheses string is valid if and only if:
//
// It is the empty string,
// It can be written as AB (A concatenated with B), where A and B are valid strings, or
// It can be written as (A), where A is a valid string.
// You are given a parentheses string s. In one move, you can insert a parenthesis at any position of the string.
//
// For example, if s = "()))", you can insert an opening parenthesis to be "(()))" or a closing parenthesis to be "())))".
// Return the minimum number of moves required to make s valid.
//
// Example 1:
//
// Input: s = "())"
// Output: 1
//
// Example 2:
//
// Input: s = "((("
// Output: 3
//
// Constraints:
//
// - 1 <= s.length <= 1000
// - s[i] is either '(' or ')'.
//

struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut moves = 0;
        let mut stack = vec![];
        for c in s.chars() {
            if c == '(' {
                stack.push(c);
            } else if c == ')' {
                if !stack.is_empty() {
                    stack.pop();
                } else {
                    moves += 1;
                }
            }
        }

        moves + stack.len() as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_add_to_make_valid("())".to_string()), 1);
    assert_eq!(Solution::min_add_to_make_valid("(((".to_string()), 3);
    assert_eq!(Solution::min_add_to_make_valid("()".to_string()), 0);
    assert_eq!(Solution::min_add_to_make_valid("()))((".to_string()), 4);
}
