#![allow(dead_code)]

// 64. Minimum Path Sum
// https://leetcode.com/problems/minimum-path-sum/
// https://leetcode.cn/problems/minimum-path-sum/
//
// Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right, which minimizes the sum of all numbers along its path.
//
// Note: You can only move either down or right at any point in time.
//
// Example 1:
//
// Input: grid = [[1,3,1],[1,5,1],[4,2,1]]
// Output: 7
// Explanation: Because the path 1 → 3 → 1 → 1 → 1 minimizes the sum.
//
// Example 2:
//
// Input: grid = [[1,2,3],[4,5,6]]
// Output: 12
//
// Constraints:
//
// - m == grid.length
// - n == grid[i].length
// - 1 <= m, n <= 200
// - 0 <= grid[i][j] <= 100
//

struct Solution;

impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        for j in 1..n {
            grid[0][j] += grid[0][j - 1];
        }
        for i in 1..m {
            grid[i][0] += grid[i - 1][0];
            for j in 1..n {
                grid[i][j] += grid[i][j - 1].min(grid[i - 1][j]);
            }
        }
        grid[m - 1][n - 1]
    }
}

#[test]
fn test() {
    let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    assert_eq!(Solution::min_path_sum(grid), 7);

    let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
    assert_eq!(Solution::min_path_sum(grid), 12);
}
