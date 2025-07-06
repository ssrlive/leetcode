#![allow(dead_code)]

// 2544. Alternating Digit Sum
// https://leetcode.com/problems/alternating-digit-sum/
// https://leetcode.cn/problems/alternating-digit-sum/
//
// Easy
//
// You are given a positive integer n. Each digit of n has a sign according to the following rules:
//
// The most significant digit is assigned a positive sign.
// Each other digit has an opposite sign to its adjacent digits.
//
// Return the sum of all digits with their corresponding sign.
//
// Example 1:
//
// Input: n = 521
// Output: 4
// Explanation: (+5) + (-2) + (+1) = 4.
//
// Example 2:
//
// Input: n = 111
// Output: 1
// Explanation: (+1) + (-1) + (+1) = 1.
//
// Example 3:
//
// Input: n = 886996
// Output: 0
// Explanation: (+8) + (-8) + (+6) + (-9) + (+9) + (-6) = 0.
//
// Constraints:
//
// 1 <= n <= 10^9
//

struct Solution;

impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        let mut v = vec![];
        let mut n = n;

        while n > 0 {
            v.push(n % 10);
            n /= 10;
        }
        v.reverse();

        let mut ret = 0;
        for (i, &item) in v.iter().enumerate() {
            if i.is_multiple_of(2) {
                ret += item;
            } else {
                ret -= item;
            }
        }

        ret
    }
}

#[test]
fn test() {
    assert_eq!(Solution::alternate_digit_sum(521), 4);
    assert_eq!(Solution::alternate_digit_sum(111), 1);
    assert_eq!(Solution::alternate_digit_sum(886996), 0);
}
