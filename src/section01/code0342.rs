#![allow(dead_code)]

// 342. Power of Four
// https://leetcode.com/problems/power-of-four/
// https://leetcode.cn/problems/power-of-four/
//
// Given an integer n, return true if it is a power of four. Otherwise, return false.
//
// An integer n is a power of four, if there exists an integer x such that n == 4x.
//
// Example 1:
//
// Input: n = 16
// Output: true
//
// Example 2:
//
// Input: n = 5
// Output: false
//
// Example 3:
//
// Input: n = 1
// Output: true
//
// Constraints:
//
// - -2^31 <= n <= 2^31 - 1
//
// Follow up: Could you solve it without loops/recursion?
//

struct Solution;

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let mut n = n;
        while n % 4 == 0 {
            n /= 4;
        }
        n == 1
    }
}

#[test]
fn test_is_power_of_four() {
    assert!(Solution::is_power_of_four(16));
    assert!(!Solution::is_power_of_four(5));
    assert!(Solution::is_power_of_four(1));
}
