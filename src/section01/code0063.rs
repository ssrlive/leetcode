#![allow(dead_code)]

// 63. Unique Paths II
// https://leetcode.com/problems/unique-paths-ii/
// https://leetcode.cn/problems/unique-paths-ii/
//
// You are given an m x n integer array grid. There is a robot initially located at the top-left corner (i.e., grid[0][0]).
// The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]). The robot can only move either down or right at any point in time.
//
// An obstacle and space are marked as 1 or 0 respectively in grid. A path that the robot takes cannot include any square that is an obstacle.
//
// Return the number of possible unique paths that the robot can take to reach the bottom-right corner.
//
// The testcases are generated so that the answer will be less than or equal to 2 * 109.
//
// Example 1:
//
// Input: obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
// Output: 2
// Explanation: There is one obstacle in the middle of the 3x3 grid above.
// There are two ways to reach the bottom-right corner:
// 1. Right -> Right -> Down -> Down
// 2. Down -> Down -> Right -> Right
//
// Example 2:
//
// Input: obstacleGrid = [[0,1],[0,0]]
// Output: 1
//
// Constraints:
//
// - m == obstacleGrid.length
// - n == obstacleGrid[i].length
// - 1 <= m, n <= 100
// - obstacleGrid[i][j] is 0 or 1.
//

struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let n = obstacle_grid[0].len();
        let mut dp_prev = obstacle_grid[0]
            .iter()
            .scan(1, |ok, cell| {
                *ok &= (*cell == 0) as i32;
                Some(*ok)
            })
            .collect::<Vec<_>>();
        let mut dp_curr = vec![0; n];
        for item in obstacle_grid.iter().skip(1) {
            dp_curr[0] = if item[0] == 0 { dp_prev[0] } else { 0 };
            for j in 1..n {
                dp_curr[j] = if item[j] == 0 { dp_curr[j - 1] + dp_prev[j] } else { 0 };
            }
            std::mem::swap(&mut dp_prev, &mut dp_curr);
        }
        dp_prev[n - 1]
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]], 2),
        (vec![vec![0, 1], vec![0, 0]], 1),
        (vec![vec![0, 0], vec![1, 1], vec![0, 0]], 0),
        (vec![vec![0, 0], vec![0, 1]], 0),
    ];
    for (obstacle_grid, expected) in cases {
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), expected);
    }
}
