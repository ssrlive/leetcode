#![allow(dead_code)]

// 2556. Disconnect Path in a Binary Matrix by at Most One Flip
// https://leetcode.com/problems/disconnect-path-in-a-binary-matrix-by-at-most-one-flip/
// https://leetcode.cn/problems/disconnect-path-in-a-binary-matrix-by-at-most-one-flip/
//
// Medium
//
// You are given a 0-indexed m x n binary matrix grid. You can move from a cell (row, col) to any
// of the cells (row + 1, col) or (row, col + 1) that has the value 1.
// The matrix is disconnected if there is no path from (0, 0) to (m - 1, n - 1).
//
// You can flip the value of at most one (possibly none) cell. You cannot flip the cells (0, 0) and (m - 1, n - 1).
//
// Return true if it is possible to make the matrix disconnect or false otherwise.
//
// Note that flipping a cell changes its value from 0 to 1 or from 1 to 0.
//
// Example 1:
//
// Input: grid = [[1,1,1],[1,0,0],[1,1,1]]
// Output: true
// Explanation: We can change the cell shown in the diagram above. There is no path from (0, 0) to (2, 2) in the resulting grid.
//
// Example 2:
//
// Input: grid = [[1,1,1],[1,0,1],[1,1,1]]
// Output: false
// Explanation: It is not possible to change at most one cell such that there is not path from (0, 0) to (2, 2).
//
// Constraints:
//
// -    m == grid.length
// -    n == grid[i].length
// -    1 <= m, n <= 1000
// -    1 <= m * n <= 10^5
// -    grid[i][j] is either 0 or 1.
// -    grid[0][0] == grid[m - 1][n - 1] == 1
//

struct Solution;

impl Solution {
    pub fn is_possible_to_cut_path(grid: Vec<Vec<i32>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut order = vec![vec![0; n]; m];

        let mut seq = 1;
        Self::dfs(&grid, &mut order, 0, 0, &mut seq);

        let mut ret = false;
        let val = order[m - 1][n - 1];
        Self::dfs_reverse(&grid, &mut order, 0, 0, val, &mut ret);
        !ret
    }

    fn dfs(grid: &Vec<Vec<i32>>, order: &mut Vec<Vec<i32>>, i: i32, j: i32, seq: &mut i32) {
        if i == grid.len() as i32 || j == grid[0].len() as i32 {
            return;
        }
        if grid[i as usize][j as usize] == 0 || order[i as usize][j as usize] != 0 {
            return;
        }

        *seq += 1;
        order[i as usize][j as usize] = *seq;
        Self::dfs(grid, order, i + 1, j, seq);
        Self::dfs(grid, order, i, j + 1, seq);
    }

    fn dfs_reverse(grid: &Vec<Vec<i32>>, order: &mut Vec<Vec<i32>>, i: i32, j: i32, val: i32, ret: &mut bool) {
        if i == grid.len() as i32 || j == grid[0].len() as i32 {
            return;
        }
        if grid[i as usize][j as usize] == 0 || order[i as usize][j as usize] == 0 {
            return;
        }

        if i + j > 0 && order[i as usize][j as usize] < val {
            return;
        }

        order[i as usize][j as usize] = 0;
        if i == grid.len() as i32 - 1 && j == grid[0].len() as i32 - 1 {
            *ret = true;
            return;
        }

        Self::dfs_reverse(grid, order, i, j + 1, val, ret);
        if !(*ret) {
            Self::dfs_reverse(grid, order, i + 1, j, val, ret);
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 1, 1], vec![1, 0, 0], vec![1, 1, 1]], true),
        (vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]], false),
        (
            vec![
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 1, 1, 1, 1, 1, 1, 0, 1],
                vec![1, 0, 1, 0, 0, 0, 0, 1, 0, 1],
                vec![1, 0, 1, 0, 1, 1, 1, 1, 0, 1],
                vec![1, 0, 1, 0, 1, 0, 0, 0, 0, 1],
                vec![1, 0, 1, 0, 1, 0, 1, 1, 1, 1],
                vec![1, 0, 1, 0, 1, 0, 1, 0, 0, 1],
                vec![1, 0, 1, 1, 1, 0, 1, 0, 1, 1],
                vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            ],
            false,
        ),
        (
            vec![
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 1, 1, 1, 1, 1, 1, 0, 1],
                vec![1, 0, 1, 0, 0, 0, 0, 1, 0, 1],
                vec![1, 0, 1, 0, 1, 1, 1, 1, 0, 1],
                vec![1, 0, 1, 0, 1, 0, 0, 0, 0, 1],
                vec![1, 0, 1, 0, 1, 0, 1, 1, 1, 1],
                vec![1, 0, 1, 0, 1, 0, 1, 0, 0, 1],
                vec![1, 0, 1, 1, 1, 0, 1, 0, 1, 1],
                vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            ],
            false,
        ),
    ];
    for (grid, expected) in cases {
        assert_eq!(Solution::is_possible_to_cut_path(grid), expected);
    }
}
