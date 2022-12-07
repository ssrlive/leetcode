#![allow(dead_code)]

// 227. Basic Calculator II
// https://leetcode.com/problems/basic-calculator-ii/
//
// Given a string s which represents an expression, evaluate this expression and return its value.
//
// The integer division should truncate toward zero.
//
// You may assume that the given expression is always valid. All intermediate results
// will be in the range of [-231, 231 - 1].
//
// Note: You are not allowed to use any built-in function which evaluates strings
// as mathematical expressions, such as eval().
//
// Example 1:
//
// Input: "3+2*2"
// Output: 7
//
// Example 2:
//
// Input: " 3/2 "
// Output: 1
//
// Example 3:
//
// Input: " 3+5 / 2 "
// Output: 5
//
// Constraints:
//
// - 1 <= s.length <= 3 * 105
// - s consists of integers and operators ('+', '-', '*', '/') separated by some number of spaces.
// - s represents a valid expression.
// - All the integers in the expression are non-negative integers in the range [0, 231 - 1].
// - The answer is guaranteed to fit in a 32-bit integer.
//

struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        fn _calculate(s: String) -> Option<i32> {
            let mut stack = std::collections::VecDeque::new();
            let mut num = 0;
            let mut sign = '+';
            for (i, c) in s.chars().enumerate() {
                if c.is_ascii_digit() {
                    num = num * 10 + c.to_digit(10)? as i32;
                }
                if !c.is_ascii_digit() && c != ' ' || i == s.len() - 1 {
                    match sign {
                        '+' => stack.push_back(num),
                        '-' => stack.push_back(-num),
                        '*' => {
                            let n = stack.pop_back()?;
                            stack.push_back(n * num);
                        }
                        '/' => {
                            let n = stack.pop_back()?;
                            stack.push_back(n / num);
                        }
                        _ => (),
                    }
                    sign = c;
                    num = 0;
                }
            }
            Some(stack.iter().sum())
        }
        _calculate(s).unwrap_or(0)
    }
}

#[test]
fn test_calculate() {
    assert_eq!(Solution::calculate("3+2*2".to_string()), 7);
    assert_eq!(Solution::calculate(" 3/2 ".to_string()), 1);
    assert_eq!(Solution::calculate(" 3+5 / 2 ".to_string()), 5);
}
