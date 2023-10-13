#![allow(dead_code)]

// 794. Valid Tic-Tac-Toe State
// https://leetcode.com/problems/valid-tic-tac-toe-state/
// https://leetcode.cn/problems/valid-tic-tac-toe-state/
//
// Given a Tic-Tac-Toe board as a string array board, return true if and only if it is possible
// to reach this board position during the course of a valid tic-tac-toe game.
//
// The board is a 3 x 3 array that consists of characters ' ', 'X', and 'O'.
// The ' ' character represents an empty square.
//
// Here are the rules of Tic-Tac-Toe:
//
// Players take turns placing characters into empty squares ' '.
// The first player always places 'X' characters, while the second player always places 'O' characters.
// 'X' and 'O' characters are always placed into empty squares, never filled ones.
// The game ends when there are three of the same (non-empty) character filling any row, column, or diagonal.
// The game also ends if all squares are non-empty.
// No more moves can be played if the game is over.
//
// Example 1:
//
// Input: board = ["O  ","   ","   "]
// Output: false
// Explanation: The first player always plays "X".
//
// Example 2:
//
// Input: board = ["XOX"," X ","   "]
// Output: false
// Explanation: Players take turns making moves.
//
// Example 3:
//
// Input: board = ["XOX","O O","XOX"]
// Output: true
//
// Constraints:
//
// - board.length == 3
// - board[i].length == 3
// - board[i][j] is either 'X', 'O', or ' '.
//

struct Solution;

impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        fn _valid_tic_tac_toe(board: Vec<String>) -> Option<bool> {
            let mut x_count = 0;
            let mut o_count = 0;
            let mut x_wins = false;
            let mut o_wins = false;
            for row in board.iter() {
                for c in row.chars() {
                    if c == 'X' {
                        x_count += 1;
                    } else if c == 'O' {
                        o_count += 1;
                    }
                }
            }
            if x_count < o_count || x_count > o_count + 1 {
                return Some(false);
            }
            for i in 0..3 {
                if board[i].chars().all(|c| c == 'X') {
                    x_wins = true;
                }
                if board[i].chars().all(|c| c == 'O') {
                    o_wins = true;
                }
                if board[0].chars().nth(i)? == board[1].chars().nth(i)? && board[1].chars().nth(i)? == board[2].chars().nth(i)? {
                    if board[0].chars().nth(i)? == 'X' {
                        x_wins = true;
                    } else if board[0].chars().nth(i)? == 'O' {
                        o_wins = true;
                    }
                }
            }
            if board[0].chars().next()? == board[1].chars().nth(1)? && board[1].chars().nth(1)? == board[2].chars().nth(2)? {
                if board[0].starts_with('X') {
                    x_wins = true;
                } else if board[0].starts_with('O') {
                    o_wins = true;
                }
            }
            if board[0].chars().nth(2)? == board[1].chars().nth(1)? && board[1].chars().nth(1)? == board[2].chars().next()? {
                if board[0].chars().nth(2)? == 'X' {
                    x_wins = true
                } else if board[0].chars().nth(2)? == 'O' {
                    o_wins = true;
                }
            }
            if x_wins && o_wins {
                return Some(false);
            }
            if x_wins && x_count == o_count {
                return Some(false);
            }
            if o_wins && x_count > o_count {
                return Some(false);
            }
            Some(true)
        }

        _valid_tic_tac_toe(board).unwrap_or(false)
    }
}

#[test]
fn test() {
    let board = ["O  ", "   ", "   "];
    let board = board.iter().map(|s| s.to_string()).collect();
    assert!(!Solution::valid_tic_tac_toe(board));

    let board = ["XOX", " X ", "   "];
    let board = board.iter().map(|s| s.to_string()).collect();
    assert!(!Solution::valid_tic_tac_toe(board));

    let board = ["XOX", "O O", "XOX"];
    let board = board.iter().map(|s| s.to_string()).collect();
    assert!(Solution::valid_tic_tac_toe(board));
}
