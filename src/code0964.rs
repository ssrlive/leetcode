#![allow(dead_code)]

// 964. Least Operators to Express Number
// https://leetcode.com/problems/least-operators-to-express-number/
// https://leetcode.cn/problems/least-operators-to-express-number/
//
// Given a single positive integer x, we will write an expression of the form x (op1) x (op2) x (op3) x ...
// where each operator op1, op2, etc. is either addition, subtraction, multiplication, or division (+, -, *, or /).
// For example, with x = 3, we might write 3 * 3 / 3 + 3 - 3 which is a value of 3.
//
// When writing such an expression, we adhere to the following conventions:
//
// The division operator (/) returns rational numbers.
// There are no parentheses placed anywhere.
// We use the usual order of operations: multiplication and division happen before addition and subtraction.
// It is not allowed to use the unary negation operator (-). For example, "x - x" is a valid expression as it only uses subtraction, but "-x + x" is not because it uses negation.
// We would like to write an expression with the least number of operators such that the expression equals the given target. Return the least number of operators used.
//
// Example 1:
//
// Input: x = 3, target = 19
// Output: 5
// Explanation: 3 * 3 + 3 * 3 + 3 / 3.
// The expression contains 5 operations.
//
// Example 2:
//
// Input: x = 5, target = 501
// Output: 8
// Explanation: 5 * 5 * 5 * 5 - 5 * 5 * 5 + 5 / 5.
// The expression contains 8 operations.
//
// Example 3:
//
// Input: x = 100, target = 100000000
// Output: 3
// Explanation: 100 * 100 * 100 * 100.
// The expression contains 3 operations.
//
// Constraints:
//
// - 2 <= x <= 100
// - 1 <= target <= 2 * 10^8
//

struct Solution;

impl Solution {
    pub fn least_ops_express_target(x: i32, target: i32) -> i32 {
        let mut levels = vec![];
        let mut target = target;
        while target > 0 {
            levels.push(target % x);
            target /= x;
        }
        let mut carrybitted = levels.len() as i32;
        let mut nocarrybitted = 0;
        for i in (1..levels.len()).rev() {
            let carry = std::cmp::min(
                (x - levels[i] - 1) * i as i32 + carrybitted,
                (levels[i] + 1) * i as i32 + nocarrybitted,
            );
            let nocarry = std::cmp::min(
                (x - levels[i]) * i as i32 + carrybitted,
                (levels[i]) * i as i32 + nocarrybitted,
            );
            carrybitted = carry;
            nocarrybitted = nocarry;
        }
        std::cmp::min(levels[0] * 2 + nocarrybitted, (x - levels[0]) * 2 + carrybitted) - 1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::least_ops_express_target(3, 19), 5);
    assert_eq!(Solution::least_ops_express_target(5, 501), 8);
    assert_eq!(Solution::least_ops_express_target(100, 100000000), 3);
}
