#![allow(dead_code)]

// 3665. Twisted Mirror Path Count
// https://leetcode.com/problems/twisted-mirror-path-count/
// https://leetcode.cn/problems/twisted-mirror-path-count/
//
// Medium
//
// Given an m x n binary grid grid where:
//
// grid[i][j] == 0 represents an empty cell, and
// grid[i][j] == 1 represents a mirror.
// A robot starts at the top-left corner of the grid (0, 0) and wants to reach the bottom-right corner (m - 1, n - 1).
// It can move only right or down. If the robot attempts to move into a mirror cell, it is reflected before entering that cell:
//
// If it tries to move right into a mirror, it is turned down and moved into the cell directly below the mirror.
// If it tries to move down into a mirror, it is turned right and moved into the cell directly to the right of the mirror.
// If this reflection would cause the robot to move outside the grid boundaries, the path is considered invalid and should not be counted.
//
// Return the number of unique valid paths from (0, 0) to (m - 1, n - 1).
//
// Since the answer may be very large, return it modulo 109 + 7.
//
// Note: If a reflection moves the robot into a mirror cell, the robot is immediately reflected again based
// on the direction it used to enter that mirror: if it entered while moving right, it will be turned down;
// if it entered while moving down, it will be turned right. This process will continue until either the
// last cell is reached, the robot moves out of bounds or the robot moves to a non-mirror cell.
//
// Example 1:
//
// Input: grid = [[0,1,0],[0,0,1],[1,0,0]]
//
// Output: 5
//
// Explanation:
//
// Number	Full Path
// 1	(0, 0) → (0, 1) [M] → (1, 1) → (1, 2) [M] → (2, 2)
// 2	(0, 0) → (0, 1) [M] → (1, 1) → (2, 1) → (2, 2)
// 3	(0, 0) → (1, 0) → (1, 1) → (1, 2) [M] → (2, 2)
// 4	(0, 0) → (1, 0) → (1, 1) → (2, 1) → (2, 2)
// 5	(0, 0) → (1, 0) → (2, 0) [M] → (2, 1) → (2, 2)
// [M] indicates the robot attempted to enter a mirror cell and instead reflected.
//
// Example 2:
//
// Input: grid = [[0,0],[0,0]]
//
// Output: 2
//
// Explanation:
//
// Number	Full Path
// 1	(0, 0) → (0, 1) → (1, 1)
// 2	(0, 0) → (1, 0) → (1, 1)
//
// Example 3:
//
// Input: grid = [[0,1,1],[1,1,0]]
//
// Output: 1
//
// Explanation:
//
// Number	Full Path
// 1	(0, 0) → (0, 1) [M] → (1, 1) [M] → (1, 2)
// (0, 0) → (1, 0) [M] → (1, 1) [M] → (2, 1) goes out of bounds, so it is invalid.
//
// Constraints:
//
// m == grid.length
// n == grid[i].length
// 2 <= m, n <= 500
// grid[i][j] is either 0 or 1.
// grid[0][0] == grid[m - 1][n - 1] == 0
//

struct Solution;

impl Solution {
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &Vec<Vec<i32>>, i: usize, j: usize, dp: &mut Vec<Vec<Vec<i32>>>, n: usize, m: usize, dir: usize) -> i32 {
            let mod_val = 1_000_000_007;
            if i >= n || j >= m {
                return 0;
            }
            if i == n - 1 && j == m - 1 {
                return 1;
            }
            if dp[i][j][dir] != -1 {
                return dp[i][j][dir];
            }
            if grid[i][j] == 1 {
                if dir == 1 {
                    return dfs(grid, i + 1, j, dp, n, m, 0);
                } else if dir == 0 {
                    return dfs(grid, i, j + 1, dp, n, m, 1);
                }
            }
            let mut move_count: i64 = 0;
            if j + 1 < m {
                move_count += dfs(grid, i, j + 1, dp, n, m, 1) as i64;
            }
            if i + 1 < n {
                move_count += dfs(grid, i + 1, j, dp, n, m, 0) as i64;
            }
            dp[i][j][dir] = (move_count % mod_val) as i32;
            dp[i][j][dir]
        }

        let n = grid.len();
        let m = grid[0].len();
        let mut dp = vec![vec![vec![-1; 2]; m]; n];
        dfs(&grid, 0, 0, &mut dp, n, m, 0)
    }
}

#[test]
fn test() {
    let grid = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 0, 0]];
    assert_eq!(Solution::unique_paths(grid), 5);

    let grid = vec![vec![0, 0], vec![0, 0]];
    assert_eq!(Solution::unique_paths(grid), 2);

    let grid = vec![vec![0, 1, 1], vec![1, 1, 0]];
    assert_eq!(Solution::unique_paths(grid), 1);
}
