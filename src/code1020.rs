#![allow(dead_code)]

// 1020. Number of Enclaves
// https://leetcode.com/problems/number-of-enclaves/
// https://leetcode.cn/problems/number-of-enclaves/
//
// You are given an m x n binary matrix grid, where 0 represents a sea cell and 1 represents a land cell.
//
// A move consists of walking from one land cell to another adjacent (4-directionally) land cell or walking off the boundary of the grid.
//
// Return the number of land cells in grid for which we cannot walk off the boundary of the grid in any number of moves.
//
// Example 1:
//
// Input: grid = [[0,0,0,0],[1,0,1,0],[0,1,1,0],[0,0,0,0]]
// Output: 3
// Explanation: There are three 1s that are enclosed by 0s, and one 1 that is not enclosed because its on the boundary.
//
// Example 2:
//
// Input: grid = [[0,1,1,0],[0,0,1,0],[0,0,1,0],[0,0,0,0]]
// Output: 0
// Explanation: All 1s are either on the boundary or can reach the boundary.
//
// Constraints:
//
// - m == grid.length
// - n == grid[i].length
// - 1 <= m, n <= 500
// - grid[i][j] is either 0 or 1.
//

struct Solution;

impl Solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        fn bfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) {
            if grid[i][j] == 0 {
                return;
            }
            let mut q = Vec::new();
            q.push((i, j));
            while let Some((x, y)) = q.pop() {
                grid[x][y] = 0;
                for (dx, dy) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                    let nx = (x as i32 + dx) as usize;
                    let ny = (y as i32 + dy) as usize;
                    if nx < grid.len() && ny < grid[0].len() && grid[nx][ny] == 1 {
                        q.push((nx, ny));
                    }
                }
            }
        }

        let mut grid = grid;

        let m = grid.len();
        let n = grid[0].len();
        for j in 0..n {
            bfs(&mut grid, 0, j);
            bfs(&mut grid, m - 1, j);
        }
        for i in 0..m {
            bfs(&mut grid, i, 0);
            bfs(&mut grid, i, n - 1);
        }
        grid.iter().fold(0, |acc, row| row.iter().sum::<i32>() + acc)
    }
}

#[test]
fn test() {
    let grid = vec![vec![0, 0, 0, 0], vec![1, 0, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]];
    assert_eq!(Solution::num_enclaves(grid), 3);

    let grid = vec![vec![0, 1, 1, 0], vec![0, 0, 1, 0], vec![0, 0, 1, 0], vec![0, 0, 0, 0]];
    assert_eq!(Solution::num_enclaves(grid), 0);
}
