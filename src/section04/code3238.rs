#![allow(dead_code)]

// 3238. Find the Number of Winning Players
// https://leetcode.com/problems/find-the-number-of-winning-players/
// https://leetcode.cn/problems/find-the-number-of-winning-players/
//
// Easy
//
// You are given an integer n representing the number of players in a game and a 2D array pick
// where pick[i] = [xi, yi] represents that the player xi picked a ball of color yi.
//
// Player i wins the game if they pick strictly more than i balls of the same color. In other words,
//
//     Player 0 wins if they pick any ball.
//     Player 1 wins if they pick at least two balls of the same color.
//     ...
//     Player i wins if they pick at leasti + 1 balls of the same color.
//
// Return the number of players who win the game.
//
// Note that multiple players can win the game.
//
// Example 1:
//
// Input: n = 4, pick = [[0,0],[1,0],[1,0],[2,1],[2,1],[2,0]]
//
// Output: 2
//
// Explanation:
//
// Player 0 and player 1 win the game, while players 2 and 3 do not win.
//
// Example 2:
//
// Input: n = 5, pick = [[1,1],[1,2],[1,3],[1,4]]
//
// Output: 0
//
// Explanation:
//
// No player wins the game.
//
// Example 3:
//
// Input: n = 5, pick = [[1,1],[2,4],[2,4],[2,4]]
//
// Output: 1
//
// Explanation:
//
// Player 2 wins the game by picking 3 balls with color 4.
//
// Constraints:
//
//     2 <= n <= 10
//     1 <= pick.length <= 100
//     pick[i].length == 2
//     0 <= xi <= n - 1
//     0 <= yi <= 10
//

struct Solution;

impl Solution {
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        pick.into_iter()
            .fold(vec![[0; 11]; n as usize], |mut acc, p| {
                acc[p[0] as usize][p[1] as usize] += 1;
                acc
            })
            .into_iter()
            .enumerate()
            .fold(0, |acc, (i, p)| if p.into_iter().any(|v| v > i) { acc + 1 } else { acc })
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::winning_player_count(4, vec![vec![0, 0], vec![1, 0], vec![1, 0], vec![2, 1], vec![2, 1], vec![2, 0]]),
        2
    );
    assert_eq!(
        Solution::winning_player_count(5, vec![vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4]]),
        0
    );
    assert_eq!(
        Solution::winning_player_count(5, vec![vec![1, 1], vec![2, 4], vec![2, 4], vec![2, 4]]),
        1
    );
}
