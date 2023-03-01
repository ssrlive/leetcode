#![allow(dead_code)]

/*

// 1975. Maximum Matrix Sum
// https://leetcode.com/problems/maximum-matrix-sum/
// https://leetcode.cn/problems/maximum-matrix-sum/
//
// Medium
//
// You are given an n x n integer matrix. You can do the following operation any number of times:

    Choose any two adjacent elements of matrix and multiply each of them by -1.

Two elements are considered adjacent if and only if they share a border.

Your goal is to maximize the summation of the matrix's elements. Return the maximum sum of the matrix's elements using the operation mentioned above.

Example 1:

Input: matrix = [[1,-1],[-1,1]]
Output: 4
Explanation: We can follow the following steps to reach sum equals 4:
- Multiply the 2 elements in the first row by -1.
- Multiply the 2 elements in the first column by -1.

Example 2:

Input: matrix = [[1,2,3],[-1,-2,-3],[1,2,3]]
Output: 16
Explanation: We can follow the following step to reach sum equals 16:
- Multiply the 2 last elements in the second row by -1.

Constraints:

    n == matrix.length == matrix[i].length
    2 <= n <= 250
    -10^5 <= matrix[i][j] <= 10^5
*/

struct Solution;

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let matrix = matrix
            .iter()
            .map(|row| row.iter().map(|&x| x as i64).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut neg = false;
        let mut sum = 0_i64;
        let mut mini = i64::MAX;
        for row in matrix.iter() {
            for &x in row.iter() {
                if x < 0 {
                    neg = !neg;
                }
                let x = x.abs();
                sum += x;
                if x < mini {
                    mini = x;
                }
            }
        }
        if neg {
            sum -= 2 * mini;
        }
        sum
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, -1], vec![-1, 1]], 4),
        (vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]], 16),
    ];
    for (matrix, expected) in cases {
        assert_eq!(Solution::max_matrix_sum(matrix), expected);
    }
}
