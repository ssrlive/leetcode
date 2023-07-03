#![allow(dead_code)]

/*

// 1568. Minimum Number of Days to Disconnect Island
// https://leetcode.com/problems/minimum-number-of-days-to-disconnect-island/
// https://leetcode.cn/problems/minimum-number-of-days-to-disconnect-island/
//
// Hard
//
// You are given an m x n binary grid grid where 1 represents land and 0 represents water. An island is a maximal 4-directionally (horizontal or vertical) connected group of 1's.

The grid is said to be connected if we have exactly one island, otherwise is said disconnected.

In one day, we are allowed to change any single land cell (1) into a water cell (0).

Return the minimum number of days to disconnect the grid.

Example 1:

Input: grid = [[0,1,1,0],[0,1,1,0],[0,0,0,0]]

Output: 2
Explanation: We need at least 2 days to get a disconnected grid.
Change land grid[1][1] and grid[0][2] to water and get 2 disconnected island.

Example 2:

Input: grid = [[1,1]]
Output: 2
Explanation: Grid of full water is also disconnected ([[1,1]] -> [[0,0]]), 0 islands.

Constraints:

    m == grid.length
    n == grid[i].length
    1 <= m, n <= 30
    grid[i][j] is either 0 or 1.
*/

struct Solution;

impl Solution {
    pub fn min_days(grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(x: usize, y: usize, grid: &Vec<Vec<i32>>, vis: &mut Vec<Vec<i32>>) {
            let (dx, dy) = (vec![1, -1, 0, 0], vec![0, 0, 1, -1]);
            let (n, m) = (grid.len(), grid[0].len());
            vis[x][y] = 1;
            for a in 0..4 {
                let nx = x as i32 + dx[a];
                let ny = y as i32 + dy[a];
                if nx >= 0
                    && ny >= 0
                    && nx < n as i32
                    && ny < m as i32
                    && vis[nx as usize][ny as usize] == 0
                    && grid[nx as usize][ny as usize] == 1
                {
                    dfs(nx as usize, ny as usize, grid, vis);
                }
            }
        }

        fn count_islands(grid: &Vec<Vec<i32>>) -> i32 {
            let mut islands = 0;
            let (n, m) = (grid.len(), grid[0].len());
            let mut vis = vec![vec![0; m]; n];
            for i in 0..n {
                for j in 0..m {
                    if vis[i][j] == 0 && grid[i][j] == 1 {
                        dfs(i, j, grid, &mut vis);
                        islands += 1;
                    }
                }
            }
            islands
        }

        let mut grid = grid;
        let mut islands = count_islands(&grid);
        let (n, m) = (grid.len(), grid[0].len());
        if islands > 1 || islands == 0 {
            return 0;
        } else {
            for i in 0..n {
                for j in 0..m {
                    if grid[i][j] == 1 {
                        grid[i][j] = 0;
                        islands = count_islands(&grid);
                        grid[i][j] = 1;
                        if islands > 1 || islands == 0 {
                            return 1;
                        }
                    }
                }
            }
        }
        2
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]], 2),
        (vec![vec![1, 1]], 2),
        (vec![vec![1, 0, 1, 0]], 0),
        (
            vec![vec![1, 1, 0, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 0, 1, 1], vec![1, 1, 0, 1, 1]],
            1,
        ),
        (
            vec![vec![1, 1, 0, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 0, 1, 1], vec![1, 1, 1, 1, 1]],
            2,
        ),
        (vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]], 1),
    ];
    for (grid, expect) in cases {
        assert_eq!(Solution::min_days(grid), expect);
    }
}
