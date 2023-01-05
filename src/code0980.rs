#![allow(dead_code)]

// 980. Unique Paths III
// https://leetcode.com/problems/unique-paths-iii/
// https://leetcode.cn/problems/unique-paths-iii/
//
// You are given an m x n integer array grid where grid[i][j] could be:
//
// - 1 representing the starting square. There is exactly one starting square.
// - 2 representing the ending square. There is exactly one ending square.
// - 0 representing empty squares we can walk over.
// - -1 representing obstacles that we cannot walk over.
//
// Return the number of 4-directional walks from the starting square to the ending square,
// that walk over every non-obstacle square exactly once.
//
// Example 1:
//
// Input: grid = [[1,0,0,0],[0,0,0,0],[0,0,2,-1]]
// Output: 2
// Explanation: We have the following two paths:
// 1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2)
// 2. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2)
//
// Example 2:
//
// Input: grid = [[1,0,0,0],[0,0,0,0],[0,0,0,2]]
// Output: 4
// Explanation: We have the following four paths:
// 1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2),(2,3)
// 2. (0,0),(0,1),(1,1),(1,0),(2,0),(2,1),(2,2),(1,2),(0,2),(0,3),(1,3),(2,3)
// 3. (0,0),(1,0),(2,0),(2,1),(2,2),(1,2),(1,1),(0,1),(0,2),(0,3),(1,3),(2,3)
// 4. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2),(2,3)
//
// Example 3:
//
// Input: grid = [[0,1],[2,0]]
// Output: 0
// Explanation: There is no path that walks over every empty square exactly once.
// Note that the starting and ending square can be anywhere in the grid.
//
// Constraints:
//
// - m == grid.length
// - n == grid[i].length
// - 1 <= m, n <= 20
// - 1 <= m * n <= 20
// - -1 <= grid[i][j] <= 2
// - There is exactly one starting cell and one ending cell.
//

struct Solution;

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        fn helper(grid: &mut Vec<Vec<i32>>, pos: (usize, usize), count: usize) -> i32 {
            if grid[pos.0][pos.1] == 2 {
                return i32::from(count == 0);
            }
            grid[pos.0][pos.1] = 1;
            let ret = if pos.0 > 0 && grid[pos.0 - 1][pos.1].abs() != 1 {
                helper(grid, (pos.0 - 1, pos.1), count - 1)
            } else {
                0
            } + if pos.1 > 0 && grid[pos.0][pos.1 - 1].abs() != 1 {
                helper(grid, (pos.0, pos.1 - 1), count - 1)
            } else {
                0
            } + if pos.0 < grid.len() - 1 && grid[pos.0 + 1][pos.1].abs() != 1 {
                helper(grid, (pos.0 + 1, pos.1), count - 1)
            } else {
                0
            } + if pos.1 < grid[0].len() - 1 && grid[pos.0][pos.1 + 1].abs() != 1 {
                helper(grid, (pos.0, pos.1 + 1), count - 1)
            } else {
                0
            };
            grid[pos.0][pos.1] = 0;
            ret
        }

        let mut grid = grid;
        let mut pos = (0, 0);
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                match grid[i][j] {
                    0 => count += 1,
                    1 => pos = (i, j),
                    _ => {}
                }
            }
        }
        helper(&mut grid, pos, count + 1)
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]], 2),
        (vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]], 4),
        (vec![vec![0, 1], vec![2, 0]], 0),
    ];
    for (grid, expected) in cases {
        assert_eq!(Solution::unique_paths_iii(grid), expected);
    }
}
