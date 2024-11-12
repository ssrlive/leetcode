#![allow(dead_code)]

// 3345. Smallest Divisible Digit Product I
// https://leetcode.com/problems/smallest-divisible-digit-product-i/
// https://leetcode.cn/problems/smallest-divisible-digit-product-i/
//
// Easy
//
// You are given two integers n and t. Return the smallest number greater than or equal to n such that the product of its digits is divisible by t.
//
// Example 1:
//
// Input: n = 10, t = 2
//
// Output: 10
//
// Explanation:
//
// The digit product of 10 is 0, which is divisible by 2, making it the smallest number greater than or equal to 10 that satisfies the condition.
//
// Example 2:
//
// Input: n = 15, t = 3
//
// Output: 16
//
// Explanation:
//
// The digit product of 16 is 6, which is divisible by 3, making it the smallest number greater than or equal to 15 that satisfies the condition.
//
// Constraints:
//
//     1 <= n <= 100
//     1 <= t <= 10
//

struct Solution;

impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        (n..n + 10).find(|&x| Self::f(x, t)).unwrap()
    }

    fn f(mut x: i32, t: i32) -> bool {
        let mut p = 1;
        while x > 0 {
            p *= x % 10;
            x /= 10
        }
        p % t == 0
    }
}

#[test]
fn test() {
    assert_eq!(Solution::smallest_number(10, 2), 10);
    assert_eq!(Solution::smallest_number(15, 3), 16);
}
