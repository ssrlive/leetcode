#![allow(dead_code)]

// 130. Surrounded Regions
// https://leetcode.com/problems/surrounded-regions/
// https://leetcode.cn/problems/surrounded-regions/
//
// Given an m x n matrix board containing 'X' and 'O', capture all regions that are 4-directionally surrounded by 'X'.
//
// A region is captured by flipping all 'O's into 'X's in that surrounded region.
//

struct Solution;

impl Solution {
    pub fn solve(board: &mut [Vec<char>]) {
        if board.is_empty() {
            return;
        }
        let m = board.len();
        let n = board[0].len();
        let mut visited = vec![vec![false; n]; m];
        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 'O' && !visited[i][j] {
                    let mut queue = std::collections::VecDeque::new();
                    queue.push_back((i, j));
                    let mut is_surrounded = true;
                    let mut region = vec![];
                    while let Some((i, j)) = queue.pop_front() {
                        if visited[i][j] {
                            continue;
                        }
                        visited[i][j] = true;
                        region.push((i, j));
                        if i == 0 || i == m - 1 || j == 0 || j == n - 1 {
                            is_surrounded = false;
                        }
                        if i > 0 && board[i - 1][j] == 'O' {
                            queue.push_back((i - 1, j));
                        }
                        if i < m - 1 && board[i + 1][j] == 'O' {
                            queue.push_back((i + 1, j));
                        }
                        if j > 0 && board[i][j - 1] == 'O' {
                            queue.push_back((i, j - 1));
                        }
                        if j < n - 1 && board[i][j + 1] == 'O' {
                            queue.push_back((i, j + 1));
                        }
                    }
                    if is_surrounded {
                        for (i, j) in region {
                            board[i][j] = 'X';
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_solve() {
    let mut board = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    Solution::solve(&mut board);
    assert_eq!(
        board,
        vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ]
    );
}
