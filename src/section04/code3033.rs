#![allow(dead_code)]

// 3033. Modify the Matrix
// https://leetcode.com/problems/modify-the-matrix/
// https://leetcode.cn/problems/modify-the-matrix/
//
// Easy
//
// Given a 0-indexed m x n integer matrix matrix, create a new 0-indexed matrix called answer.
// Make answer equal to matrix, then replace each element with the value -1 with the maximum element in its respective column.
//
// Return the matrix answer.
//
// Example 1:
//
// Input: matrix = [[1,2,-1],[4,-1,6],[7,8,9]]
// Output: [[1,2,9],[4,8,6],[7,8,9]]
// Explanation: The diagram above shows the elements that are changed (in blue).
// - We replace the value in the cell [1][1] with the maximum value in the column 1, that is 8.
// - We replace the value in the cell [0][2] with the maximum value in the column 2, that is 9.
//
// Example 2:
//
// Input: matrix = [[3,-1],[5,2]]
// Output: [[3,2],[5,2]]
// Explanation: The diagram above shows the elements that are changed (in blue).
//
// Constraints:
//
// m == matrix.length
// n == matrix[i].length
// 2 <= m, n <= 50
// -1 <= matrix[i][j] <= 100
// The input is generated such that each column contains at least one non-negative integer.
//

struct Solution;

impl Solution {
    pub fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let hi = (0..n).map(|j| (0..m).map(|i| matrix[i][j]).max().unwrap()).collect::<Vec<i32>>();
        (0..m)
            .map(|i| {
                (0..n)
                    .map(|j| if matrix[i][j] == -1 { hi[j] } else { matrix[i][j] })
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>()
    }
}

#[test]
fn test() {
    let matrix = vec![vec![1, 2, -1], vec![4, -1, 6], vec![7, 8, 9]];
    let answer = vec![vec![1, 2, 9], vec![4, 8, 6], vec![7, 8, 9]];
    assert_eq!(Solution::modified_matrix(matrix), answer);

    let matrix = vec![vec![3, -1], vec![5, 2]];
    let answer = vec![vec![3, 2], vec![5, 2]];
    assert_eq!(Solution::modified_matrix(matrix), answer);
}
