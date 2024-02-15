#![allow(dead_code)]

// 289. Game of Life
// https://leetcode.com/problems/game-of-life/
// https://leetcode.cn/problems/game-of-life/
//
// According to Wikipedia's article: "The Game of Life, also known simply as Life, is a cellular automaton devised by the British mathematician John Horton Conway in 1970."
//
// The board is made up of an m x n grid of cells, where each cell has an initial state: live (represented by a 1) or dead (represented by a 0). Each cell interacts with its eight neighbors (horizontal, vertical, diagonal) using the following four rules (taken from the above Wikipedia article):
//
// - Any live cell with fewer than two live neighbors dies as if caused by under-population.
// - Any live cell with two or three live neighbors lives on to the next generation.
// - Any live cell with more than three live neighbors dies, as if by over-population.
// - Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
//
// The next state is created by applying the above rules simultaneously to every cell in the current state, where births and deaths occur simultaneously. Given the current state of the m x n grid board, return the next state.
//
// Example 1:
//
// Input: board = [[0,1,0],[0,0,1],[1,1,1],[0,0,0]]
// Output: [[0,0,0],[1,0,1],[0,1,1],[0,1,0]]
//
// Example 2:
//
// Input: board = [[1,1],[1,0]]
// Output: [[1,1],[1,1]]
//
// Constraints:
//
// - m == board.length
// - n == board[i].length
// - 1 <= m, n <= 25
// - board[i][j] is 0 or 1.
//
// Follow up:
//
// - Could you solve it in-place? Remember that the board needs to be updated simultaneously: You cannot update some cells first and then use their updated values to update other cells.
// - In this question, we represent the board using a 2D array. In principle, the board is infinite, which would cause problems when the active area encroaches upon the border of the array (i.e., live cells reach the border). How would you address these problems?
//

struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut [Vec<i32>]) {
        let (m, n) = (board.len(), board[0].len());
        let mut next = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                let mut c = 0;
                for (di, dj) in &[(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)] {
                    let (ni, nj) = (i as i32 + di, j as i32 + dj);
                    if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                        c += board[ni as usize][nj as usize];
                    }
                }
                if board[i][j] == 1 {
                    if c == 2 || c == 3 {
                        next[i][j] = 1;
                    }
                } else if c == 3 {
                    next[i][j] = 1;
                }
            }
        }
        for i in 0..m {
            for j in 0..n {
                board[i][j] = next[i][j];
            }
        }
    }
}

#[test]
fn test_game_of_life() {
    let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
    Solution::game_of_life(&mut board);
    assert_eq!(board, vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]]);
}
