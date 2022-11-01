#![allow(dead_code)]

// 279. Perfect Squares
// https://leetcode.com/problems/perfect-squares/
//
// Given an integer n, return the least number of perfect square numbers that sum to n.
//
// A perfect square is an integer that is the square of an integer; in other words,
// it is the product of some integer with itself. For example, 1, 4, 9, and 16
// are perfect squares while 3 and 11 are not.
//
// Example 1:
//
// Input: n = 12
// Output: 3
// Explanation: 12 = 4 + 4 + 4.
//
// Example 2:
//
// Input: n = 13
// Output: 2
// Explanation: 13 = 4 + 9.
//
// Constraints:
//
// 1 <= n <= 104
//

struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        for i in 1..=n {
            let mut min = i;
            let mut j = 1;
            while j * j <= i {
                min = min.min(dp[i - j * j] + 1);
                j += 1;
            }
            dp[i] = min;
        }
        dp[n] as i32
    }
}

#[test]
fn test_num_squares() {
    assert_eq!(Solution::num_squares(12), 3);
    assert_eq!(Solution::num_squares(13), 2);
}
