#![allow(dead_code)]

// 1025. Divisor Game
// https://leetcode.com/problems/divisor-game/
// https://leetcode.cn/problems/divisor-game/
//
// Alice and Bob take turns playing a game, with Alice starting first.
//
// Initially, there is a number n on the chalkboard. On each player's turn, that player makes a move consisting of:
//
// Choosing any x with 0 < x < n and n % x == 0.
// Replacing the number n on the chalkboard with n - x.
// Also, if a player cannot make a move, they lose the game.
//
// Return true if and only if Alice wins the game, assuming both players play optimally.
//
// Example 1:
//
// Input: n = 2
// Output: true
// Explanation: Alice chooses 1, and Bob has no more moves.
//
// Example 2:
//
// Input: n = 3
// Output: false
// Explanation: Alice chooses 1, Bob chooses 1, and Alice has no more moves.
//
// Constraints:
//
// - 1 <= n <= 1000
//

struct Solution;

impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        n % 2 == 0
    }
}

#[test]
fn test() {
    let cases = vec![(2, true), (3, false), (4, true), (5, false), (6, true)];
    for (n, expected) in cases {
        assert_eq!(Solution::divisor_game(n), expected);
    }
}
