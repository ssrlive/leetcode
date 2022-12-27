#![allow(dead_code)]

// 2485. Find the Pivot Integer
// https://leetcode.com/problems/find-the-pivot-integer/
// https://leetcode.cn/problems/find-the-pivot-integer/
//
// Given a positive integer n, find the pivot integer x such that:
//
// The sum of all elements between 1 and x inclusively equals the sum of all elements between x and n inclusively.
// Return the pivot integer x. If no such integer exists, return -1. It is guaranteed that there will be at most one pivot index for the given input.
//
// Example 1:
//
// Input: n = 8
// Output: 6
// Explanation: 6 is the pivot integer since: 1 + 2 + 3 + 4 + 5 + 6 = 6 + 7 + 8 = 21.
//
// Example 2:
//
// Input: n = 1
// Output: 1
// Explanation: 1 is the pivot integer since: 1 = 1.
//
// Example 3:
//
// Input: n = 4
// Output: -1
// Explanation: It can be proved that no such integer exist.
//
// Constraints:
//
// - 1 <= n <= 1000
//

struct Solution;

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        fn sum_of_range(start: i32, end: i32) -> i32 {
            (start..=end).sum::<i32>()
        }

        for num in 0..=n {
            if sum_of_range(0, num) == sum_of_range(num, n) {
                return num;
            }
        }
        -1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::pivot_integer(8), 6);
    assert_eq!(Solution::pivot_integer(1), 1);
    assert_eq!(Solution::pivot_integer(4), -1);
}
