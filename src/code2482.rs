#![allow(dead_code)]

// 2482. Difference Between Ones and Zeros in Row and Column
// https://leetcode.com/problems/difference-between-ones-and-zeros-in-row-and-column/
// https://leetcode.cn/problems/difference-between-ones-and-zeros-in-row-and-column/
//
// You are given a 0-indexed m x n binary matrix grid.
//
// A 0-indexed m x n difference matrix diff is created with the following procedure:
//
// Let the number of ones in the i-th row be onesRow[i].
// Let the number of ones in the j-th column be onesCol[j].
// Let the number of zeros in the i-th row be zerosRow[i].
// Let the number of zeros in the j-th column be zerosCol[j].
// diff[i][j] = onesRow[i] + onesCol[j] - zerosRow[i] - zerosCol[j]
//
// Return the difference matrix diff.
//
// Example 1:
//
// Input: grid = [[0,1,1],[1,0,1],[0,0,1]]
// Output: [[0,0,4],[0,0,4],[-2,-2,2]]
// Explanation:
// - diff[0][0] = onesRow0 + onesCol0 - zerosRow0 - zerosCol0 = 2 + 1 - 1 - 2 = 0
// - diff[0][1] = onesRow0 + onesCol1 - zerosRow0 - zerosCol1 = 2 + 1 - 1 - 2 = 0
// - diff[0][2] = onesRow0 + onesCol2 - zerosRow0 - zerosCol2 = 2 + 3 - 1 - 0 = 4
// - diff[1][0] = onesRow1 + onesCol0 - zerosRow1 - zerosCol0 = 2 + 1 - 1 - 2 = 0
// - diff[1][1] = onesRow1 + onesCol1 - zerosRow1 - zerosCol1 = 2 + 1 - 1 - 2 = 0
// - diff[1][2] = onesRow1 + onesCol2 - zerosRow1 - zerosCol2 = 2 + 3 - 1 - 0 = 4
// - diff[2][0] = onesRow2 + onesCol0 - zerosRow2 - zerosCol0 = 1 + 1 - 2 - 2 = -2
// - diff[2][1] = onesRow2 + onesCol1 - zerosRow2 - zerosCol1 = 1 + 1 - 2 - 2 = -2
// - diff[2][2] = onesRow2 + onesCol2 - zerosRow2 - zerosCol2 = 1 + 3 - 2 - 0 = 2
//
// Example 2:
//
// Input: grid = [[1,1,1],[1,1,1]]
// Output: [[5,5,5],[5,5,5]]
// Explanation:
// - diff[0][0] = onesRow0 + onesCol0 - zerosRow0 - zerosCol0 = 3 + 2 - 0 - 0 = 5
// - diff[0][1] = onesRow0 + onesCol1 - zerosRow0 - zerosCol1 = 3 + 2 - 0 - 0 = 5
// - diff[0][2] = onesRow0 + onesCol2 - zerosRow0 - zerosCol2 = 3 + 2 - 0 - 0 = 5
// - diff[1][0] = onesRow1 + onesCol0 - zerosRow1 - zerosCol0 = 3 + 2 - 0 - 0 = 5
// - diff[1][1] = onesRow1 + onesCol1 - zerosRow1 - zerosCol1 = 3 + 2 - 0 - 0 = 5
// - diff[1][2] = onesRow1 + onesCol2 - zerosRow1 - zerosCol2 = 3 + 2 - 0 - 0 = 5
//
// Constraints:
//
// - m == grid.length
// - n == grid[i].length
// - 1 <= m, n <= 10^5
// - 1 <= m * n <= 10^5
// - grid[i][j] is either 0 or 1.
//

struct Solution;

impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row_len = grid.len();
        let col_len = grid[0].len();
        let mut row_ones = vec![0; row_len];
        let mut col_ones = vec![0; col_len];
        for (y, row) in grid.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                row_ones[y] += val;
                col_ones[x] += val;
            }
        }
        let mut ans = vec![vec![0; col_len]; row_len];
        for (y, row) in ans.iter_mut().enumerate() {
            for (x, val) in row.iter_mut().enumerate() {
                *val = 2 * (row_ones[y] + col_ones[x]) - (row_len + col_len) as i32;
            }
        }
        ans
    }
}

#[test]
fn test() {
    let grid = vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]];
    let expected = vec![vec![0, 0, 4], vec![0, 0, 4], vec![-2, -2, 2]];
    assert_eq!(Solution::ones_minus_zeros(grid), expected);

    let grid = vec![vec![1, 1, 1], vec![1, 1, 1]];
    let expected = vec![vec![5, 5, 5], vec![5, 5, 5]];
    assert_eq!(Solution::ones_minus_zeros(grid), expected);
}
