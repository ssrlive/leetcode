#![allow(dead_code)]

// 326. Power of Three
// https://leetcode.com/problems/power-of-three/
// https://leetcode.cn/problems/power-of-three/
//
// Given an integer n, return true if it is a power of three. Otherwise, return false.
// An integer n is a power of three, if there exists an integer x such that n == 3x.
//
// Example 1:
//
// Input: n = 27
// Output: true
//
// Example 2:
//
// Input: n = 0
// Output: false
//
// Example 3:
//
// Input: n = 9
// Output: true
//
// Example 4:
//
// Input: n = 45
// Output: false
//
// Constraints:
//
// -2^31 <= n <= 2^31 - 1
//
// Follow up: Could you solve it without loops/recursion?
//

struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let mut n = n;
        while n % 3 == 0 {
            n /= 3;
        }
        n == 1
    }
}

#[test]
fn test_is_power_of_three() {
    assert!(Solution::is_power_of_three(27));
    assert!(!Solution::is_power_of_three(0));
    assert!(Solution::is_power_of_three(9));
    assert!(!Solution::is_power_of_three(45));
}
