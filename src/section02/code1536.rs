#![allow(dead_code)]

/*

// 1536. Minimum Swaps to Arrange a Binary Grid
// https://leetcode.com/problems/minimum-swaps-to-arrange-a-binary-grid/
// https://leetcode.cn/problems/minimum-swaps-to-arrange-a-binary-grid/
//
// Medium
//
// Given an n x n binary grid, in one step you can choose two adjacent rows of the grid and swap them.

A grid is said to be valid if all the cells above the main diagonal are zeros.

Return the minimum number of steps needed to make the grid valid, or -1 if the grid cannot be valid.

The main diagonal of a grid is the diagonal that starts at cell (1, 1) and ends at cell (n, n).

Example 1:

Input: grid = [[0,0,1],[1,1,0],[1,0,0]]
Output: 3

Example 2:

Input: grid = [[0,1,1,0],[0,1,1,0],[0,1,1,0],[0,1,1,0]]
Output: -1
Explanation: All rows are similar, swaps have no effect on the grid.

Example 3:

Input: grid = [[1,0,0],[1,1,0],[1,1,1]]
Output: 0

Constraints:

    n == grid.length == grid[i].length
    1 <= n <= 200
    grid[i][j] is either 0 or 1
*/

struct Solution;

impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let mut zeros = vec![0; grid.len()];
        for (i, row) in grid.iter().enumerate() {
            zeros[i] = row.iter().rev().take_while(|&&x| x == 0).count() as i32;
        }
        let mut swaps = 0;
        for i in 0..zeros.len() {
            if zeros[i] < (zeros.len() - i - 1) as i32 {
                let mut j = i + 1;
                while j < zeros.len() && zeros[j] < (zeros.len() - i - 1) as i32 {
                    j += 1;
                }
                if j == zeros.len() {
                    return -1;
                }
                while j > i {
                    zeros.swap(j, j - 1);
                    swaps += 1;
                    j -= 1;
                }
            }
        }
        swaps
    }
}

#[test]
fn test() {
    let grid = vec![vec![0, 0, 1], vec![1, 1, 0], vec![1, 0, 0]];
    assert_eq!(Solution::min_swaps(grid), 3);
    let grid = vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 1, 1, 0]];
    assert_eq!(Solution::min_swaps(grid), -1);
    let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 1]];
    assert_eq!(Solution::min_swaps(grid), 0);
}
