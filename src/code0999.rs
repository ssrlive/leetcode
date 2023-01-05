#![allow(dead_code)]

// 999. Available Captures for Rook
// https://leetcode.com/problems/available-captures-for-rook/
// https://leetcode.cn/problems/available-captures-for-rook/
//
// On an 8 x 8 chessboard, there is exactly one white rook 'R' and some number of white bishops 'B', black pawns 'p', and empty squares '.'.
//
// When the rook moves, it chooses one of four cardinal directions (north, east, south, or west), then moves in that direction until it chooses to stop,
// reaches the edge of the board, captures a black pawn, or is blocked by a white bishop.
// A rook is considered attacking a pawn if the rook can capture the pawn on the rook's turn.
// The number of available captures for the white rook is the number of pawns that the rook is attacking.
//
// Return the number of available captures for the white rook.
//
// Example 1:
//
// Input: board = [[".",".",".",".",".",".",".","."],[".",".",".","p",".",".",".","."],[".",".",".","R",".",".",".","p"],
//                 [".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".","."],[".",".",".","p",".",".",".","."],
//                 [".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".","."]]
// Output: 3
// Explanation: In this example, the rook is attacking all the pawns.
//
// Example 2:
//
// Input: board = [[".",".",".",".",".",".",".","."],[".","p","p","p","p","p",".","."],[".","p","p","B","p","p",".","."],
//                 [".","p","B","R","B","p",".","."],[".","p","p","B","p","p",".","."],[".","p","p","p","p","p",".","."],
//                 [".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".","."]]
// Output: 0
// Explanation: The bishops are blocking the rook from attacking any of the pawns.
//
// Example 3:
//
// Input: board = [[".",".",".",".",".",".",".","."],[".",".",".","p",".",".",".","."],[".",".",".","p",".",".",".","."],
//                 ["p","p",".","R",".","p","B","."],[".",".",".",".",".",".",".","."],[".",".",".","B",".",".",".","."],
//                 [".",".",".","p",".",".",".","."],[".",".",".",".",".",".",".","."]]
// Output: 3
// Explanation: The rook is attacking the pawns at positions b5, d6, and f5.
//
// Constraints:
//
// - board.length == 8
// - board[i].length == 8
// - board[i][j] is either 'R', '.', 'B', or 'p'
// - There is exactly one cell with board[i][j] == 'R'
//

struct Solution;

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        fn calc_cap(it: impl Iterator<Item = char>, rook_pos: usize) -> i32 {
            let (mut res, mut num) = (0, 0);
            for (pos, fig) in it.enumerate() {
                match (fig, pos < rook_pos) {
                    ('R', _) => res += num,
                    ('p', true) => num = 1,
                    ('B', true) => num = 0,
                    ('p', false) => return res + 1,
                    ('B', false) => return res,
                    _ => (),
                }
            }
            res
        }

        for (y, row) in board.iter().enumerate() {
            if let Some(x) = row.iter().position(|c| *c == 'R') {
                let row_it = row.iter().cloned();
                let col_it = (0..8_usize).map(|i| board[i][x]);
                return calc_cap(row_it, x) + calc_cap(col_it, y);
            }
        }

        unreachable!()
    }
}

#[test]
fn test() {
    let board = vec![
        vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'R', '.', '.', '.', 'p'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.'],
    ];
    assert_eq!(Solution::num_rook_captures(board), 3);

    let board = vec![
        vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
        vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
        vec!['.', 'p', 'B', 'R', 'B', 'p', '.', '.'],
        vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
        vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.'],
    ];
    assert_eq!(Solution::num_rook_captures(board), 0);

    let board = vec![
        vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
        vec!['p', 'p', '.', 'R', '.', 'p', 'B', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.'],
    ];
    assert_eq!(Solution::num_rook_captures(board), 3);
}
