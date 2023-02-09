#![allow(dead_code)]

/*

// 1523. Count Odd Numbers in an Interval Range
// https://leetcode.com/problems/count-odd-numbers-in-an-interval-range/
// https://leetcode.cn/problems/count-odd-numbers-in-an-interval-range/
//
// Easy
//
// Given two non-negative integers low and high. Return the count of odd numbers between low and high (inclusive).

Example 1:

Input: low = 3, high = 7
Output: 3
Explanation: The odd numbers between 3 and 7 are [3,5,7].

Example 2:

Input: low = 8, high = 10
Output: 1
Explanation: The odd numbers between 8 and 10 are [9].

Constraints:

    0 <= low <= high <= 10^9
*/

struct Solution;

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        (high - low) / 2 + ((high % 2) | (low % 2))
    }
}

#[test]
fn test() {
    let cases = vec![(3, 7, 3), (8, 10, 1), (0, 10, 5), (0, 11, 6)];
    for (low, high, expect) in cases {
        assert_eq!(Solution::count_odds(low, high), expect);
    }
}
