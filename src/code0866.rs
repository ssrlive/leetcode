#![allow(dead_code)]

// 866. Prime Palindrome
// https://leetcode.com/problems/prime-palindrome/
// https://leetcode.cn/problems/prime-palindrome/
//
// Given an integer n, return the smallest prime palindrome greater than or equal to n.
//
// An integer is prime if it has exactly two divisors: 1 and itself. Note that 1 is not a prime number.
//
// For example, 2, 3, 5, 7, 11, and 13 are all primes.
// An integer is a palindrome if it reads the same from left to right as it does from right to left.
//
// For example, 101 and 12321 are palindromes.
// The test cases are generated so that the answer always exists and is in the range [2, 2 * 108].
//
// Example 1:
//
// Input: n = 6
// Output: 7
//
// Example 2:
//
// Input: n = 8
// Output: 11
//
// Example 3:
//
// Input: n = 13
// Output: 101
//
// Constraints:
//
// - 1 <= n <= 10^8
//

struct Solution;

impl Solution {
    pub fn prime_palindrome(n: i32) -> i32 {
        fn is_prime(n: i32) -> bool {
            if n == 1 {
                return false;
            }
            for i in 2..=(n as f64).sqrt() as i32 {
                if n % i == 0 {
                    return false;
                }
            }
            true
        }

        let n_str = n.to_string();
        let len = n_str.len() as i32;

        for k in std::cmp::max(0, len / 2 - 1)..5 {
            for i in 10_i32.pow(k as u32)..10_i32.pow(k as u32 + 1) {
                let i_str = i.to_string();
                if k > 0 && "24658".contains(i_str.chars().next().unwrap()) {
                    continue;
                }

                let cur = format!("{}{}", i_str, i_str.chars().rev().skip(1).collect::<String>());
                let cur_int = cur.parse::<i32>().unwrap();
                if cur_int >= n && is_prime(cur_int) {
                    return cur_int;
                }
            }

            for i in 10_i32.pow(k as u32)..10_i32.pow(k as u32 + 1) {
                let i_str = i.to_string();
                if "24658".contains(i_str.chars().next().unwrap()) {
                    continue;
                }

                let cur = format!("{}{}", i_str, i_str.chars().rev().collect::<String>());
                let cur_int = cur.parse::<i32>().unwrap();
                if cur_int >= n && is_prime(cur_int) {
                    return cur_int;
                }
            }
        }

        -1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::prime_palindrome(6), 7);
    assert_eq!(Solution::prime_palindrome(8), 11);
    assert_eq!(Solution::prime_palindrome(13), 101);
    assert_eq!(Solution::prime_palindrome(1000), 10301);
    assert_eq!(Solution::prime_palindrome(100000000), 100030001);
    assert_eq!(Solution::prime_palindrome(10), 11);
}
