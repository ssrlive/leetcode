#![allow(dead_code)]

// 357. Count Numbers with Unique Digits
// https://leetcode.com/problems/count-numbers-with-unique-digits/
// https://leetcode.cn/problems/count-numbers-with-unique-digits/
//
// Given an integer n, return the count of all numbers with unique digits, x, where 0 <= x < 10^n.
//
// Example 1:
//
// Input: n = 2
// Output: 91
// Explanation: The answer should be the total numbers in the range of 0 ≤ x < 100,
// excluding 11,22,33,44,55,66,77,88,99
//
// Example 2:
//
// Input: n = 0
// Output: 1
//
// Constraints:
//
// - 0 <= n <= 8
//

struct Solution;

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        fn p(n: i32, r: i32) -> i32 {
            (n - r + 1..=n).product()
        }
        (1..=n).fold(1, |acc, r| acc + p(10, r) - p(9, r - 1))
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_numbers_with_unique_digits(0), 1);
    assert_eq!(Solution::count_numbers_with_unique_digits(1), 10);
    assert_eq!(Solution::count_numbers_with_unique_digits(2), 91);
    assert_eq!(Solution::count_numbers_with_unique_digits(3), 739);
    assert_eq!(Solution::count_numbers_with_unique_digits(4), 5275);
    assert_eq!(Solution::count_numbers_with_unique_digits(5), 32491);
    assert_eq!(Solution::count_numbers_with_unique_digits(6), 168571);
    assert_eq!(Solution::count_numbers_with_unique_digits(7), 712891);
    assert_eq!(Solution::count_numbers_with_unique_digits(8), 2345851);
}
