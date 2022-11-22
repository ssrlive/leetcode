#![allow(dead_code)]

// 371. Sum of Two Integers
// https://leetcode.com/problems/sum-of-two-integers/
//
// Given two integers a and b, return the sum of the two integers without using the operators + and -.
//
// Example 1:
//
// Input: a = 1, b = 2
// Output: 3
//
// Example 2:
//
// Input: a = -2, b = 3
// Output: 1
//
// Constraints:
//
// -1000 <= a, b <= 1000
//

struct Solution;

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            let carry = a & b;
            a ^= b;
            b = carry << 1;
        }
        a
    }
}

#[test]
fn test() {
    assert_eq!(Solution::get_sum(1, 2), 3);
    assert_eq!(Solution::get_sum(-2, 3), 1);
}
