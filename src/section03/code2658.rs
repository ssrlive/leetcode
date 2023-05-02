#![allow(dead_code)]

/*
// 2658. Maximum Number of Fish in a Grid
// https://leetcode.com/problems/maximum-number-of-fish-in-a-grid/
// https://leetcode.cn/problems/maximum-number-of-fish-in-a-grid/
//
// Medium
//
// You are given a 0-indexed 2D matrix grid of size m x n, where (r, c) represents:

    A land cell if grid[r][c] = 0, or
    A water cell containing grid[r][c] fish, if grid[r][c] > 0.

A fisher can start at any water cell (r, c) and can do the following operations any number of times:

    Catch all the fish at cell (r, c), or
    Move to any adjacent water cell.

Return the maximum number of fish the fisher can catch if he chooses his starting cell optimally, or 0 if no water cell exists.

An adjacent cell of the cell (r, c), is one of the cells (r, c + 1), (r, c - 1), (r + 1, c) or (r - 1, c) if it exists.

Example 1:

Input: grid = [[0,2,1,0],[4,0,0,3],[1,0,0,4],[0,3,2,0]]
Output: 7
Explanation: The fisher can start at cell (1,3) and collect 3 fish, then move to cell (2,3) and collect 4 fish.

Example 2:

Input: grid = [[1,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,1]]
Output: 1
Explanation: The fisher can start at cells (0,0) or (3,3) and collect a single fish.

Constraints:

    m == grid.length
    n == grid[i].length
    1 <= m, n <= 10
    0 <= grid[i][j] <= 10
*/

struct Solution;

impl Solution {
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
            if i.min(j) < 0 {
                return 0;
            }
            if i >= grid.len() as i32 || j >= grid[0].len() as i32 || grid[i as usize][j as usize] == 0 {
                return 0;
            };
            let val = grid[i as usize][j as usize];
            grid[i as usize][j as usize] = 0;

            dfs(grid, i + 1, j) + dfs(grid, i - 1, j) + dfs(grid, i, j + 1) + dfs(grid, i, j - 1) + val
        }

        let mut grid = grid;
        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] > 0 {
                    ans = ans.max(dfs(&mut grid, i as i32, j as i32));
                }
            }
        }
        ans
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (vec![vec![0, 2, 1, 0], vec![4, 0, 0, 3], vec![1, 0, 0, 4], vec![0, 3, 2, 0]], 7),
        (vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 1]], 1),
    ];
    for (grid, expect) in cases {
        assert_eq!(Solution::find_max_fish(grid), expect);
    }
}
