#![allow(dead_code)]

// 3402. Minimum Operations to Make Columns Strictly Increasing
// https://leetcode.com/problems/minimum-operations-to-make-columns-strictly-increasing/
// https://leetcode.cn/problems/minimum-operations-to-make-columns-strictly-increasing/
//
// Easy
//
// You are given a m x n matrix grid consisting of non-negative integers.
//
// In one operation, you can increment the value of any grid[i][j] by 1.
//
// Return the minimum number of operations needed to make all columns of grid strictly increasing.
//
// Example 1:
//
// Input: grid = [[3,2],[1,3],[3,4],[0,1]]
//
// Output: 15
//
// Explanation:
//
// To make the 0th column strictly increasing, we can apply 3 operations on grid[1][0], 2 operations on grid[2][0], and 6 operations on grid[3][0].
// To make the 1st column strictly increasing, we can apply 4 operations on grid[3][1].
//
// Example 2:
//
// Input: grid = [[3,2,1],[2,1,0],[1,2,3]]
//
// Output: 12
//
// Explanation:
//
// To make the 0th column strictly increasing, we can apply 2 operations on grid[1][0], and 4 operations on grid[2][0].
// To make the 1st column strictly increasing, we can apply 2 operations on grid[1][1], and 2 operations on grid[2][1].
// To make the 2nd column strictly increasing, we can apply 2 operations on grid[1][2].
//
// Constraints:
//
// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 50
// 0 <= grid[i][j] < 2500
//

struct Solution;

impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut grid = grid;
        for c in 0..grid[0].len() {
            for r in 1..grid.len() {
                if grid[r][c] > grid[r - 1][c] {
                    continue;
                }
                ans += grid[r - 1][c] + 1 - grid[r][c];
                grid[r][c] = grid[r - 1][c] + 1;
            }
        }
        ans
    }
}

#[test]
fn test() {
    let grid = vec![vec![3, 2], vec![1, 3], vec![3, 4], vec![0, 1]];
    let res = 15;
    assert_eq!(Solution::minimum_operations(grid), res);

    let grid = vec![vec![3, 2, 1], vec![2, 1, 0], vec![1, 2, 3]];
    let res = 12;
    assert_eq!(Solution::minimum_operations(grid), res);
}
