#![allow(dead_code)]

/*

// 1727. Largest Submatrix With Rearrangements
// https://leetcode.com/problems/largest-submatrix-with-rearrangements/
// https://leetcode.cn/problems/largest-submatrix-with-rearrangements/
//
// Medium
//
// You are given a binary matrix matrix of size m x n, and you are allowed to rearrange the columns of the matrix in any order.

Return the area of the largest submatrix within matrix where every element of the submatrix is 1 after reordering the columns optimally.

Example 1:

Input: matrix = [[0,0,1],[1,1,1],[1,0,1]]
Output: 4
Explanation: You can rearrange the columns as shown above.
The largest submatrix of 1s, in bold, has an area of 4.

Example 2:

Input: matrix = [[1,0,1,0,1]]
Output: 3
Explanation: You can rearrange the columns as shown above.
The largest submatrix of 1s, in bold, has an area of 3.

Example 3:

Input: matrix = [[1,1,0],[1,0,1]]
Output: 2
Explanation: Notice that you must rearrange entire columns, and there is no way to make a submatrix of 1s larger than an area of 2.

Constraints:

    m == matrix.length
    n == matrix[i].length
    1 <= m * n <= 10^5
    matrix[i][j] is either 0 or 1.
*/

struct Solution;

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let mut matrix = matrix;
        let m = matrix.len();
        let n = matrix[0].len();
        for i in 1..m {
            for j in 0..n {
                if matrix[i][j] == 1 {
                    matrix[i][j] += matrix[i - 1][j];
                }
            }
        }
        let mut ans = 0;
        for item_i in matrix.iter_mut() {
            item_i.sort();
            for (j, &item_j) in item_i.iter().enumerate() {
                ans = ans.max(item_j * (n - j) as i32);
            }
        }
        ans
    }
}

#[test]
fn test() {
    let matrix = vec![vec![0, 0, 1], vec![1, 1, 1], vec![1, 0, 1]];
    assert_eq!(Solution::largest_submatrix(matrix), 4);
    let matrix = vec![vec![1, 0, 1, 0, 1]];
    assert_eq!(Solution::largest_submatrix(matrix), 3);
    let matrix = vec![vec![1, 1, 0], vec![1, 0, 1]];
    assert_eq!(Solution::largest_submatrix(matrix), 2);
}
