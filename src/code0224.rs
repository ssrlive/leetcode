#![allow(dead_code)]

// 224. Basic Calculator
// https://leetcode.com/problems/basic-calculator/
// https://leetcode.cn/problems/basic-calculator/
//
// Given a string s representing a valid expression, implement a basic calculator to evaluate it,
// and return the result of the evaluation.
//
// Note: You are not allowed to use any built-in function which evaluates strings
// as mathematical expressions, such as eval().
//
// Example 1:
// Input: s = "1 + 1"
// Output: 2
//
// Example 2:
// Input: s = " 2-1 + 2 "
// Output: 3
//
// Example 3:
// Input: s = "(1+(4+5+2)-3)+(6+8)"
// Output: 23
//
// Constraints:
// 1 <= s.length <= 3 * 10^5
// s consists of digits, '+', '-', '(', ')', and ' '.
// s represents a valid expression.
// '+' is not used as a unary operation.
// '-' could be used as a unary operation but it has to be inside parentheses.
// There will be no two consecutive operators in the input.
// Every number and running calculation will fit in a signed 32-bit integer.
//

struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        fn _calculate(s: &str) -> Option<i32> {
            let mut stack = std::collections::VecDeque::new();
            let mut num = 0;
            let mut sign = 1;
            let mut result = 0;

            for c in s.chars() {
                if c.is_ascii_digit() {
                    num = num * 10 + c.to_digit(10)? as i32;
                } else if c == '+' {
                    result += sign * num;
                    num = 0;
                    sign = 1;
                } else if c == '-' {
                    result += sign * num;
                    num = 0;
                    sign = -1;
                } else if c == '(' {
                    stack.push_back(result);
                    stack.push_back(sign);
                    result = 0;
                    sign = 1;
                } else if c == ')' {
                    result += sign * num;
                    num = 0;
                    result *= stack.pop_back()?;
                    result += stack.pop_back()?;
                }
            }

            if num != 0 {
                result += sign * num;
            }

            Some(result)
        }
        _calculate(&s).unwrap_or_default()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::calculate("1 + 1".to_string()), 2);
    assert_eq!(Solution::calculate(" 2-1 + 2 ".to_string()), 3);
    assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string()), 23);
}
