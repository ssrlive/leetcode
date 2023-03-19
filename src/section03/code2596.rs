#![allow(dead_code)]

/*

// 2596. Check Knight Tour Configuration
// https://leetcode.com/problems/check-knight-tour-configuration/
// https://leetcode.cn/problems/check-knight-tour-configuration/
//
// Medium
//
// There is a knight on an n x n chessboard. In a valid configuration, the knight starts at the top-left cell of the board and visits every cell on the board exactly once.

You are given an n x n integer matrix grid consisting of distinct integers from the range [0, n * n - 1] where grid[row][col] indicates that the cell (row, col) is the grid[row][col]th cell that the knight visited. The moves are 0-indexed.

Return true if grid represents a valid configuration of the knight's movements or false otherwise.

Note that a valid knight move consists of moving two squares vertically and one square horizontally, or two squares horizontally and one square vertically. The figure below illustrates all the possible eight moves of a knight from some cell.

Example 1:

Input: grid = [[0,11,16,5,20],[17,4,19,10,15],[12,1,8,21,6],[3,18,23,14,9],[24,13,2,7,22]]
Output: true
Explanation: The above diagram represents the grid. It can be shown that it is a valid configuration.

Example 2:

Input: grid = [[0,3,6],[5,8,1],[2,7,4]]
Output: false
Explanation: The above diagram represents the grid. The 8th move of the knight is not valid considering its position after the 7th move.

Constraints:

    n == grid.length == grid[i].length
    3 <= n <= 7
    0 <= grid[row][col] < n * n
    All integers in grid are unique.
*/

struct Solution;

impl Solution {
    pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        if grid[0][0] != 0 {
            return false;
        }
        let mut m = vec![[0, 0]; 7 * 7];
        for x in 0..n {
            for y in 0..n {
                m[grid[x][y] as usize] = [x as i32, y as i32];
            }
        }
        for i in 1..n * n {
            let dx = (m[i][0] - m[i - 1][0]).abs();
            let dy = (m[i][1] - m[i - 1][1]).abs();
            if dx.min(dy) != 1 || dx.max(dy) != 2 {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let grid = vec![
        vec![0, 11, 16, 5, 20],
        vec![17, 4, 19, 10, 15],
        vec![12, 1, 8, 21, 6],
        vec![3, 18, 23, 14, 9],
        vec![24, 13, 2, 7, 22],
    ];
    assert_eq!(Solution::check_valid_grid(grid), true);

    let grid = vec![vec![0, 3, 6], vec![5, 8, 1], vec![2, 7, 4]];
    assert_eq!(Solution::check_valid_grid(grid), false);
}
