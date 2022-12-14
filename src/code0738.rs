#![allow(dead_code)]

// 738. Monotone Increasing Digits
// https://leetcode.com/problems/monotone-increasing-digits/
//
// An integer has monotone increasing digits if and only if each pair of adjacent digits x and y satisfy x <= y.
//
// Given an integer n, return the largest number that is less than or equal to n with monotone increasing digits.
//
// Example 1:
//
// Input: n = 10
// Output: 9
//
// Example 2:
//
// Input: n = 1234
// Output: 1234
//
// Example 3:
//
// Input: n = 332
// Output: 299
//
// Constraints:
//
// - 0 <= n <= 10^9
//

struct Solution;

impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut n = n;
        let mut digits = Vec::new();
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.reverse();
        let mut i = 1;
        while i < digits.len() && digits[i - 1] <= digits[i] {
            i += 1;
        }
        if i < digits.len() {
            while i > 0 && digits[i - 1] > digits[i] {
                digits[i - 1] -= 1;
                i -= 1;
            }
            for item in digits.iter_mut().skip(i + 1) {
                *item = 9;
            }
        }
        let mut result = 0;
        for d in digits {
            result = result * 10 + d;
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::monotone_increasing_digits(10), 9);
    assert_eq!(Solution::monotone_increasing_digits(1234), 1234);
    assert_eq!(Solution::monotone_increasing_digits(332), 299);
}
