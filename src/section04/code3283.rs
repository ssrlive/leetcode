#![allow(dead_code)]

// 3283. Maximum Number of Moves to Kill All Pawns
// https://leetcode.com/problems/maximum-number-of-moves-to-kill-all-pawns/
// https://leetcode.cn/problems/maximum-number-of-moves-to-kill-all-pawns/
//
// Hard
//
// There is a 50 x 50 chessboard with one knight and some pawns on it. You are given two integers kx and ky where (kx, ky) denotes
// the position of the knight, and a 2D array positions where positions[i] = [xi, yi] denotes the position of the pawns on the chessboard.
//
// Alice and Bob play a turn-based game, where Alice goes first. In each player's turn:
//
// - The player selects a pawn that still exists on the board and captures it with the knight in the fewest possible moves.
//   Note that the player can select any pawn, it might not be one that can be captured in the least number of moves.
// - In the process of capturing the selected pawn, the knight may pass other pawns without capturing them.
//   Only the selected pawn can be captured in this turn.
//
// Alice is trying to maximize the sum of the number of moves made by both players until there are
// no more pawns on the board, whereas Bob tries to minimize them.
//
// Return the maximum total number of moves made during the game that Alice can achieve, assuming both players play optimally.
//
// Note that in one move, a chess knight has eight possible positions it can move to, as illustrated below.
// Each move is two cells in a cardinal direction, then one cell in an orthogonal direction.
//
// Example 1:
//
// Input: kx = 1, ky = 1, positions = [[0,0]]
//
// Output: 4
//
// Explanation:
//
// The knight takes 4 moves to reach the pawn at (0, 0).
//
// Example 2:
//
// Input: kx = 0, ky = 2, positions = [[1,1],[2,2],[3,3]]
//
// Output: 8
//
// Explanation:
//
//     Alice picks the pawn at (2, 2) and captures it in two moves: (0, 2) -> (1, 4) -> (2, 2).
//     Bob picks the pawn at (3, 3) and captures it in two moves: (2, 2) -> (4, 1) -> (3, 3).
//     Alice picks the pawn at (1, 1) and captures it in four moves: (3, 3) -> (4, 1) -> (2, 2) -> (0, 3) -> (1, 1).
//
// Example 3:
//
// Input: kx = 0, ky = 0, positions = [[1,2],[2,4]]
//
// Output: 3
//
// Explanation:
//
//     Alice picks the pawn at (2, 4) and captures it in two moves: (0, 0) -> (1, 2) -> (2, 4). Note that the pawn at (1, 2) is not captured.
//     Bob picks the pawn at (1, 2) and captures it in one move: (2, 4) -> (1, 2).
//
// Constraints:
//
//     0 <= kx, ky <= 49
//     1 <= positions.length <= 15
//     positions[i].length == 2
//     0 <= positions[i][0], positions[i][1] <= 49
//     All positions[i] are unique.
//     The input is generated such that positions[i] != [kx, ky] for all 0 <= i < positions.length.
//

struct Solution;

impl Solution {
    pub fn max_moves(kx: i32, ky: i32, positions: Vec<Vec<i32>>) -> i32 {
        let pos: Vec<_> = positions.iter().map(|p| (p[0], p[1])).chain(std::iter::once((kx, ky))).collect();
        let n = pos.len() - 1;

        let d = [(1, 2), (-1, 2), (1, -2), (-1, -2), (2, 1), (-2, 1), (2, -1), (-2, -1)];
        let mut s = Vec::new();
        for i in 0..pos.len() - 1 {
            let mut cur = vec![0; pos.len() - i - 1];
            let mut left = cur.len();
            let mut prev = vec![pos[i]];
            let mut seen = vec![vec![false; 50]; 50];
            seen[pos[i].0 as usize][pos[i].1 as usize] = true;
            let mut step = 0;
            while left > 0 {
                let mut nxt = Vec::new();
                for &p in &prev {
                    for j in i + 1..pos.len() {
                        if pos[j] == p {
                            cur[j - 1 - i] = step;
                            left -= 1;
                            break;
                        }
                    }
                    for &dr in &d {
                        let (nx, ny) = (p.0 + dr.0, p.1 + dr.1);
                        if (0..50).contains(&nx) && (0..50).contains(&ny) && !seen[nx as usize][ny as usize] {
                            nxt.push((nx, ny));
                            seen[nx as usize][ny as usize] = true;
                        }
                    }
                }
                step += 1;
                prev = nxt;
            }
            s.push(cur);
        }

        let mut dp = vec![-1; (n + 1) * (1 << n)];
        for i in 0..n {
            dp[i * (1 << n)] = 0;
        }
        Self::dp((n + 1) * (1 << n) - 1, n, true, &mut dp, &s)
    }

    fn dp(mark: usize, n: usize, is_alice: bool, dp: &mut Vec<i32>, s: &Vec<Vec<i32>>) -> i32 {
        if dp[mark] < 0 {
            let (mask, cur) = (mark % (1 << n), mark / (1 << n));
            dp[mark] = if is_alice {
                (0..n)
                    .filter(|i| mask & (1 << i) != 0)
                    .map(|i| {
                        let (a, b) = (i.min(cur), i.max(cur));
                        Self::dp(i * (1 << n) + (mask - (1 << i)), n, !is_alice, dp, s) + s[a][b - a - 1]
                    })
                    .max()
                    .unwrap()
            } else {
                (0..n)
                    .filter(|i| mask & (1 << i) != 0)
                    .map(|i| {
                        let (a, b) = (i.min(cur), i.max(cur));
                        Self::dp(i * (1 << n) + (mask - (1 << i)), n, !is_alice, dp, s) + s[a][b - a - 1]
                    })
                    .min()
                    .unwrap()
            };
        }
        dp[mark]
    }
}

#[test]
fn test() {
    let kx = 1;
    let ky = 1;
    let positions = vec![vec![0, 0]];
    assert_eq!(Solution::max_moves(kx, ky, positions), 4);

    let kx = 0;
    let ky = 2;
    let positions = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
    assert_eq!(Solution::max_moves(kx, ky, positions), 8);

    let kx = 0;
    let ky = 0;
    let positions = vec![vec![1, 2], vec![2, 4]];
    assert_eq!(Solution::max_moves(kx, ky, positions), 3);
}
