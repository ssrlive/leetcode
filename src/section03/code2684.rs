#![allow(dead_code)]

// 2684. Maximum Number of Moves in a Grid
// https://leetcode.com/problems/maximum-number-of-moves-in-a-grid/
// https://leetcode.cn/problems/maximum-number-of-moves-in-a-grid/
//
// Medium
//
// You are given a 0-indexed m x n matrix grid consisting of positive integers.
//
// You can start at any cell in the first column of the matrix, and traverse the grid in the following way:
//
//     - From a cell (row, col), you can move to any of the cells: (row - 1, col + 1), (row, col + 1)
//       and (row + 1, col + 1) such that the value of the cell you move to, should be strictly bigger than
//       the value of the current cell.
//
// Return the maximum number of moves that you can perform.
//
// Example 1:
//
// Input: grid = [[2,4,3,5],[5,4,9,3],[3,4,2,11],[10,9,13,15]]
// Output: 3
// Explanation: We can start at the cell (0, 0) and make the following moves:
// - (0, 0) -> (0, 1).
// - (0, 1) -> (1, 2).
// - (1, 2) -> (2, 3).
// It can be shown that it is the maximum number of moves that can be made.
//
// Example 2:
//
// Input: grid = [[3,2,4],[2,1,9],[1,1,7]]
// Output: 0
// Explanation: Starting from any cell in the first column we cannot perform any moves.
//
// Constraints:
//
//     m == grid.length
//     n == grid[i].length
//     2 <= m, n <= 1000
//     4 <= m * n <= 10^5
//     1 <= grid[i][j] <= 10^6
//

struct Solution;

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![0; n]; m];
        let mut ret = 0;

        for j in 1..n {
            for i in 0..m {
                if grid[i][j - 1] < grid[i][j] && (j == 1 || dp[i][j - 1] > 0) {
                    dp[i][j] = dp[i][j].max(dp[i][j - 1] + 1);
                }
                if i > 0 && grid[i - 1][j - 1] < grid[i][j] && (j == 1 || dp[i - 1][j - 1] > 0) {
                    dp[i][j] = dp[i][j].max(dp[i - 1][j - 1] + 1);
                }
                if i + 1 < m && grid[i + 1][j - 1] < grid[i][j] && (j == 1 || dp[i + 1][j - 1] > 0) {
                    dp[i][j] = dp[i][j].max(dp[i + 1][j - 1] + 1);
                }

                ret = ret.max(dp[i][j]);
            }
        }

        ret
    }
}

#[test]
fn test() {
    let v = vec![vec![2, 4, 3, 5], vec![5, 4, 9, 3], vec![3, 4, 2, 11], vec![10, 9, 13, 15]];
    assert_eq!(Solution::max_moves(v), 3);

    assert_eq!(Solution::max_moves(vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]]), 0);
}
