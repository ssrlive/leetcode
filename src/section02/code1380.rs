#![allow(dead_code)]

// 1380. Lucky Numbers in a Matrix
// https://leetcode.com/problems/lucky-numbers-in-a-matrix/
// https://leetcode.cn/problems/lucky-numbers-in-a-matrix/
//
// Easy
//
// Given an m x n matrix of distinct numbers, return all lucky numbers in the matrix in any order.
//
// A lucky number is an element of the matrix such that it is the minimum element in its row and maximum in its column.
//
// Example 1:
//
// Input: matrix = [[3,7,8],[9,11,13],[15,16,17]]
// Output: [15]
// Explanation: 15 is the only lucky number since it is the minimum in its row and the maximum in its column.
//
// Example 2:
//
// Input: matrix = [[1,10,4,2],[9,3,8,7],[15,16,17,12]]
// Output: [12]
// Explanation: 12 is the only lucky number since it is the minimum in its row and the maximum in its column.
//
// Example 3:
//
// Input: matrix = [[7,8],[1,2]]
// Output: [7]
// Explanation: 7 is the only lucky number since it is the minimum in its row and the maximum in its column.
//
// Constraints:
//
// -    m == mat.length
// -    n == mat[i].length
// -    1 <= n, m <= 50
// -    1 <= matrix[i][j] <= 10^5.
// -    All elements in the matrix are distinct.
//

struct Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut min_row = vec![i32::MAX; matrix.len()];
        let mut max_col = vec![i32::MIN; matrix[0].len()];
        for i in 0..matrix.len() {
            for (j, item) in max_col.iter_mut().enumerate() {
                min_row[i] = min_row[i].min(matrix[i][j]);
                *item = (*item).max(matrix[i][j]);
            }
        }
        let mut res = vec![];
        for i in 0..matrix.len() {
            for (j, &item) in max_col.iter().enumerate() {
                if matrix[i][j] == min_row[i] && matrix[i][j] == item {
                    res.push(matrix[i][j]);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]], vec![15]),
        (vec![vec![1, 10, 4, 2], vec![9, 3, 8, 7], vec![15, 16, 17, 12]], vec![12]),
        (vec![vec![7, 8], vec![1, 2]], vec![7]),
    ];
    for (matrix, expect) in cases {
        assert_eq!(Solution::lucky_numbers(matrix), expect);
    }
}
