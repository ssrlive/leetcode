#![allow(dead_code)]

// 566. Reshape the Matrix
// https://leetcode.com/problems/reshape-the-matrix/
// https://leetcode.cn/problems/reshape-the-matrix/
//
// In MATLAB, there is a handy function called reshape which can reshape an m x n matrix
// into a new one with a different size r x c keeping its original data.
//
// You are given an m x n matrix mat and two integers r and c representing the number of rows
// and the number of columns of the wanted reshaped matrix.
//
// The reshaped matrix should be filled with all the elements of the original matrix
// in the same row-traversing order as they were.
//
// If the reshape operation with given parameters is possible and legal,
// output the new reshaped matrix; Otherwise, output the original matrix.
//
// Example 1:
//
// Input: mat = [[1,2],[3,4]], r = 1, c = 4
// Output: [[1,2,3,4]]
//
// Example 2:
//
// Input: mat = [[1,2],[3,4]], r = 2, c = 4
// Output: [[1,2],[3,4]]
//
// Constraints:
//
// - m == mat.length
// - n == mat[i].length
// - 1 <= m, n <= 100
// - -1000 <= mat[i][j] <= 1000
// - 1 <= r, c <= 300
//

struct Solution;

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();

        if m * n != (r * c) as usize {
            return mat;
        }

        let mut result = vec![vec![0; c as usize]; r as usize];
        let mut i = 0;
        let mut j = 0;

        for row in mat {
            for num in row {
                result[i][j] = num;
                j += 1;

                if j == c as usize {
                    j = 0;
                    i += 1;
                }
            }
        }

        result
    }
}

#[test]
fn test() {
    let mat = vec![vec![1, 2], vec![3, 4]];
    let r = 1;
    let c = 4;
    let expected = vec![vec![1, 2, 3, 4]];
    let result = Solution::matrix_reshape(mat, r, c);
    assert_eq!(result, expected);

    let mat = vec![vec![1, 2], vec![3, 4]];
    let r = 2;
    let c = 4;
    let expected = vec![vec![1, 2], vec![3, 4]];
    let result = Solution::matrix_reshape(mat, r, c);
    assert_eq!(result, expected);
}
