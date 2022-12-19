#![allow(dead_code)]

// 397. Integer Replacement
// https://leetcode.com/problems/integer-replacement/
// https://leetcode.cn/problems/integer-replacement/
//
// Given a positive integer n, you can apply one of the following operations:
//
// If n is even, replace n with n / 2.
// If n is odd, replace n with either n + 1 or n - 1.
// Return the minimum number of operations needed for n to become 1.
//
// Example 1:
//
// Input: n = 8
// Output: 3
// Explanation: 8 -> 4 -> 2 -> 1
//
// Example 2:
//
// Input: n = 7
// Output: 4
// Explanation: 7 -> 8 -> 4 -> 2 -> 1
// or 7 -> 6 -> 3 -> 2 -> 1
//
// Example 3:
//
// Input: n = 4
// Output: 2
//
// Constraints:
//
// -  1 <= n <= 2^31 - 1
//

struct Solution;

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        pub fn integer_replacement_i64(n: i64) -> i64 {
            let mut n = n;
            let mut count = 0;
            while n != 1 {
                if n % 2 == 0 {
                    n /= 2;
                } else if n == 3 || n % 4 == 1 {
                    n -= 1;
                } else {
                    n += 1;
                }
                count += 1;
            }
            count
        }
        integer_replacement_i64(n as i64) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::integer_replacement(8), 3);
    assert_eq!(Solution::integer_replacement(7), 4);
    assert_eq!(Solution::integer_replacement(4), 2);
    assert_eq!(Solution::integer_replacement(2147483647), 32);
}
