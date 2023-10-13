#![allow(dead_code)]

// 877. Stone Game
// https://leetcode.com/problems/stone-game/
// https://leetcode.cn/problems/stone-game/
//
// Alice and Bob play a game with piles of stones. There are an even number of piles arranged in a row,
// and each pile has a positive integer number of stones piles[i].
//
// The objective of the game is to end with the most stones.
// The total number of stones across all the piles is odd, so there are no ties.
//
// Alice and Bob take turns, with Alice starting first. Each turn, a player takes the entire pile of stones either
// from the beginning or from the end of the row. This continues until there are no more piles left,
// at which point the person with the most stones wins.
//
// Assuming Alice and Bob play optimally, return true if Alice wins the game, or false if Bob wins.
//
// Example 1:
//
// Input: piles = [5,3,4,5]
// Output: true
// Explanation:
// Alice starts first, and can only take the first 5 or the last 5.
// Say she takes the first 5, so that the row becomes [3, 4, 5].
// If Bob takes 3, then the board is [4, 5], and Alice takes 5 to win with 10 points.
// If Bob takes the last 5, then the board is [3, 4], and Alice takes 4 to win with 9 points.
// This demonstrated that taking the first 5 was a winning move for Alice, so we return true.
//
// Example 2:
//
// Input: piles = [3,7,2,3]
// Output: true
//
// Constraints:
//
// - 2 <= piles.length <= 500
// - piles.length is even.
// - 1 <= piles[i] <= 500
// - sum(piles[i]) is odd.
//

struct Solution;

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let mut alex: i32 = 0;
        let mut lee: i32 = 0;
        let mut p = piles;
        let step: i32 = 1;
        while !p.is_empty() {
            let mut mx = p[0];
            mx = mx.max(p[p.len() - 1]);
            let idx = p.iter().position(|&x| x == mx).unwrap();
            if step % 2 == 0 {
                lee += mx;
            } else {
                alex += mx;
            }
            p.remove(idx);
        }
        alex > lee
    }
}

#[test]
fn test() {
    assert!(Solution::stone_game(vec![5, 3, 4, 5]));
    assert!(Solution::stone_game(vec![3, 7, 2, 3]));
}
