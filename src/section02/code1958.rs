#![allow(dead_code)]

/*

// 1958. Check if Move is Legal
// https://leetcode.com/problems/check-if-move-is-legal/
// https://leetcode.cn/problems/check-if-move-is-legal/
//
// Medium
//
// You are given a 0-indexed 8 x 8 grid board, where board[r][c] represents the cell (r, c) on a game board. On the board, free cells are represented by '.', white cells are represented by 'W', and black cells are represented by 'B'.

Each move in this game consists of choosing a free cell and changing it to the color you are playing as (either white or black). However, a move is only legal if, after changing it, the cell becomes the endpoint of a good line (horizontal, vertical, or diagonal).

A good line is a line of three or more cells (including the endpoints) where the endpoints of the line are one color, and the remaining cells in the middle are the opposite color (no cells in the line are free). You can find examples for good lines in the figure below:

Given two integers rMove and cMove and a character color representing the color you are playing as (white or black), return true if changing cell (rMove, cMove) to color color is a legal move, or false if it is not legal.

Example 1:

Input: board = [[".",".",".","B",".",".",".","."],[".",".",".","W",".",".",".","."],[".",".",".","W",".",".",".","."],[".",".",".","W",".",".",".","."],["W","B","B",".","W","W","W","B"],[".",".",".","B",".",".",".","."],[".",".",".","B",".",".",".","."],[".",".",".","W",".",".",".","."]], rMove = 4, cMove = 3, color = "B"
Output: true
Explanation: '.', 'W', and 'B' are represented by the colors blue, white, and black respectively, and cell (rMove, cMove) is marked with an 'X'.
The two good lines with the chosen cell as an endpoint are annotated above with the red rectangles.

Example 2:

Input: board = [[".",".",".",".",".",".",".","."],[".","B",".",".","W",".",".","."],[".",".","W",".",".",".",".","."],[".",".",".","W","B",".",".","."],[".",".",".",".",".",".",".","."],[".",".",".",".","B","W",".","."],[".",".",".",".",".",".","W","."],[".",".",".",".",".",".",".","B"]], rMove = 4, cMove = 4, color = "W"
Output: false
Explanation: While there are good lines with the chosen cell as a middle cell, there are no good lines with the chosen cell as an endpoint.

Constraints:

    board.length == board[r].length == 8
    0 <= rMove, cMove < 8
    board[rMove][cMove] == '.'
    color is either 'B' or 'W'.
*/

struct Solution;

impl Solution {
    pub fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
        let x_arr = [0, 0, 1, -1, 1, -1, -1, 1];
        let y_arr = [1, -1, 0, 0, 1, -1, 1, -1];
        for i in 0..8 {
            let mut x = r_move + x_arr[i];
            let mut y = c_move + y_arr[i];
            let mut count = 0;
            while x < board.len() as i32 && x >= 0 && y < board[i].len() as i32 && y >= 0 {
                if board[x as usize][y as usize] == '.' {
                    break;
                } else if board[x as usize][y as usize] != color {
                    count += 1;
                } else if board[x as usize][y as usize] == color && count < 1 {
                    break;
                } else if board[x as usize][y as usize] == color && count >= 1 {
                    return true;
                }
                x += x_arr[i];
                y += y_arr[i];
            }
        }
        false
    }
}

#[test]
fn test() {
    let board = vec![
        vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
        vec!['W', 'B', 'B', '.', 'W', 'W', 'W', 'B'],
        vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
    ];
    let r_move = 4;
    let c_move = 3;
    let color = 'B';
    let res = true;
    assert_eq!(Solution::check_move(board, r_move, c_move, color), res);

    let board = vec![
        vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', 'B', '.', '.', 'W', '.', '.', '.'],
        vec!['.', '.', 'W', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'W', 'B', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', 'B', 'W', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', 'W', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', 'B'],
    ];
    let r_move = 4;
    let c_move = 4;
    let color = 'W';
    let res = false;
    assert_eq!(Solution::check_move(board, r_move, c_move, color), res);
}
