#![allow(dead_code)]

// 263. Ugly Numbers
// https://leetcode.com/problems/ugly-number/
// https://leetcode.cn/problems/ugly-number/
//
// An ugly number is a positive integer whose prime factors are limited to 2, 3, and 5.
//
// Given an integer n, return true if n is an ugly number.
//
// Example 1:
//
// Input: n = 6
// Output: true
// Explanation: 6 = 2 × 3
//
// Example 2:
//
// Input: n = 1
// Output: true
// Explanation: 1 has no prime factors, therefore all of its prime factors are limited to 2, 3, and 5.
//
// Example 3:
//
// Input: n = 14
// Output: false
// Explanation: 14 is not ugly since it includes the prime factor 7.
//
// Constraints:
//
// - -2^31 <= n <= 2^31 - 1
//

struct Solution;

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        let mut n = n;
        if n <= 0 {
            return false;
        }
        while n % 2 == 0 {
            n /= 2;
        }
        while n % 3 == 0 {
            n /= 3;
        }
        while n % 5 == 0 {
            n /= 5;
        }
        n == 1
    }
}

#[test]
fn test_is_ugly() {
    assert!(Solution::is_ugly(6));
    assert!(Solution::is_ugly(1));
    assert!(!Solution::is_ugly(14));
}
