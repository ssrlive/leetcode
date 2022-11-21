#![allow(dead_code)]

// 343. Integer Break
// https://leetcode.com/problems/integer-break/
//
// Given an integer n, break it into the sum of k positive integers, where k >= 2, and maximize the product of those integers.
//
// Return the maximum product you can get.
//
// Example 1:
//
// Input: n = 2
// Output: 1
// Explanation: 2 = 1 + 1, 1 × 1 = 1.
//
// Example 2:
//
// Input: n = 10
// Output: 36
// Explanation: 10 = 3 + 3 + 4, 3 × 3 × 4 = 36.
//
// Constraints:
//
// 2 <= n <= 58
//
// Follow up: Could you solve it in O(n) time and O(1) space?

struct Solution;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n == 2 {
            return 1;
        }
        if n == 3 {
            return 2;
        }
        let mut n = n;
        let mut result = 1;
        while n > 4 {
            result *= 3;
            n -= 3;
        }
        result * n
    }
}

#[test]
fn test_integer_break() {
    assert_eq!(Solution::integer_break(2), 1);
    assert_eq!(Solution::integer_break(10), 36);
}
