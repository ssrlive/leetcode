#![allow(dead_code)]

// 29. Divide Two Integers
// https://leetcode.com/problems/divide-two-integers/
// https://leetcode.cn/problems/divide-two-integers/
//
// Given two integers dividend and divisor, divide two integers without using multiplication, division, and mod operator.
//
// The integer division should truncate toward zero, which means losing its fractional part. For example, 8.345 would be truncated to 8, and -2.7335 would be truncated to -2.
//
// Return the quotient after dividing dividend by divisor.
//
// Note: Assume we are dealing with an environment that could only store integers within the 32-bit signed integer range: [−231, 231 − 1]. For this problem, if the quotient is strictly greater than 231 - 1, then return 231 - 1, and if the quotient is strictly less than -231, then return -231.
//
// Example 1:
//
// Input: dividend = 10, divisor = 3
// Output: 3
// Explanation: 10/3 = 3.33333.. which is truncated to 3.
//
// Example 2:
//
// Input: dividend = 7, divisor = -3
// Output: -2
// Explanation: 7/-3 = -2.33333.. which is truncated to -2.
//
// Constraints:
//
// - -2^31 <= dividend, divisor <= 2^31 - 1
// - divisor != 0
//

struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        pub fn rdiv(dividend: i64, divisor: i64) -> (i64, i64) {
            if dividend - divisor < divisor {
                (1, dividend - divisor)
            } else {
                let (q, r) = rdiv(dividend >> 1, divisor);
                (q << 1, (r << 1) + (dividend & 1))
            }
        }

        pub fn recursive_divide(dividend: i64, divisor: i64) -> i64 {
            if dividend < divisor {
                return 0;
            }
            let (mut q, mut r) = rdiv(dividend, divisor);
            while r >= divisor {
                let (temp_q, temp_r) = rdiv(r, divisor);
                q += temp_q;
                r = temp_r;
            }
            q
        }

        let signs = (if dividend >= 0 { 1 } else { -1 }, if divisor >= 0 { 1 } else { -1 });

        let sign = matches!(signs.0 + signs.1, 0);

        let mut n = recursive_divide((dividend as i64).abs(), (divisor as i64).abs());

        if sign {
            n = (!n) + 1;
        }

        if n > i32::MAX as i64 {
            i32::MAX
        } else if n < i32::MIN as i64 {
            i32::MIN
        } else {
            n as i32
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::divide(10, 3), 3);
    assert_eq!(Solution::divide(7, -3), -2);
    assert_eq!(Solution::divide(-2147483648, -1), 2147483647);
}
