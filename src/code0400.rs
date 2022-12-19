#![allow(dead_code)]

// 400. Nth Digit
// https://leetcode.com/problems/nth-digit/
// https://leetcode.cn/problems/nth-digit/
//
// Given an integer n, return the nth digit of the infinite integer sequence [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ...].
//
// Example 1:
//
// Input: n = 3
// Output: 3
//
// Example 2:
//
// Input: n = 11
// Output: 0
// Explanation: The 11th digit of the sequence 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ... is a 0, which is part of the number 10.
//
// Constraints:
//
// -  1 <= n <= 2^31 - 1
//

struct Solution;

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        fn _find_nth_digit(n: i32) -> Option<i32> {
            let mut n = n as i64;
            let mut digits = 1;
            let mut start = 1;
            let mut count = 9;
            while n > count {
                n -= count;
                digits += 1;
                start *= 10;
                count = digits as i64 * start * 9;
            }
            let num = start + (n - 1) / digits as i64;
            let s = num.to_string();
            let idx = (n - 1) % digits as i64;
            let v = s.chars().nth(idx as usize)? as i32 - '0' as i32;
            Some(v)
        }

        _find_nth_digit(n).unwrap_or_default()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_nth_digit(3), 3);
    assert_eq!(Solution::find_nth_digit(11), 0);
}
