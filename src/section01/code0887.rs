#![allow(dead_code)]

// 887. Super Egg Drop
// https://leetcode.com/problems/super-egg-drop/
// https://leetcode.cn/problems/super-egg-drop/
//
// You are given k identical eggs and you have access to a building with n floors labeled from 1 to n.
//
// You know that there exists a floor f where 0 <= f <= n such that any egg dropped at a floor higher than f will break,
// and any egg dropped at or below floor f will not break.
//
// Each move, you may take an unbroken egg and drop it from any floor x (where 1 <= x <= n).
// If the egg breaks, you can no longer use it. However, if the egg does not break, you may reuse it in future moves.
//
// Return the minimum number of moves that you need to determine with certainty what the value of f is.
//
// Example 1:
//
// Input: k = 1, n = 2
// Output: 2
// Explanation:
// Drop the egg from floor 1. If it breaks, we know that f = 0.
// Otherwise, drop the egg from floor 2. If it breaks, we know that f = 1.
// If it does not break, then we know f = 2.
// Hence, we need at minimum 2 moves to determine with certainty what the value of f is.
//
// Example 2:
//
// Input: k = 2, n = 6
// Output: 3
//
// Example 3:
//
// Input: k = 3, n = 14
// Output: 4
//
// Constraints:
//
// - 1 <= k <= 100
// - 1 <= n <= 10^4
//

struct Solution;

impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let k = k as usize;
        let n = n as usize;

        let mut dp = vec![vec![0_i64; n + 1]; k + 1];
        let mut moves = 1;

        while moves <= n {
            for eggs in 1..=k {
                dp[eggs][moves] = 1 + dp[eggs - 1][moves - 1] + dp[eggs][moves - 1];
            }
            if dp[k][moves] >= n as i64 {
                break;
            }
            moves += 1;
        }

        moves as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::super_egg_drop(1, 2), 2);
    assert_eq!(Solution::super_egg_drop(2, 6), 3);
    assert_eq!(Solution::super_egg_drop(3, 14), 4);
}
