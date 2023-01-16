#![allow(dead_code)]

// 2413. Smallest Even Multiple
// https://leetcode.com/problems/smallest-even-multiple/
// https://leetcode.cn/problems/smallest-even-multiple/
//
// Given a positive integer n, return the smallest positive integer that is a multiple of both 2 and n.
//
// Example 1:
//
// Input: n = 5
// Output: 10
// Explanation: The smallest multiple of both 5 and 2 is 10.
//
// Example 2:
//
// Input: n = 6
// Output: 6
// Explanation: The smallest multiple of both 6 and 2 is 6. Note that a number is a multiple of itself.
//
// Constraints:
//
// - 1 <= n <= 150
//

struct Solution;

impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        match n % 2 {
            0 => n,
            _ => n * 2,
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::smallest_even_multiple(5), 10);
    assert_eq!(Solution::smallest_even_multiple(6), 6);
}
