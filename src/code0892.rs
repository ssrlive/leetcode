#![allow(dead_code)]

// 892. Surface Area of 3D Shapes
// https://leetcode.com/problems/surface-area-of-3d-shapes/
// https://leetcode.cn/problems/surface-area-of-3d-shapes/
//
// You are given an n x n grid where you have placed some 1 x 1 x 1 cubes. Each value v = grid[i][j] represents a tower of v cubes placed on top of cell (i, j).
//
// After placing these cubes, you have decided to glue any directly adjacent cubes to each other, forming several irregular 3D shapes.
//
// Return the total surface area of the resulting shapes.
//
// Note: The bottom face of each shape counts toward its surface area.
//
// Example 1:
//
// Input: grid = [[1,2],[3,4]]
// Output: 34
//
// Example 2:
//
// Input: grid = [[1,1,1],[1,0,1],[1,1,1]]
// Output: 32
//
// Example 3:
//
// Input: grid = [[2,2,2],[2,1,2],[2,2,2]]
// Output: 46
//
// Constraints:
//
// - n == grid.length == grid[i].length
// - 1 <= n <= 50
// - 0 <= grid[i][j] <= 50
//

struct Solution;

impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let l = grid.len() as i32;
        let mut res = 0;

        for (row, i) in grid.iter().zip(0..) {
            for (&val, j) in row.iter().zip(0..).filter(|(val, _)| **val > 0) {
                res += 2;
                for (dx, dy) in dirs.iter().cloned() {
                    let (xx, yy) = (j + dx, i + dy);
                    res += match xx >= 0 && yy >= 0 && xx < l && yy < l {
                        true => (val - grid[yy as usize][xx as usize]).max(0),
                        false => val,
                    };
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let grid = vec![vec![1, 2], vec![3, 4]];
    assert_eq!(Solution::surface_area(grid), 34);

    let grid = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    assert_eq!(Solution::surface_area(grid), 32);

    let grid = vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]];
    assert_eq!(Solution::surface_area(grid), 46);
}
