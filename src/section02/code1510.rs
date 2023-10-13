#![allow(dead_code)]

/*

// 1510. Stone Game IV
// https://leetcode.com/problems/stone-game-iv/
// https://leetcode.cn/problems/stone-game-iv/
//
// Hard
//
// Alice and Bob take turns playing a game, with Alice starting first.

Initially, there are n stones in a pile. On each player's turn, that player makes a move consisting
of removing any non-zero square number of stones in the pile.

Also, if a player cannot make a move, he/she loses the game.

Given a positive integer n, return true if and only if Alice wins the game otherwise return false, assuming both players play optimally.

Example 1:

Input: n = 1
Output: true
Explanation: Alice can remove 1 stone winning the game because Bob doesn't have any moves.

Example 2:

Input: n = 2
Output: false
Explanation: Alice can only remove 1 stone, after that Bob removes the last one winning the game (2 -> 1 -> 0).

Example 3:

Input: n = 4
Output: true
Explanation: n is already a perfect square, Alice can win with one move, removing 4 stones (4 -> 0).

Constraints:

    1 <= n <= 10^5
*/

struct Solution;

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let n = n as usize;
        let mut dp = vec![false; n + 1];
        for i in 1..=n {
            let mut j = 1;
            while j * j <= i {
                if !dp[i - j * j] {
                    dp[i] = true;
                    break;
                }
                j += 1;
            }
        }
        dp[n]
    }
}

#[test]
fn test() {
    assert!(Solution::winner_square_game(1));
    assert!(!Solution::winner_square_game(2));
    assert!(Solution::winner_square_game(4));
    assert!(!Solution::winner_square_game(7));
    assert!(!Solution::winner_square_game(17));
    assert!(Solution::winner_square_game(100000));
}
