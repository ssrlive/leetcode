#![allow(dead_code)]

// 867. Transpose Matrix
// https://leetcode.com/problems/transpose-matrix/
// https://leetcode.cn/problems/transpose-matrix/
//
// Given a 2D integer array matrix, return the transpose of matrix.
//
// The transpose of a matrix is the matrix flipped over its main diagonal, switching the matrix's row and column indices.
//
// Example 1:
//
// Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
// Output: [[1,4,7],[2,5,8],[3,6,9]]
//
// Example 2:
//
// Input: matrix = [[1,2,3],[4,5,6]]
// Output: [[1,4],[2,5],[3,6]]
//
// Constraints:
//
// - m == matrix.length
// - n == matrix[i].length
// - 1 <= m, n <= 1000
// - 1 <= m * n <= 10^5
// - -10^9 <= matrix[i][j] <= 10^9
//

struct Solution;

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let j_len = matrix[0].len();
        let mut result = Vec::new();

        for j in 0..j_len {
            let mut temp = Vec::new();
            for item in matrix.iter() {
                temp.push(item[j]);
            }
            result.push(temp);
        }

        result
    }
}

#[test]
fn test() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let result = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
    assert_eq!(Solution::transpose(matrix), result);

    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let result = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
    assert_eq!(Solution::transpose(matrix), result);
}
