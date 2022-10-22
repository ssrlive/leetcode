#![allow(dead_code)]

// 150. Evaluate Reverse Polish Notation
// https://leetcode.com/problems/evaluate-reverse-polish-notation/

// Evaluate the value of an arithmetic expression in Reverse Polish Notation.
//
// Valid operators are +, -, *, and /. Each operand may be an integer or another expression.
//
// - Note that division between two integers should truncate toward zero.
//
// It is guaranteed that the given RPN expression is always valid.
// That means the expression would always evaluate to a result, and there will not be any division by zero operation.

struct Solution;

use std::ops::{Add, Div, Mul, Sub};
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];

        let binary_op = |f: fn(_, _) -> _, stack: &mut Vec<_>| {
            let (o2, o1) = (stack.pop().unwrap(), stack.pop().unwrap());
            stack.push(f(o1, o2))
        };

        for token in tokens {
            match token.as_str() {
                "+" => binary_op(i32::add, &mut stack),
                "-" => binary_op(i32::sub, &mut stack),
                "*" => binary_op(i32::mul, &mut stack),
                "/" => binary_op(i32::div, &mut stack),
                number => stack.push(number.parse().unwrap()),
            }
        }
        stack.pop().unwrap()
    }
}

#[test]
fn test_eval_rpn() {
    assert_eq!(
        Solution::eval_rpn(vec![
            "2".to_string(),
            "1".to_string(),
            "+".to_string(),
            "3".to_string(),
            "*".to_string()
        ]),
        9
    );
    assert_eq!(
        Solution::eval_rpn(vec![
            "4".to_string(),
            "13".to_string(),
            "5".to_string(),
            "/".to_string(),
            "+".to_string()
        ]),
        6
    );
    assert_eq!(
        Solution::eval_rpn(vec![
            "10".to_string(),
            "6".to_string(),
            "9".to_string(),
            "3".to_string(),
            "+".to_string(),
            "-11".to_string(),
            "*".to_string(),
            "/".to_string(),
            "*".to_string(),
            "17".to_string(),
            "+".to_string(),
            "5".to_string(),
            "+".to_string()
        ]),
        22
    );
}
