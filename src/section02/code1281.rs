#![allow(dead_code)]

// 1281. Subtract the Product and Sum of Digits of an Integer
// https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/
// https://leetcode.cn/problems/subtract-the-product-and-sum-of-digits-of-an-integer/
//
// Easy
//
// Given an integer number n, return the difference between the product of its digits and the sum of its digits.
//
// Example 1:
//
// Input: n = 234
// Output: 15
// Explanation:
// Product of digits = 2 * 3 * 4 = 24
// Sum of digits = 2 + 3 + 4 = 9
// Result = 24 - 9 = 15
//
// Example 2:
//
// Input: n = 4421
// Output: 21
// Explanation:
// Product of digits = 4 * 4 * 2 * 1 = 32
// Sum of digits = 4 + 4 + 2 + 1 = 11
// Result = 32 - 11 = 21
//
// Constraints:
//
// -    1 <= n <= 10^5
//

struct Solution;

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut n = n;
        let mut product = 1;
        let mut sum = 0;
        while n > 0 {
            let digit = n % 10;
            product *= digit;
            sum += digit;
            n /= 10;
        }
        product - sum
    }
}

#[test]
fn test() {
    assert_eq!(Solution::subtract_product_and_sum(234), 15);
    assert_eq!(Solution::subtract_product_and_sum(4421), 21);
}
