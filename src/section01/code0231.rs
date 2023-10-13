#![allow(dead_code)]

// 231. Power of Two
// https://leetcode.com/problems/power-of-two/
// https://leetcode.cn/problems/power-of-two/
//
// Given an integer n, return true if it is a power of two. Otherwise, return false.
//
// An integer n is a power of two, if there exists an integer x such that n == 2x.
//
// Example 1:
// Input: n = 1
// Output: true
// Explanation: 20 = 1
//
// Example 2:
// Input: n = 16
// Output: true
// Explanation: 24 = 16
//
// Example 3:
// Input: n = 3
// Output: false
//

struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            return false;
        }

        let mut n = n;
        while n % 2 == 0 {
            n /= 2;
        }

        n == 1
    }
}

#[test]
fn test_is_power_of_two() {
    assert!(Solution::is_power_of_two(1));
    assert!(Solution::is_power_of_two(16));
    assert!(!Solution::is_power_of_two(3));
}
