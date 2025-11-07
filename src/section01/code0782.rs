#![allow(dead_code)]

// 782. Transform to Chessboard
// https://leetcode.com/problems/transform-to-chessboard/
// https://leetcode.cn/problems/transform-to-chessboard/
//
// You are given an n x n binary grid board. In each move, you can swap any two rows with each other, or any two columns with each other.
//
// Return the minimum number of moves to transform the board into a chessboard board. If the task is impossible, return -1.
//
// A chessboard board is a board where no 0's and no 1's are 4-directionally adjacent.
//
// Example 1:
//
// Input: board = [[0,1,1,0],[0,1,1,0],[1,0,0,1],[1,0,0,1]]
// Output: 2
// Explanation: One potential sequence of moves is shown.
// The first move swaps the first and second column.
// The second move swaps the second and third row.
//
// Example 2:
//
// Input: board = [[0,1],[1,0]]
// Output: 0
// Explanation: Also note that the board with 0 in the top left corner, is also a valid chessboard.
//
// Example 3:
//
// Input: board = [[1,0],[1,0]]
// Output: -1
// Explanation: No matter what sequence of moves you make, you cannot end with a valid chessboard.
//
// Constraints:
//
// - n == board.length
// - n == board[i].length
// - 2 <= n <= 30
// - board[i][j] is either 0 or 1.
//

struct Solution;

impl Solution {
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let mut row = 0;
        let mut col = 0;
        let mut row_diff = 0;
        let mut col_diff = 0;
        for i in 0..n {
            for j in 0..n {
                if board[0][0] ^ board[0][j] ^ board[i][0] ^ board[i][j] == 1 {
                    return -1;
                }
            }
        }
        for (i, (&row_val, &col_val)) in board[0].iter().zip(board.iter().map(|r| &r[0])).enumerate() {
            row += row_val;
            col += col_val;
            if col_val == i as i32 % 2 {
                row_diff += 1;
            }
            if row_val == i as i32 % 2 {
                col_diff += 1;
            }
        }
        if row < n as i32 / 2 || row > (n as i32 + 1) / 2 {
            return -1;
        }
        if col < n as i32 / 2 || col > (n as i32 + 1) / 2 {
            return -1;
        }
        if n % 2 == 1 {
            if row_diff % 2 == 1 {
                row_diff = n - row_diff;
            }
            if col_diff % 2 == 1 {
                col_diff = n - col_diff;
            }
        } else {
            row_diff = std::cmp::min(row_diff, n - row_diff);
            col_diff = std::cmp::min(col_diff, n - col_diff);
        }
        ((row_diff + col_diff) / 2) as i32
    }
}

#[test]
fn test() {
    let board = vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![1, 0, 0, 1], vec![1, 0, 0, 1]];
    assert_eq!(Solution::moves_to_chessboard(board), 2);
    assert_eq!(Solution::moves_to_chessboard(vec![vec![0, 1], vec![1, 0]]), 0);
    assert_eq!(Solution::moves_to_chessboard(vec![vec![1, 0], vec![1, 0]]), -1);
}
