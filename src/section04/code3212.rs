#![allow(dead_code)]

// 3212. Count Submatrices With Equal Frequency of X and Y
// https://leetcode.com/problems/count-submatrices-with-equal-frequency-of-x-and-y/
// https://leetcode.cn/problems/count-submatrices-with-equal-frequency-of-x-and-y/
//
// Medium
//
// Given a 2D character matrix grid, where grid[i][j] is either 'X', 'Y', or '.', return the number of
// submatrices that contain:
//
// - grid[0][0]
// - an equal frequency of 'X' and 'Y'.
// - at least one 'X'.
//
// Example 1:
//
// Input: grid = [["X","Y","."],["Y",".","."]]
//
// Output: 3
//
// Explanation:
//
// Example 2:
//
// Input: grid = [["X","X"],["X","Y"]]
//
// Output: 0
//
// Explanation:
//
// No submatrix has an equal frequency of 'X' and 'Y'.
//
// Example 3:
//
// Input: grid = [[".","."],[".","."]]
//
// Output: 0
//
// Explanation:
//
// No submatrix has at least one 'X'.
//
// Constraints:
//
//     1 <= grid.length, grid[i].length <= 1000
//     grid[i][j] is either 'X', 'Y', or '.'.
//

struct Solution;

impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let yy = grid.len();
        let xx = grid[0].len();
        let mut cnt_x = vec![0; xx + 1];
        let mut cnt_y = vec![0; xx + 1];
        let mut res = 0;

        for grid_i in grid.iter().take(yy) {
            let mut curr_x = 0;
            let mut curr_y = 0;
            for j in 0..xx {
                if grid_i[j] == 'X' {
                    curr_x += 1;
                } else if grid_i[j] == 'Y' {
                    curr_y += 1;
                }
                cnt_x[j + 1] += curr_x;
                cnt_y[j + 1] += curr_y;

                if cnt_x[j + 1] == cnt_y[j + 1] && cnt_x[j + 1] > 0 {
                    res += 1;
                }
            }
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_submatrices(vec![vec!['X', 'Y', '.'], vec!['Y', '.', '.']]), 3);
    assert_eq!(Solution::number_of_submatrices(vec![vec!['X', 'X'], vec!['X', 'Y']]), 0);
    assert_eq!(Solution::number_of_submatrices(vec![vec!['.', '.'], vec!['.', '.']]), 0);
}
