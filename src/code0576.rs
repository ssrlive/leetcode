#![allow(dead_code)]

// 576. Out of Boundary Paths
// https://leetcode.com/problems/out-of-boundary-paths/
//
// There is an m x n grid with a ball. The ball is initially at the position [startRow, startColumn].
// You are allowed to move the ball to one of the four adjacent cells in the grid (possibly out of the grid crossing the grid boundary).
// You can apply at most maxMove moves to the ball.
//
// Given the five integers m, n, maxMove, startRow, startColumn, return the number of paths to move
// the ball out of the grid boundary. Since the answer can be very large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: m = 2, n = 2, maxMove = 2, startRow = 0, startColumn = 0
// Output: 6
//
// Example 2:
//
// Input: m = 1, n = 3, maxMove = 3, startRow = 0, startColumn = 1
// Output: 12
//
// Constraints:
//
// - 1 <= m, n <= 50
// - 0 <= maxMove <= 50
// - 0 <= startRow < m
// - 0 <= startColumn < n
//

struct Solution;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut grid = std::collections::HashMap::new();
        grid.insert((start_row, start_column), 1);
        let mut answer = 0;
        for _ in 0..max_move {
            for ((row, col), k) in grid.drain().collect::<Vec<_>>() {
                if row == 0 {
                    answer = (answer + k) % MOD
                }
                if row == m - 1 {
                    answer = (answer + k) % MOD
                }
                if col == 0 {
                    answer = (answer + k) % MOD
                }
                if col == n - 1 {
                    answer = (answer + k) % MOD
                }
                for d in [0, 1, 0, -1, 0].windows(2) {
                    let (row, col) = (row + d[0], col + d[1]);
                    if (0..m).contains(&row) && (0..n).contains(&col) {
                        let v = grid.entry((row, col)).or_default();
                        *v = (*v + k) % MOD;
                    }
                }
            }
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_paths(2, 2, 2, 0, 0), 6);
    assert_eq!(Solution::find_paths(1, 3, 3, 0, 1), 12);
}
