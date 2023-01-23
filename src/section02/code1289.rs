#![allow(dead_code)]

// 1289. Minimum Falling Path Sum II
// https://leetcode.com/problems/minimum-falling-path-sum-ii/
// https://leetcode.cn/problems/minimum-falling-path-sum-ii/
//
// Hard
//
// Given an n x n integer matrix grid, return the minimum sum of a falling path with non-zero shifts.
//
// A falling path with non-zero shifts is a choice of exactly one element from each row of grid
// such that no two elements chosen in adjacent rows are in the same column.
//
// Example 1:
//
// Input: arr = [[1,2,3],[4,5,6],[7,8,9]]
// Output: 13
// Explanation:
// The possible falling paths are:
// [1,5,9], [1,5,7], [1,6,7], [1,6,8],
// [2,4,8], [2,4,9], [2,6,7], [2,6,8],
// [3,4,8], [3,4,9], [3,5,7], [3,5,9]
// The falling path with the smallest sum is [1,5,7], so the answer is 13.
//
// Example 2:
//
// Input: grid = [[7]]
// Output: 7
//
// Constraints:
//
// -    n == grid.length == grid[i].length
// -    1 <= n <= 200
// -    -99 <= grid[i][j] <= 99
//

struct Solution;

impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dp = vec![vec![0; n]; n];
        for i in 0..n {
            dp[0][i] = grid[0][i];
        }
        for i in 1..n {
            let mut min1 = i32::MAX;
            let mut min2 = i32::MAX;
            for j in 0..n {
                if dp[i - 1][j] < min1 {
                    min2 = min1;
                    min1 = dp[i - 1][j];
                } else if dp[i - 1][j] < min2 {
                    min2 = dp[i - 1][j];
                }
            }
            for j in 0..n {
                if dp[i - 1][j] == min1 {
                    dp[i][j] = min2 + grid[i][j];
                } else {
                    dp[i][j] = min1 + grid[i][j];
                }
            }
        }
        let mut min = i32::MAX;
        for i in 0..n {
            if dp[n - 1][i] < min {
                min = dp[n - 1][i];
            }
        }
        min
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 13),
        (vec![vec![7]], 7),
    ];
    for (grid, expect) in cases {
        assert_eq!(Solution::min_falling_path_sum(grid), expect);
    }
}
