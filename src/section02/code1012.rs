#![allow(dead_code)]

// 1012. Numbers With Repeated Digits
// https://leetcode.com/problems/numbers-with-repeated-digits/
// https://leetcode.cn/problems/numbers-with-repeated-digits/
//
// Given an integer n, return the number of positive integers in the range [1, n] that have at least one repeated digit.
//
// Example 1:
//
// Input: n = 20
// Output: 1
// Explanation: The only positive number (<= 20) with at least 1 repeated digit is 11.
//
// Example 2:
//
// Input: n = 100
// Output: 10
// Explanation: The positive numbers (<= 100) with atleast 1 repeated digit are 11, 22, 33, 44, 55, 66, 77, 88, 99, and 100.
//
// Example 3:
//
// Input: n = 1000
// Output: 262
//
// Constraints:
//
// - 1 <= n <= 10^9
//

struct Solution;

impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        fn _a(m: i32, n: i32) -> i32 {
            if n == 0 { 1 } else { _a(m, n - 1) * (m - n + 1) }
        }

        let mut digits = Vec::new();
        let mut x = n + 1;
        while x > 0 {
            digits.insert(0, x % 10);
            x /= 10;
        }

        let mut res = 0;
        let len = digits.len();
        for i in 1..len {
            res += 9 * _a(9, i as i32 - 1);
        }

        let mut seen = std::collections::HashSet::new();
        for (i, &digit) in digits.iter().enumerate() {
            for j in i32::from(i == 0)..digit {
                if !seen.contains(&j) {
                    res += _a(9 - i as i32, len as i32 - i as i32 - 1);
                }
            }
            if seen.contains(&digit) {
                break;
            }
            seen.insert(digit);
        }
        n - res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_dup_digits_at_most_n(20), 1);
    assert_eq!(Solution::num_dup_digits_at_most_n(100), 10);
    assert_eq!(Solution::num_dup_digits_at_most_n(1000), 262);
}
