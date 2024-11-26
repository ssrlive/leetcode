#![allow(dead_code)]

// 3360. Stone Removal Game
// https://leetcode.com/problems/stone-removal-game/
// https://leetcode.cn/problems/stone-removal-game/
//
// Easy
//
// Alice and Bob are playing a game where they take turns removing stones from a pile, with Alice going first.
//
// Alice starts by removing exactly 10 stones on her first turn.
// For each subsequent turn, each player removes exactly 1 fewer stone than the previous opponent.
// The player who cannot make a move loses the game.
//
// Given a positive integer n, return true if Alice wins the game and false otherwise.
//
// Example 1:
//
// Input: n = 12
//
// Output: true
//
// Explanation:
//
// Alice removes 10 stones on her first turn, leaving 2 stones for Bob.
// Bob cannot remove 9 stones, so Alice wins.
//
// Example 2:
//
// Input: n = 1
//
// Output: false
//
// Explanation:
//
// Alice cannot remove 10 stones, so Alice loses.
//
// Constraints:
//
// 1 <= n <= 50
//

struct Solution;

impl Solution {
    pub fn can_alice_win(n: i32) -> bool {
        let mut n = n;
        let mut is_alice = false;
        for i in (1..=10).rev() {
            if n < i {
                return is_alice;
            }
            n -= i;
            is_alice = !is_alice;
        }
        false
    }
}

#[test]
fn test() {
    let result = true;
    let n = 12;
    assert_eq!(Solution::can_alice_win(n), result);

    let result = false;
    let n = 1;
    assert_eq!(Solution::can_alice_win(n), result);
}
