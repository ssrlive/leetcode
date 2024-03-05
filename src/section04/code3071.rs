#![allow(dead_code)]

// 3071. Minimum Operations to Write the Letter Y on a Grid
// https://leetcode.com/problems/minimum-operations-to-write-the-letter-y-on-a-grid/
// https://leetcode.cn/problems/minimum-operations-to-write-the-letter-y-on-a-grid/
//
// Medium
// You are given a 0-indexed n x n grid where n is odd, and grid[r][c] is 0, 1, or 2.
//
// We say that a cell belongs to the Letter Y if it belongs to one of the following:
//
// The diagonal starting at the top-left cell and ending at the center cell of the grid.
// The diagonal starting at the top-right cell and ending at the center cell of the grid.
// The vertical line starting at the center cell and ending at the bottom border of the grid.
// The Letter Y is written on the grid if and only if:
//
// All values at cells belonging to the Y are equal.
// All values at cells not belonging to the Y are equal.
// The values at cells belonging to the Y are different from the values at cells not belonging to the Y.
// Return the minimum number of operations needed to write the letter Y on the grid given that in one operation you can change the value at any cell to 0, 1, or 2.
//
// Example 1:
//
// Input: grid = [[1,2,2],[1,1,0],[0,1,0]]
// Output: 3
// Explanation: We can write Y on the grid by applying the changes highlighted in blue in the image above.
// After the operations, all cells that belong to Y, denoted in bold, have the same value of 1 while those that do not belong to Y are equal to 0.
// It can be shown that 3 is the minimum number of operations needed to write Y on the grid.
//
// Example 2:
//
// Input: grid = [[0,1,0,1,0],[2,1,0,1,2],[2,2,2,0,1],[2,2,2,2,2],[2,1,2,2,2]]
// Output: 12
// Explanation: We can write Y on the grid by applying the changes highlighted in blue in the image above.
// After the operations, all cells that belong to Y, denoted in bold, have the same value of 0 while those that do not belong to Y are equal to 2.
// It can be shown that 12 is the minimum number of operations needed to write Y on the grid.
//
// Constraints:
//
// 3 <= n <= 49
// n == grid.length == grid[i].length
// 0 <= grid[i][j] <= 2
// n is odd.
//

struct Solution;

impl Solution {
    pub fn minimum_operations_to_write_y(grid: Vec<Vec<i32>>) -> i32 {
        let (n, k) = (grid.len(), grid.len() / 2);
        let (mut cnts_o, mut cnts_y) = ([0, 0, 0], [0, 0, 0]);

        for y in 0..n {
            for x in 0..n {
                cnts_o[grid[y][x] as usize] += 1;
            }
        }

        cnts_y[grid[k][k] as usize] += 1;
        cnts_o[grid[k][k] as usize] -= 1;

        for i in 0..k {
            cnts_y[grid[i][i] as usize] += 1;
            cnts_o[grid[i][i] as usize] -= 1;
            cnts_y[grid[i][n - 1 - i] as usize] += 1;
            cnts_o[grid[i][n - 1 - i] as usize] -= 1;
            cnts_y[grid[k + i + 1][k] as usize] += 1;
            cnts_o[grid[k + i + 1][k] as usize] -= 1;
        }

        let (s_y, s_o, mut res) = (
            cnts_y[0] + cnts_y[1] + cnts_y[2],
            cnts_o[0] + cnts_o[1] + cnts_o[2],
            (n * n) as i32 + 1,
        );
        for (i, &cnts_y_i) in cnts_y.iter().enumerate() {
            for (j, &cnts_o_j) in cnts_o.iter().enumerate() {
                if i != j {
                    let tmp = s_y - cnts_y_i + s_o - cnts_o_j;
                    res = res.min(tmp);
                }
            }
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::minimum_operations_to_write_y(vec![vec![1, 2, 2], vec![1, 1, 0], vec![0, 1, 0]]),
        3
    );
    assert_eq!(
        Solution::minimum_operations_to_write_y(vec![
            vec![0, 1, 0, 1, 0],
            vec![2, 1, 0, 1, 2],
            vec![2, 2, 2, 0, 1],
            vec![2, 2, 2, 2, 2],
            vec![2, 1, 2, 2, 2]
        ]),
        12
    );
}
