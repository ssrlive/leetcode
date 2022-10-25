#![allow(dead_code)]

// 201. Bitwise AND of Numbers Range
// https://leetcode.com/problems/bitwise-and-of-numbers-range/
//
// Given a range [m, n] where 0 <= m <= n <= 2147483647, return the bitwise AND
// of all numbers in this range, inclusive.
//
// For example, given the range [5, 7], you should return 4.

struct Solution;

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut left = left;
        let mut right = right;
        let mut count = 0;
        while left != right {
            left >>= 1;
            right >>= 1;
            count += 1;
        }
        left << count
    }
}

#[test]
fn test_range_bitwise_and() {
    assert_eq!(Solution::range_bitwise_and(5, 7), 4);
    assert_eq!(Solution::range_bitwise_and(0, 1), 0);
}
