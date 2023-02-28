#![allow(dead_code)]

/*

// 1922. Count Good Numbers
// https://leetcode.com/problems/count-good-numbers/
// https://leetcode.cn/problems/count-good-numbers/
//
// Medium
//
// A digit string is good if the digits (0-indexed) at even indices are even and the digits at odd indices are prime (2, 3, 5, or 7).

    For example, "2582" is good because the digits (2 and 8) at even positions are even and the digits (5 and 2) at odd positions are prime. However, "3245" is not good because 3 is at an even index but is not even.

Given an integer n, return the total number of good digit strings of length n. Since the answer may be large, return it modulo 109 + 7.

A digit string is a string consisting of digits 0 through 9 that may contain leading zeros.

Example 1:

Input: n = 1
Output: 5
Explanation: The good numbers of length 1 are "0", "2", "4", "6", "8".

Example 2:

Input: n = 4
Output: 400

Example 3:

Input: n = 50
Output: 564908303

Constraints:

    1 <= n <= 10^15
*/

struct Solution;

impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        const MOD: i64 = 1_000_000_007;
        fn repeat_square(n: i64, p: i64) -> i64 {
            if p == 0 {
                1
            } else if p % 2 == 0 {
                repeat_square(n, p / 2).pow(2) % MOD
            } else {
                n * repeat_square(n, p - 1) % MOD
            }
        }

        let odd = n / 2;
        let even = n % 2 + n / 2;
        (repeat_square(4, odd) * repeat_square(5, even) % MOD) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_good_numbers(1), 5);
    assert_eq!(Solution::count_good_numbers(4), 400);
    assert_eq!(Solution::count_good_numbers(50), 564908303);
}
