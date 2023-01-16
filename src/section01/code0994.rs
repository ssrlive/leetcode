#![allow(dead_code)]

// 994. Rotting Oranges
// https://leetcode.com/problems/rotting-oranges/
// https://leetcode.cn/problems/rotting-oranges/
//
// You are given an m x n grid where each cell can have one of three values:
//
// 0 representing an empty cell,
// 1 representing a fresh orange, or
// 2 representing a rotten orange.
// Every minute, any fresh orange that is 4-directionally adjacent to a rotten orange becomes rotten.
//
// Return the minimum number of minutes that must elapse until no cell has a fresh orange. If this is impossible, return -1.
//
// Example 1:
//
// Input: grid = [[2,1,1],[1,1,0],[0,1,1]]
// Output: 4
//
// Example 2:
//
// Input: grid = [[2,1,1],[0,1,1],[1,0,1]]
// Output: -1
// Explanation: The orange in the bottom left corner (row 2, column 0) is never rotten, because rotting only happens 4-directionally.
//
// Example 3:
//
// Input: grid = [[0,2]]
// Output: 0
// Explanation: Since there are already no fresh oranges at minute 0, the answer is just 0.
//
// Constraints:
//
// - m == grid.length
// - n == grid[i].length
// - 1 <= m, n <= 10
// - grid[i][j] is 0, 1, or 2.
//

struct Solution;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let m = grid.len();
        let n = grid[0].len();
        let mut rotten = vec![];
        let mut rottened = vec![];

        let mut fresh = 0;
        for (i, row) in grid.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                match *item {
                    2 => {
                        rotten.push((i, j));
                    }
                    1 => {
                        fresh += 1;
                    }
                    _ => (),
                }
            }
        }

        let mut time = 0;
        loop {
            for (i, j) in rotten.iter() {
                for (r, c) in [0, 1, 0, !0, 0]
                    .windows(2)
                    .map(|deltas| (i.wrapping_add(deltas[0]), j.wrapping_add(deltas[1])))
                    .filter(|(r, c)| *r < m && *c < n)
                {
                    if grid[r][c] == 1 {
                        grid[r][c] = 2;
                        fresh -= 1;
                        rottened.push((r, c));
                    }
                }
            }
            if rottened.is_empty() {
                break;
            } else {
                time += 1;
                rotten = rottened;
                rottened = vec![];
            }
        }

        if fresh > 0 {
            -1
        } else {
            time
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]], 4),
        (vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]], -1),
        (vec![vec![0, 2]], 0),
    ];
    for (grid, expected) in cases {
        assert_eq!(Solution::oranges_rotting(grid), expected);
    }
}
