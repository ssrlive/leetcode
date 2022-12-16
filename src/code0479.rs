#![allow(dead_code)]

// 479. Largest Palindrome Product
// https://leetcode.com/problems/largest-palindrome-product/
//
// Given an integer n, return the largest palindromic integer that can be represented as the product of two n-digits integers. Since the answer can be very large, return it modulo 1337.
//
// Example 1:
//
// Input: n = 2
// Output: 987
// Explanation: 99 x 91 = 9009, 9009 % 1337 = 987
//
// Example 2:
//
// Input: n = 1
// Output: 9
//
// Constraints:
//
// - 1 <= n <= 8
//

struct Solution;

impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        fn _largest_palindrome(n: i32) -> Option<i32> {
            if n == 1 {
                return Some(9);
            }
            let upper = 10_i64.pow(n as u32) - 1;
            let lower = 10_i64.pow(n as u32 - 1);
            for i in (lower..=upper).rev() {
                let s = format!("{}{}", i, i.to_string().chars().rev().collect::<String>());
                let palindrome = s.parse::<i64>().ok()?;
                for j in (lower..=upper).rev() {
                    if j * j < palindrome {
                        break;
                    }
                    if palindrome % j == 0 {
                        return Some((palindrome % 1337) as i32);
                    }
                }
            }
            Some(0)
        }

        _largest_palindrome(n).unwrap_or_default()
    }
}

#[test]
fn test_largest_palindrome() {
    assert_eq!(Solution::largest_palindrome(2), 987);
    assert_eq!(Solution::largest_palindrome(1), 9);
}
