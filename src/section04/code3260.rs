#![allow(dead_code)]

// 3260. Find the Largest Palindrome Divisible by K
// https://leetcode.com/problems/find-the-largest-palindrome-divisible-by-k/
// https://leetcode.cn/problems/find-the-largest-palindrome-divisible-by-k/
//
// Hard
//
// You are given two positive integers n and k.
//
// An integer x is called k-palindromic if:
//
//     x is a palindrome.
//     x is divisible by k.
//
// Return the largest integer having n digits (as a string) that is k-palindromic.
//
// Note that the integer must not have leading zeros.
//
// Example 1:
//
// Input: n = 3, k = 5
//
// Output: "595"
//
// Explanation:
//
// 595 is the largest k-palindromic integer with 3 digits.
//
// Example 2:
//
// Input: n = 1, k = 4
//
// Output: "8"
//
// Explanation:
//
// 4 and 8 are the only k-palindromic integers with 1 digit.
//
// Example 3:
//
// Input: n = 5, k = 6
//
// Output: "89898"
//
// Constraints:
//
//     1 <= n <= 10^5
//     1 <= k <= 9
//

struct Solution;

impl Solution {
    pub fn largest_palindrome(n: i32, k: i32) -> String {
        use std::iter::once;
        let n = n as usize;
        match k {
            1 | 3 | 9 => "9".repeat(n),
            5 => {
                if n == 1 {
                    "5".to_string()
                } else {
                    once('5').chain(std::iter::repeat_n('9', n - 2)).chain(once('5')).collect()
                }
            }
            2 => {
                if n == 1 {
                    "8".to_string()
                } else {
                    once('8').chain(std::iter::repeat_n('9', n - 2)).chain(once('8')).collect()
                }
            }
            4 => {
                if n <= 4 {
                    "8".repeat(n)
                } else {
                    std::iter::repeat_n('8', 2)
                        .chain(std::iter::repeat_n('9', n - 4))
                        .chain(std::iter::repeat_n('8', 2))
                        .collect()
                }
            }
            8 => {
                if n <= 6 {
                    "8".repeat(n)
                } else {
                    std::iter::repeat_n('8', 3)
                        .chain(std::iter::repeat_n('9', n - 6))
                        .chain(std::iter::repeat_n('8', 3))
                        .collect()
                }
            }
            6 => {
                if n <= 2 {
                    "6".repeat(n)
                } else if n == 3 {
                    "888".to_string()
                } else if n.is_multiple_of(2) {
                    once('8')
                        .chain(std::iter::repeat_n('9', (n - 4) / 2))
                        .chain(std::iter::repeat_n('7', 2))
                        .chain(std::iter::repeat_n('9', (n - 4) / 2))
                        .chain(once('8'))
                        .collect()
                } else {
                    once('8')
                        .chain(std::iter::repeat_n('9', (n - 3) / 2))
                        .chain(once('8'))
                        .chain(std::iter::repeat_n('9', (n - 3) / 2))
                        .chain(once('8'))
                        .collect()
                }
            }
            7 => match n % 12 {
                0 | 6 => "9".repeat(n),
                1 | 5 => std::iter::repeat_n('9', (n - 1) / 2)
                    .chain(once('7'))
                    .chain(std::iter::repeat_n('9', (n - 1) / 2))
                    .collect(),
                2 | 4 => std::iter::repeat_n('9', n / 2 - 1)
                    .chain(std::iter::repeat_n('7', 2))
                    .chain(std::iter::repeat_n('9', n / 2 - 1))
                    .collect(),
                3 => std::iter::repeat_n('9', (n - 1) / 2)
                    .chain(once('5'))
                    .chain(std::iter::repeat_n('9', (n - 1) / 2))
                    .collect(),
                7 | 11 => std::iter::repeat_n('9', (n - 1) / 2)
                    .chain(once('4'))
                    .chain(std::iter::repeat_n('9', (n - 1) / 2))
                    .collect(),
                8 | 10 => std::iter::repeat_n('9', n / 2 - 1)
                    .chain(std::iter::repeat_n('4', 2))
                    .chain(std::iter::repeat_n('9', n / 2 - 1))
                    .collect(),
                9 => std::iter::repeat_n('9', (n - 1) / 2)
                    .chain(once('6'))
                    .chain(std::iter::repeat_n('9', (n - 1) / 2))
                    .collect(),
                _ => String::new(),
            },
            _ => String::new(),
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::largest_palindrome(3, 5), "595");
    assert_eq!(Solution::largest_palindrome(1, 4), "8");
    assert_eq!(Solution::largest_palindrome(5, 6), "89898");
}
