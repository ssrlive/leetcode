#![allow(dead_code)]

// 1162. As Far from Land as Possible
// https://leetcode.com/problems/as-far-from-land-as-possible/
// https://leetcode.cn/problems/as-far-from-land-as-possible/
//
// Given an n x n grid containing only values 0 and 1, where 0 represents water and 1 represents land,
// find a water cell such that its distance to the nearest land cell is maximized, and return the distance.
// If no land or water exists in the grid, return -1.
//
// The distance used in this problem is the Manhattan distance: the distance
// between two cells (x0, y0) and (x1, y1) is |x0 - x1| + |y0 - y1|.
//
// Example 1:
//
// Input: grid = [[1,0,1],[0,0,0],[1,0,1]]
// Output: 2
// Explanation: The cell (1, 1) is as far as possible from all the land with distance 2.
//
// Example 2:
//
// Input: grid = [[1,0,0],[0,0,0],[0,0,0]]
// Output: 4
// Explanation: The cell (2, 2) is as far as possible from all the land with distance 4.
//
// Constraints:
//
// - n == grid.length
// - n == grid[i].length
// - 1 <= n <= 100
// - grid[i][j] is 0 or 1
//

struct Solution;

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        const WATER: i32 = 0;
        const LAND: i32 = 1;

        let mut dist = vec![vec![i32::MAX; grid[0].len()]; grid.len()];

        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                match grid[r][c] {
                    LAND => dist[r][c] = 0,
                    WATER => {
                        if r > 0 && dist[r - 1][c] != i32::MAX {
                            dist[r][c] = dist[r][c].min(dist[r - 1][c] + 1);
                        }

                        if c > 0 && dist[r][c - 1] != i32::MAX {
                            dist[r][c] = dist[r][c].min(dist[r][c - 1] + 1);
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        for r in (0..grid.len()).rev() {
            for c in (0..grid[r].len()).rev() {
                match grid[r][c] {
                    LAND => dist[r][c] = 0,
                    WATER => {
                        if r < grid.len() - 1 && dist[r + 1][c] != i32::MAX {
                            dist[r][c] = dist[r][c].min(dist[r + 1][c] + 1);
                        }

                        if c < grid[r].len() - 1 && dist[r][c + 1] != i32::MAX {
                            dist[r][c] = dist[r][c].min(dist[r][c + 1] + 1);
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        dist.into_iter()
            .flat_map(|x| x.into_iter())
            .max()
            .map(|x| match x {
                0 | i32::MAX => -1,
                x => x,
            })
            .unwrap()
    }
}

#[test]
fn test() {
    let grid = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
    assert_eq!(Solution::max_distance(grid), 2);

    let grid = vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
    assert_eq!(Solution::max_distance(grid), 4);
}
