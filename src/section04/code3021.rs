#![allow(dead_code)]

// 3021. Alice and Bob Playing Flower Game
// https://leetcode.com/problems/alice-and-bob-playing-flower-game/
// https://leetcode.cn/problems/alice-and-bob-playing-flower-game/
//
// Medium
//
// Alice and Bob are playing a turn-based game on a circular field surrounded by flowers.
// The circle represents the field, and there are x flowers in the clockwise direction between Alice and Bob,
// and y flowers in the anti-clockwise direction between them.
//
// The game proceeds as follows:
//
// Alice takes the first turn.
// In each turn, a player must choose either the clockwise or anti-clockwise direction and pick one flower from that side.
// At the end of the turn, if there are no flowers left at all, the current player captures their opponent and wins the game.
// Given two integers, n and m, the task is to compute the number of possible pairs (x, y) that satisfy the conditions:
//
// Alice must win the game according to the described rules.
// The number of flowers x in the clockwise direction must be in the range [1,n].
// The number of flowers y in the anti-clockwise direction must be in the range [1,m].
// Return the number of possible pairs (x, y) that satisfy the conditions mentioned in the statement.
//
// Example 1:
//
// Input: n = 3, m = 2
// Output: 3
// Explanation: The following pairs satisfy conditions described in the statement: (1,2), (3,2), (2,1).
//
// Example 2:
//
// Input: n = 1, m = 1
// Output: 0
// Explanation: No pairs satisfy the conditions described in the statement.
//
// Constraints:
//
// 1 <= n, m <= 10^5
//

struct Solution;

impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        let mut ans = 0_i64;
        for i in 1..=n.min(m) {
            ans += (m - i + 1) as i64 / 2;
            ans += (n - i + 1) as i64 / 2;
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::flower_game(3, 2), 3);
    assert_eq!(Solution::flower_game(1, 1), 0);
}
