#![allow(dead_code)]

// 498. Diagonal Traverse
// https://leetcode.com/problems/diagonal-traverse/
// https://leetcode.cn/problems/diagonal-traverse/
//
// Given an m x n matrix mat, return an array of all the elements of the array in a diagonal order.
//
// Example 1:
//
// Input: mat = [[1,2,3],[4,5,6],[7,8,9]]
// Output: [1,2,4,7,5,3,6,8,9]
//
// Example 2:
//
// Input: mat = [[1,2],[3,4]]
// Output: [1,2,3,4]
//
// Constraints:
//
// - m == mat.length
// - n == mat[i].length
// - 1 <= m, n <= 10^4
// - 1 <= m * n <= 10^4
// - -10^5 <= mat[i][j] <= 10^5
//

struct Solution;

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut row = 0;
        let mut col = 0;
        let mut up = true;
        let m = mat.len();
        let n = mat[0].len();
        for _ in 0..m * n {
            result.push(mat[row][col]);
            if up {
                if col == n - 1 {
                    row += 1;
                    up = false;
                } else if row == 0 {
                    col += 1;
                    up = false;
                } else {
                    row -= 1;
                    col += 1;
                }
            } else if row == m - 1 {
                col += 1;
                up = true;
            } else if col == 0 {
                row += 1;
                up = true;
            } else {
                row += 1;
                col -= 1;
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![1, 2, 4, 7, 5, 3, 6, 8, 9]
    );
    assert_eq!(Solution::find_diagonal_order(vec![vec![1, 2], vec![3, 4]]), vec![1, 2, 3, 4]);
}
