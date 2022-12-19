#![allow(dead_code)]

// 264. Ugly Number II
// https://leetcode.com/problems/ugly-number-ii/
// https://leetcode.cn/problems/ugly-number-ii/
//
// An ugly number is a positive integer whose prime factors are limited to 2, 3, and 5.
//
// Given an integer n, return the nth ugly number.
//
// Example 1:
//
// Input: n = 10
// Output: 12
// Explanation: [1, 2, 3, 4, 5, 6, 8, 9, 10, 12] is the sequence of the first 10 ugly numbers.
//
// Example 2:
//
// Input: n = 1
// Output: 1
// Explanation: 1 has no prime factors, therefore all of its prime factors are limited to 2, 3, and 5.
//
// Constraints:
//
// 1 <= n <= 1690
//

struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut ugly = vec![1];
        let mut i2 = 0;
        let mut i3 = 0;
        let mut i5 = 0;

        for _ in 1..n {
            let next2 = ugly[i2] * 2;
            let next3 = ugly[i3] * 3;
            let next5 = ugly[i5] * 5;
            let next = next2.min(next3).min(next5);

            if next == next2 {
                i2 += 1;
            }
            if next == next3 {
                i3 += 1;
            }
            if next == next5 {
                i5 += 1;
            }

            ugly.push(next);
        }

        ugly[n as usize - 1]
    }
}

#[test]
fn test_nth_ugly_number() {
    assert_eq!(Solution::nth_ugly_number(10), 12);
    assert_eq!(Solution::nth_ugly_number(1), 1);
}
