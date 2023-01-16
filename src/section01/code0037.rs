#![allow(dead_code)]

// 37. Sudoku Solver
// https://leetcode.com/problems/sudoku-solver/
// https://leetcode.cn/problems/sudoku-solver/
//
// Write a program to solve a Sudoku puzzle by filling the empty cells.
//
// A sudoku solution must satisfy all of the following rules:
//
// Each of the digits 1-9 must occur exactly once in each row.
// Each of the digits 1-9 must occur exactly once in each column.
// Each of the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the grid.
// The '.' character indicates empty cells.
//
// Example 1:
//
// Input: board = [["5","3",".",".","7",".",".",".","."],
//                 ["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],
//                 ["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],
//                 ["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],
//                 [".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
// Output: [["5","3","4","6","7","8","9","1","2"],["6","7","2","1","9","5","3","4","8"],
//          ["1","9","8","3","4","2","5","6","7"],["8","5","9","7","6","1","4","2","3"],
//          ["4","2","6","8","5","3","7","9","1"],["7","1","3","9","2","4","8","5","6"],
//          ["9","6","1","5","3","7","2","8","4"],["2","8","7","4","1","9","6","3","5"],
//          ["3","4","5","2","8","6","1","7","9"]]
// Explanation: The input board is shown above and the only valid solution is shown below:
//
// Constraints:
//
// - board.length == 9
// - board[i].length == 9
// - board[i][j] is a digit or '.'.
// - It is guaranteed that the input board has only one solution.
//

struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        fn check(board: &[Vec<char>], i: usize, j: usize) -> u16 {
            let mut res = 0;
            for n in 0..9 {
                let rc = board[i][n];
                let cc = board[n][j];
                let sc = board[i / 3 * 3 + n / 3][(j / 3 * 3) + (n % 3)];
                mask(&mut res, rc);
                mask(&mut res, cc);
                mask(&mut res, sc);
            }
            res
        }

        #[inline]
        fn mask(x: &mut u16, c: char) {
            match c {
                '.' => {}
                c => {
                    *x |= 1 << (c.to_digit(10).unwrap());
                }
            }
        }

        fn solve_sudoku_helper(board: &mut Vec<Vec<char>>, n: usize) -> bool {
            if n == 81 {
                return true;
            }

            let (i, j) = (n / 9, n % 9);
            if board[i][j] != '.' {
                return solve_sudoku_helper(board, n + 1);
            }

            let mask = check(board, i, j);
            for b in 1..=9 {
                if (mask >> b) & 1 == 1 {
                    continue;
                }
                board[i][j] = std::char::from_digit(b, 10).unwrap();
                if solve_sudoku_helper(board, n + 1) {
                    return true;
                }
                board[i][j] = '.';
            }
            false
        }

        solve_sudoku_helper(board, 0);
    }
}

#[test]
fn test() {
    let mut board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    let expected = vec![
        vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
        vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
        vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
        vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
        vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
        vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
        vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
        vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
        vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
    ];
    Solution::solve_sudoku(&mut board);
    assert_eq!(board, expected);
}
