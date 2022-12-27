#![allow(dead_code)]

// 233. Number of Digit One
// https://leetcode.com/problems/number-of-digit-one/
// https://leetcode.cn/problems/number-of-digit-one/
//
// Given an integer n, count the total number of digit 1 appearing in all non-negative integers less than or equal to n.
//
// Example 1:
// Input: n = 13
// Output: 6
//
// Example 2:
// Input: n = 0
// Output: 0
//
// Constraints:
// 0 <= n <= 2 * 10^9
//

struct Solution;

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let m: i128 = n as i128;
        let mut counter: i128 = 0;
        let mut i: i128 = 1;

        while i <= m {
            let divider: i128 = i * 10;
            counter += (m / divider) * i + (m % divider - i + 1).clamp(0, i);
            i *= 10;
        }
        counter as i32
    }
}

#[test]
fn test_count_digit_one() {
    assert_eq!(Solution::count_digit_one(13), 6);
    assert_eq!(Solution::count_digit_one(0), 0);
}
