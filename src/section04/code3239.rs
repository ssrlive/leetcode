#![allow(dead_code)]

// 3239. Minimum Number of Flips to Make Binary Grid Palindromic I
// https://leetcode.com/problems/minimum-number-of-flips-to-make-binary-grid-palindromic-i/
// https://leetcode.cn/problems/minimum-number-of-flips-to-make-binary-grid-palindromic-i/
//
// Medium
//
// You are given an m x n binary matrix grid.
//
// A row or column is considered palindromic if its values read the same forward and backward.
//
// You can flip any number of cells in grid from 0 to 1, or from 1 to 0.
//
// Return the minimum number of cells that need to be flipped to make either all rows palindromic or all columns palindromic.
//
// Example 1:
//
// Input: grid = [[1,0,0],[0,0,0],[0,0,1]]
//
// Output: 2
//
// Explanation:
//
// Flipping the highlighted cells makes all the rows palindromic.
//
// Example 2:
//
// Input: grid = [[0,1],[0,1],[0,0]]
//
// Output: 1
//
// Explanation:
//
// Flipping the highlighted cell makes all the columns palindromic.
//
// Example 3:
//
// Input: grid = [[1],[0]]
//
// Output: 0
//
// Explanation:
//
// All rows are already palindromic.
//
// Constraints:
//
//     m == grid.length
//     n == grid[i].length
//     1 <= m * n <= 2 * 10^5
//     0 <= grid[i][j] <= 1
//

struct Solution;

impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let mut row_count = 0;
        let n = grid.len();
        let m = grid[0].len();

        // Count flips needed for each row to make them palindromic
        for grid_i in grid.iter().take(n) {
            for j in (0..m).rev() {
                if grid_i[m - j - 1] != grid_i[j] {
                    row_count += 1;
                }
            }
        }

        // Count flips needed for each column to make them palindromic
        let col_count: i32 = (0..m)
            .map(|j| (0..n).rev().filter(|&i| grid[n - i - 1][j] != grid[i][j]).count() as i32)
            .sum();

        // Return the minimum number of flips required
        (row_count / 2).min(col_count / 2)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_flips(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]]), 2);
    assert_eq!(Solution::min_flips(vec![vec![0, 1], vec![0, 1], vec![0, 0]]), 1);
    assert_eq!(Solution::min_flips(vec![vec![1], vec![0]]), 0);
}
