#![allow(dead_code)]

// 3142. Check if Grid Satisfies Conditions
// https://leetcode.com/problems/check-if-grid-satisfies-conditions/
// https://leetcode.cn/problems/check-if-grid-satisfies-conditions/
//
// Easy
//
// You are given a 2D matrix grid of size m x n. You need to check if each cell grid[i][j] is:
//
// Equal to the cell below it, i.e. grid[i][j] == grid[i + 1][j] (if it exists).
// Different from the cell to its right, i.e. grid[i][j] != grid[i][j + 1] (if it exists).
// Return true if all the cells satisfy these conditions, otherwise, return false.
//
// Example 1:
//
// Input: grid = [[1,0,2],[1,0,2]]
//
// Output: true
//
// Explanation:
//
// All the cells in the grid satisfy the conditions.
//
// Example 2:
//
// Input: grid = [[1,1,1],[0,0,0]]
//
// Output: false
//
// Explanation:
//
// All cells in the first row are equal.
//
// Example 3:
//
// Input: grid = [[1],[2],[3]]
//
// Output: false
//
// Explanation:
//
// Cells in the first column have different values.
//
// Constraints:
//
// 1 <= n, m <= 10
// 0 <= grid[i][j] <= 9
//

struct Solution;

impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        grid[0].windows(2).all(|w| w[0] != w[1]) && grid.windows(2).all(|w| w[0] == w[1])
    }
}

#[test]
fn test() {
    let grid = vec![vec![1, 0, 2], vec![1, 0, 2]];
    assert!(Solution::satisfies_conditions(grid));

    let grid = vec![vec![1, 1, 1], vec![0, 0, 0]];
    assert!(!Solution::satisfies_conditions(grid));

    let grid = vec![vec![1], vec![2], vec![3]];
    assert!(!Solution::satisfies_conditions(grid));
}
