#![allow(dead_code)]

// 1106. Parsing A Boolean Expression
// https://leetcode.com/problems/parsing-a-boolean-expression/
// https://leetcode.cn/problems/parsing-a-boolean-expression/
//
// A boolean expression is an expression that evaluates to either true or false. It can be in one of the following shapes:
//
// 't' that evaluates to true.
// 'f' that evaluates to false.
// '!(subExpr)' that evaluates to the logical NOT of the inner expression subExpr.
// '&(subExpr1, subExpr2, ..., subExprn)' that evaluates to the logical AND of the inner expressions subExpr1, subExpr2, ..., subExprn where n >= 1.
// '|(subExpr1, subExpr2, ..., subExprn)' that evaluates to the logical OR of the inner expressions subExpr1, subExpr2, ..., subExprn where n >= 1.
//
// Given a string expression that represents a boolean expression, return the evaluation of that expression.
//
// It is guaranteed that the given expression is valid and follows the given rules.
//
// Example 1:
//
// Input: expression = "&(|(f))"
// Output: false
// Explanation:
// First, evaluate |(f) --> f. The expression is now "&(f)".
// Then, evaluate &(f) --> f. The expression is now "f".
// Finally, return false.
//
// Example 2:
//
// Input: expression = "|(f,f,f,t)"
// Output: true
// Explanation: The evaluation of (false OR false OR false OR true) is true.
//
// Example 3:
//
// Input: expression = "!(&(f,t))"
// Output: true
// Explanation:
// First, evaluate &(f,t) --> (false AND true) --> false --> f. The expression is now "!(f)".
// Then, evaluate !(f) --> NOT false --> true. We return true.
//
// Constraints:
//
// - 1 <= expression.length <= 2 * 10^4
// - expression[i] is one following characters: '(', ')', '&', '|', '!', 't', 'f', and ','.
//

struct Solution;

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        if expression == "t" {
            return true;
        }
        if expression == "f" {
            return false;
        }
        let mut stack = vec![];
        let mut paren_count = 0;
        let mut short_circuit = i32::MAX;
        let mut value = false;

        for c in expression.chars() {
            if c == '(' {
                paren_count += 1;
            }
            if c == ')' {
                paren_count -= 1;
            }

            if paren_count >= short_circuit {
                continue;
            }
            short_circuit = i32::MAX;

            match c {
                '!' | '&' | '|' => {
                    stack.push(c);
                }
                't' => {
                    value = true;
                    if *stack.last().unwrap() == '|' {
                        short_circuit = paren_count;
                    }
                }
                'f' => {
                    value = false;
                    if *stack.last().unwrap() == '&' {
                        short_circuit = paren_count;
                    }
                }
                ')' => {
                    if stack.pop().unwrap() == '!' {
                        value = !value;
                    }
                    if let Some(&op) = stack.last()
                        && ((value && op == '|') || (!value && op == '&'))
                    {
                        short_circuit = paren_count;
                    }
                }
                _ => {}
            }
        }
        value
    }
}

#[test]
fn test() {
    let cases = vec![("&(|(f))", false), ("|(f,f,f,t)", true), ("!(&(f,t))", true)];
    for (expression, expected) in cases {
        assert_eq!(Solution::parse_bool_expr(expression.to_string()), expected);
    }
}
