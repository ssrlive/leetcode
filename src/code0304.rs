#![allow(dead_code)]

// 304. Range Sum Query 2D - Immutable
// https://leetcode.com/problems/range-sum-query-2d-immutable/
//
// Given a 2D matrix matrix, handle multiple queries of the following type:
//
// Calculate the sum of the elements of matrix inside the rectangle defined by its upper
// left corner (row1, col1) and lower right corner (row2, col2).
//
// Implement the NumMatrix class:
//
// NumMatrix(int[][] matrix) Initializes the object with the integer matrix matrix.
// int sumRegion(int row1, int col1, int row2, int col2) Returns the sum of the elements of matrix inside
// the rectangle defined by its upper left corner (row1, col1) and lower right corner (row2, col2).
// You must design an algorithm where sumRegion works on O(1) time complexity.
//
// Example 1:
//
// Input
// ["NumMatrix", "sumRegion", "sumRegion", "sumRegion"]
// [[[[3, 0, 1, 4, 2], [5, 6, 3, 2, 1], [1, 2, 0, 1, 5], [4, 1, 0, 1, 7], [1, 0, 3, 0, 5]]], [2, 1, 4, 3], [1, 1, 2, 2], [1, 2, 2, 4]]
// Output
// [null, 8, 11, 12]
//
// Explanation
// NumMatrix numMatrix = new NumMatrix([[3, 0, 1, 4, 2], [5, 6, 3, 2, 1], [1, 2, 0, 1, 5], [4, 1, 0, 1, 7], [1, 0, 3, 0, 5]]);
// numMatrix.sumRegion(2, 1, 4, 3); // return 8 (i.e sum of the red rectangle)
// numMatrix.sumRegion(1, 1, 2, 2); // return 11 (i.e sum of the green rectangle)
// numMatrix.sumRegion(1, 2, 2, 4); // return 12 (i.e sum of the blue rectangle)
//
// Constraints:
//
// m == matrix.length
// n == matrix[i].length
// 1 <= m, n <= 200
// -104 <= matrix[i][j] <= 104
// 0 <= row1 <= row2 < m
// 0 <= col1 <= col2 < n
// At most 104 calls will be made to sumRegion.
//

struct NumMatrix {
    sums: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut sums = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                sums[i + 1][j + 1] = sums[i + 1][j] + sums[i][j + 1] - sums[i][j] + matrix[i][j];
            }
        }
        Self { sums }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.sums[row2 as usize + 1][col2 as usize + 1]
            - self.sums[row2 as usize + 1][col1 as usize]
            - self.sums[row1 as usize][col2 as usize + 1]
            + self.sums[row1 as usize][col1 as usize]
    }
}

#[test]
fn test_sum_region() {
    let matrix = vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ];
    let num_matrix = NumMatrix::new(matrix);
    assert_eq!(num_matrix.sum_region(2, 1, 4, 3), 8);
    assert_eq!(num_matrix.sum_region(1, 1, 2, 2), 11);
    assert_eq!(num_matrix.sum_region(1, 2, 2, 4), 12);
}
