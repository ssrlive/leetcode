#![allow(dead_code)]

/*

// 1594. Maximum Non Negative Product in a Matrix
//
// Medium
//
// You are given a m x n matrix grid. Initially, you are located at the top-left corner (0, 0), and in each step, you can only move right or down in the matrix.

Among all possible paths starting from the top-left corner (0, 0) and ending in the bottom-right corner (m - 1, n - 1), find the path with the maximum non-negative product. The product of a path is the product of all integers in the grid cells visited along the path.

Return the maximum non-negative product modulo 109 + 7. If the maximum product is negative, return -1.

Notice that the modulo is performed after getting the maximum product.

Example 1:

Input: grid = [[-1,-2,-3],[-2,-3,-3],[-3,-3,-2]]
Output: -1
Explanation: It is not possible to get non-negative product in the path from (0, 0) to (2, 2), so return -1.

Example 2:

Input: grid = [[1,-2,1],[1,-2,1],[3,-4,1]]
Output: 8
Explanation: Maximum non-negative product is shown (1 * 1 * -2 * -4 * 1 = 8).

Example 3:

Input: grid = [[1,3],[0,-4]]
Output: 0
Explanation: Maximum non-negative product is shown (1 * 0 * -4 = 0).

Constraints:

    m == grid.length
    n == grid[i].length
    1 <= m, n <= 15
    -4 <= grid[i][j] <= 4
*/

struct Solution;

impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let grid: Vec<Vec<_>> = grid.iter().map(|row| row.iter().map(|&x| x as i64).collect()).collect();
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![vec![0; 2]; n]; m];
        dp[0][0][0] = grid[0][0];
        dp[0][0][1] = grid[0][0];
        for i in 1..m {
            dp[i][0][0] = dp[i - 1][0][0] * grid[i][0];
            dp[i][0][1] = dp[i - 1][0][1] * grid[i][0];
        }
        for j in 1..n {
            dp[0][j][0] = dp[0][j - 1][0] * grid[0][j];
            dp[0][j][1] = dp[0][j - 1][1] * grid[0][j];
        }
        for i in 1..m {
            for j in 1..n {
                let a = dp[i - 1][j][0] * grid[i][j];
                let b = dp[i - 1][j][1] * grid[i][j];
                let c = dp[i][j - 1][0] * grid[i][j];
                let d = dp[i][j - 1][1] * grid[i][j];
                dp[i][j][0] = a.max(b).max(c).max(d);
                dp[i][j][1] = a.min(b).min(c).min(d);
            }
        }
        if dp[m - 1][n - 1][0] < 0 {
            -1
        } else {
            (dp[m - 1][n - 1][0] % 1_000_000_007) as _
        }
    }
}

#[test]
fn test() {
    let grid = vec![vec![-1, -2, -3], vec![-2, -3, -3], vec![-3, -3, -2]];
    assert_eq!(Solution::max_product_path(grid), -1);
    let grid = vec![vec![1, -2, 1], vec![1, -2, 1], vec![3, -4, 1]];
    assert_eq!(Solution::max_product_path(grid), 8);
    let grid = vec![vec![1, 3], vec![0, -4]];
    assert_eq!(Solution::max_product_path(grid), 0);
}
