#![allow(dead_code)]

// 73. Set Matrix Zeroes
// https://leetcode.com/problems/set-matrix-zeroes/
// https://leetcode.cn/problems/set-matrix-zeroes/
//
// Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.
//
// You must do it in place.
//
// Example 1:
//
// Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
// Output: [[1,0,1],[0,0,0],[1,0,1]]
//
// Example 2:
//
// Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
// Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
//
// Constraints:
//
// - m == matrix.length
// - n == matrix[0].length
// - 1 <= m, n <= 200
// - -2^31 <= matrix[i][j] <= 2^31 - 1
//
// Follow up:
//
// - A straightforward solution using O(mn) space is probably a bad idea.
// - A simple improvement uses O(m + n) space, but still not the best solution.
// - Could you devise a constant space solution?
//

struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut [Vec<i32>]) {
        let mut clear_row = [false; 200];
        let mut clear_col = [false; 200];
        for i in 0..matrix.len() {
            for (j, item) in clear_col.iter_mut().enumerate().take(matrix[i].len()) {
                if matrix[i][j] == 0 {
                    clear_row[i] = true;
                    *item = true;
                }
            }
        }
        for i in 0..matrix.len() {
            for (j, &item) in clear_col.iter().enumerate().take(matrix[i].len()) {
                if clear_row[i] || item {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}

#[test]
fn test() {
    let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    Solution::set_zeroes(&mut matrix);
    assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);

    let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
    Solution::set_zeroes(&mut matrix);
    assert_eq!(matrix, vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]);
}
