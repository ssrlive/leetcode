#![allow(dead_code)]

// 790. Domino and Tromino Tiling
// https://leetcode.com/problems/domino-and-tromino-tiling/
//
// You have two types of tiles: a 2 x 1 domino shape and a tromino shape. You may rotate these shapes.
//
// Given an integer n, return the number of ways to tile an 2 x n board. Since the answer may be very large, return it modulo 109 + 7.
//
// In a tiling, every square must be covered by a tile. Two tilings are different if and only if there are two 4-directionally
// adjacent cells on the board such that exactly one of the tilings has both squares occupied by a tile.
//
// Example 1:
//
// Input: n = 3
// Output: 5
// Explanation: The five different ways are show above.
//
// Example 2:
//
// Input: n = 1
// Output: 1
//
// Constraints:
//
// - 1 <= n <= 1000
//

struct Solution;

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let mut dp: Vec<Vec<i64>> = vec![vec![0; n as usize]; 3];
        if n == 1 {
            return 1;
        }
        dp[2][0] = 1;
        dp[0][1] = 1;
        dp[1][1] = 1;
        dp[2][1] = 2;

        let d: i64 = 1_000_000_000 + 7;
        for i in 2..(n as usize) {
            dp[0][i] = (dp[2][i - 2] + dp[1][i - 1]) % d;
            dp[1][i] = (dp[2][i - 2] + dp[0][i - 1]) % d;
            dp[2][i] = (dp[2][i - 2] + dp[0][i - 1] + dp[1][i - 1] + dp[2][i - 1]) % d;
        }

        dp[2][(n as usize) - 1] as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_tilings(3), 5);
    assert_eq!(Solution::num_tilings(1), 1);
}
