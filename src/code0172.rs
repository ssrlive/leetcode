#![allow(dead_code)]

// 172. Factorial Trailing Zeroes
// https://leetcode.com/problems/factorial-trailing-zeroes/
// https://leetcode.cn/problems/factorial-trailing-zeroes/
//
// Given an integer n, return the number of trailing zeroes in n!.
//
// Note that n! = n * (n - 1) * (n - 2) * ... * 3 * 2 * 1.
//

struct Solution;

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut n = n;
        let mut result = 0;
        while n > 0 {
            n /= 5;
            result += n;
        }
        result
    }
}

#[test]
fn test_trailing_zeroes() {
    assert_eq!(Solution::trailing_zeroes(3), 0);
    assert_eq!(Solution::trailing_zeroes(5), 1);
    assert_eq!(Solution::trailing_zeroes(0), 0);
    assert_eq!(Solution::trailing_zeroes(30), 7);
}
