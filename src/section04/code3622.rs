#![allow(dead_code)]

// 3622. Check Divisibility by Digit Sum and Product
// https://leetcode.com/problems/check-divisibility-by-digit-sum-and-product/
// https://leetcode.cn/problems/check-divisibility-by-digit-sum-and-product/
//
// Easy
//
// You are given a positive integer n. Determine whether n is divisible by the sum of the following two values:
//
// The digit sum of n (the sum of its digits).
//
// The digit product of n (the product of its digits).
//
// Return true if n is divisible by this sum; otherwise, return false.
//
// Example 1:
//
// Input: n = 99
//
// Output: true
//
// Explanation:
//
// Since 99 is divisible by the sum (9 + 9 = 18) plus product (9 * 9 = 81) of its digits (total 99), the output is true.
//
// Example 2:
//
// Input: n = 23
//
// Output: false
//
// Explanation:
//
// Since 23 is not divisible by the sum (2 + 3 = 5) plus product (2 * 3 = 6) of its digits (total 11), the output is false.
//
// Constraints:
//
// 1 <= n <= 10^6
//

struct Solution;

impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let n_string = n.to_string();
        let mut digit_sum = 0;
        let mut digit_product = 1;
        for karakter in n_string.chars() {
            let digit = karakter.to_digit(10).unwrap() as i32;
            digit_sum += digit;
            digit_product *= digit;
        }
        let total = digit_sum + digit_product;
        if total == 0 {
            return false;
        }
        n % total == 0
    }
}

#[test]
fn test() {
    assert!(Solution::check_divisibility(99));
    assert!(!Solution::check_divisibility(23));
}
