#![allow(dead_code)]

/*

// 2319. Check if Matrix Is X-Matrix
// https://leetcode.com/problems/check-if-matrix-is-x-matrix/
// https://leetcode.cn/problems/check-if-matrix-is-x-matrix/
//
// Easy
//
// A square matrix is said to be an X-Matrix if both of the following conditions hold:

    All the elements in the diagonals of the matrix are non-zero.
    All other elements are 0.

Given a 2D integer array grid of size n x n representing a square matrix, return true if grid is an X-Matrix. Otherwise, return false.

Example 1:

Input: grid = [[2,0,0,1],[0,3,1,0],[0,5,2,0],[4,0,0,2]]
Output: true
Explanation: Refer to the diagram above.
An X-Matrix should have the green elements (diagonals) be non-zero and the red elements be 0.
Thus, grid is an X-Matrix.

Example 2:

Input: grid = [[5,7,0],[0,3,1],[0,5,0]]
Output: false
Explanation: Refer to the diagram above.
An X-Matrix should have the green elements (diagonals) be non-zero and the red elements be 0.
Thus, grid is not an X-Matrix.

Constraints:

    n == grid.length == grid[i].length
    3 <= n <= 100
    0 <= grid[i][j] <= 10^5
*/

struct Solution;

impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        grid.iter().enumerate().all(|(i, row)| {
            row.iter().enumerate().all(|(j, &x)| match i == j || i == (n - j - 1) {
                true => x > 0,
                false => x == 0,
            })
        })
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (vec![vec![2, 0, 0, 1], vec![0, 3, 1, 0], vec![0, 5, 2, 0], vec![4, 0, 0, 2]], true),
        (vec![vec![5, 7, 0], vec![0, 3, 1], vec![0, 5, 0]], false),
    ];
    for (grid, expected) in cases {
        assert_eq!(Solution::check_x_matrix(grid), expected);
    }
}
