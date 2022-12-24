#![allow(dead_code)]

// 529. Minesweeper
// https://leetcode.com/problems/minesweeper/
// https://leetcode.cn/problems/minesweeper/
//
// Let's play the minesweeper game (Wikipedia, online game)!
//
// You are given an m x n char matrix board representing the game board where:
//
// - 'M' represents an unrevealed mine,
// - 'E' represents an unrevealed empty square,
// - 'B' represents a revealed blank square that has no adjacent mines (i.e., above, below, left, right, and all 4 diagonals),
// - digit ('1' to '8') represents how many mines are adjacent to this revealed square, and
// - 'X' represents a revealed mine.
//
// You are also given an integer array click where click = [clickr, clickc] represents
// the next click position among all the unrevealed squares ('M' or 'E').
//
// Return the board after revealing this position according to the following rules:
//
// If a mine 'M' is revealed, then the game is over. You should change it to 'X'.
// If an empty square 'E' with no adjacent mines is revealed, then change it to a revealed blank 'B' and
// all of its adjacent unrevealed squares should be revealed recursively.
// If an empty square 'E' with at least one adjacent mine is revealed, then change it
// to a digit ('1' to '8') representing the number of adjacent mines.
// Return the board when no more squares will be revealed.
//
// Example 1:
//
// Input: board = [["E","E","E","E","E"],["E","E","M","E","E"],["E","E","E","E","E"],["E","E","E","E","E"]], click = [3,0]
// Output: [["B","1","E","1","B"],["B","1","M","1","B"],["B","1","1","1","B"],["B","B","B","B","B"]]
//
// Example 2:
//
// Input: board = [["B","1","E","1","B"],["B","1","M","1","B"],["B","1","1","1","B"],["B","B","B","B","B"]], click = [1,2]
// Output: [["B","1","E","1","B"],["B","1","X","1","B"],["B","1","1","1","B"],["B","B","B","B","B"]]
//
// Constraints:
//
// - m == board.length
// - n == board[i].length
// - 1 <= m, n <= 50
// - board[i][j] is either 'M', 'E', 'B', or a digit from '1' to '8'.
// - click.length == 2
// - 0 <= clickr < m
// - 0 <= clickc < n
// - board[clickr][clickc] is either 'M' or 'E'.
//

struct Solution;

impl Solution {
    pub fn update_board(board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        let mut board = board;
        let (r, c) = (click[0] as usize, click[1] as usize);
        if board[r][c] == 'M' {
            board[r][c] = 'X';
        } else {
            Self::dfs(&mut board, r, c);
        }
        board
    }

    fn dfs(board: &mut Vec<Vec<char>>, r: usize, c: usize) {
        let mut count = 0;
        for i in r.saturating_sub(1)..=r + 1 {
            for j in c.saturating_sub(1)..=c + 1 {
                if i < board.len() && j < board[0].len() && board[i][j] == 'M' {
                    count += 1;
                }
            }
        }
        if count > 0 {
            board[r][c] = (count as u8 + b'0') as char;
        } else {
            board[r][c] = 'B';
            for i in r.saturating_sub(1)..=r + 1 {
                for j in c.saturating_sub(1)..=c + 1 {
                    if i < board.len() && j < board[0].len() && board[i][j] == 'E' && (i != r || j != c) {
                        Self::dfs(board, i, j);
                    }
                }
            }
        }
    }
}

#[test]
fn test() {
    let board = vec![
        vec!['E', 'E', 'E', 'E', 'E'],
        vec!['E', 'E', 'M', 'E', 'E'],
        vec!['E', 'E', 'E', 'E', 'E'],
        vec!['E', 'E', 'E', 'E', 'E'],
    ];
    let click = vec![3, 0];
    let result = Solution::update_board(board, click);
    let expected = vec![
        vec!['B', '1', 'E', '1', 'B'],
        vec!['B', '1', 'M', '1', 'B'],
        vec!['B', '1', '1', '1', 'B'],
        vec!['B', 'B', 'B', 'B', 'B'],
    ];
    assert_eq!(result, expected);

    let board = vec![
        vec!['B', '1', 'E', '1', 'B'],
        vec!['B', '1', 'M', '1', 'B'],
        vec!['B', '1', '1', '1', 'B'],
        vec!['B', 'B', 'B', 'B', 'B'],
    ];
    let click = vec![1, 2];
    let result = Solution::update_board(board, click);
    let expected = vec![
        vec!['B', '1', 'E', '1', 'B'],
        vec!['B', '1', 'X', '1', 'B'],
        vec!['B', '1', '1', '1', 'B'],
        vec!['B', 'B', 'B', 'B', 'B'],
    ];
    assert_eq!(result, expected);
}
