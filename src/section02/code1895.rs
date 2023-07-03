#![allow(dead_code)]

/*

// 1895. Largest Magic Square
// https://leetcode.com/problems/largest-magic-square/
// https://leetcode.cn/problems/largest-magic-square/
//
// Medium
//
// A k x k magic square is a k x k grid filled with integers such that every row sum, every column sum, and both diagonal sums are all equal. The integers in the magic square do not have to be distinct. Every 1 x 1 grid is trivially a magic square.

Given an m x n integer grid, return the size (i.e., the side length k) of the largest magic square that can be found within this grid.

Example 1:

Input: grid = [[7,1,4,5,6],[2,5,1,6,4],[1,5,4,3,2],[1,2,7,3,4]]
Output: 3
Explanation: The largest magic square has a size of 3.
Every row sum, column sum, and diagonal sum of this magic square is equal to 12.
- Row sums: 5+1+6 = 5+4+3 = 2+7+3 = 12
- Column sums: 5+5+2 = 1+4+7 = 6+3+3 = 12
- Diagonal sums: 5+4+3 = 6+4+2 = 12

Example 2:

Input: grid = [[5,1,3,1],[9,3,3,1],[1,3,3,8]]
Output: 2

Constraints:

    m == grid.length
    n == grid[i].length
    1 <= m, n <= 50
    1 <= grid[i][j] <= 10^6
*/

struct Solution;

impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        let mut pre_sum_row = vec![vec![0; (n + 1) as usize]; m as usize];
        let mut pre_sum_col = vec![vec![0; (m + 1) as usize]; n as usize];
        for r in 0..m {
            for c in 0..n {
                pre_sum_row[r as usize][c as usize + 1] = pre_sum_row[r as usize][c as usize] + grid[r as usize][c as usize];
                pre_sum_col[c as usize][r as usize + 1] = pre_sum_col[c as usize][r as usize] + grid[r as usize][c as usize];
            }
        }
        for k in (2..=m.min(n)).rev() {
            if Self::test(&grid, k, m, n, &pre_sum_row, &pre_sum_col) {
                return k;
            }
        }
        1
    }

    fn test(grid: &[Vec<i32>], k: i32, m: i32, n: i32, pre_sum_row: &[Vec<i32>], pre_sum_col: &[Vec<i32>]) -> bool {
        for r in 0..m - k + 1 {
            for c in 0..n - k + 1 {
                let mut diag = 0;
                let mut anti_diag = 0;
                for d in 0..k {
                    diag += grid[(r + d) as usize][(c + d) as usize];
                    anti_diag += grid[(r + d) as usize][(c + k - 1 - d) as usize];
                }
                let mut match_ = diag == anti_diag;
                for nr in r..r + k {
                    if !match_ {
                        break;
                    }
                    match_ = diag == Self::get_sum_row(nr, c, c + k - 1, pre_sum_row);
                }
                for nc in c..c + k {
                    if !match_ {
                        break;
                    }
                    match_ = diag == Self::get_sum_col(nc, r, r + k - 1, pre_sum_col);
                }
                if match_ {
                    return true;
                }
            }
        }
        false
    }

    fn get_sum_row(row: i32, l: i32, r: i32, pre_sum_row: &[Vec<i32>]) -> i32 {
        pre_sum_row[row as usize][r as usize + 1] - pre_sum_row[row as usize][l as usize]
    }

    fn get_sum_col(col: i32, l: i32, r: i32, pre_sum_col: &[Vec<i32>]) -> i32 {
        pre_sum_col[col as usize][r as usize + 1] - pre_sum_col[col as usize][l as usize]
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![7, 1, 4, 5, 6], vec![2, 5, 1, 6, 4], vec![1, 5, 4, 3, 2], vec![1, 2, 7, 3, 4]],
            3,
        ),
        (vec![vec![5, 1, 3, 1], vec![9, 3, 3, 1], vec![1, 3, 3, 8]], 2),
        (vec![vec![7]], 1),
    ];
    for (grid, expected) in cases {
        assert_eq!(Solution::largest_magic_square(grid), expected);
    }
}
