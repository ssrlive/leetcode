#![allow(dead_code)]

// 883. Projection Area of 3D Shapes
// https://leetcode.com/problems/projection-area-of-3d-shapes/
// https://leetcode.cn/problems/projection-area-of-3d-shapes/
//
// You are given an n x n grid where we place some 1 x 1 x 1 cubes that are axis-aligned with the x, y, and z axes.
//
// Each value v = grid[i][j] represents a tower of v cubes placed on top of the cell (i, j).
//
// We view the projection of these cubes onto the xy, yz, and zx planes.
//
// A projection is like a shadow, that maps our 3-dimensional figure to a 2-dimensional plane. We are viewing the "shadow" when looking at the cubes from the top, the front, and the side.
//
// Return the total area of all three projections.
//
// Example 1:
//
// Input: grid = [[1,2],[3,4]]
// Output: 17
// Explanation: Here are the three projections ("shadows") of the shape made with each axis-aligned plane.
//
// Example 2:
//
// Input: grid = [[2]]
// Output: 5
//
// Example 3:
//
// Input: grid = [[1,0],[0,2]]
// Output: 8
//
// Constraints:
//
// - n == grid.length == grid[i].length
// - 1 <= n <= 50
// - 0 <= grid[i][j] <= 50
//

struct Solution;

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut max_in_cols = vec![0; grid.len()];
        for row in &grid {
            let mut max_in_row = 0;
            for (col, &val) in row.iter().enumerate().filter(|(_, val)| **val > 0) {
                max_in_row = max_in_row.max(val);
                max_in_cols[col] = max_in_cols[col].max(val);
                res += 1;
            }
            res += max_in_row;
        }
        res + max_in_cols.iter().sum::<i32>()
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 2], vec![3, 4]], 17),
        (vec![vec![2]], 5),
        (vec![vec![1, 0], vec![0, 2]], 8),
    ];
    for (grid, expected) in cases {
        assert_eq!(Solution::projection_area(grid), expected);
    }
}
