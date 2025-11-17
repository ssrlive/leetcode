#![allow(dead_code)]

// 3643. Flip Square Submatrix Vertically
// https://leetcode.com/problems/flip-square-submatrix-vertically/
// https://leetcode.cn/problems/flip-square-submatrix-vertically/
//
// Easy
//
// You are given an m x n integer matrix grid, and three integers x, y, and k.
//
// The integers x and y represent the row and column indices of the top-left corner of a square submatrix and the integer k represents the size (side length) of the square submatrix.
//
// Your task is to flip the submatrix by reversing the order of its rows vertically.
//
// Return the updated matrix.
//
// Example 1:
//
// Input: grid = [[1,2,3,4],[5,6,7,8],[9,10,11,12],[13,14,15,16]], x = 1, y = 0, k = 3
//
// Output: [[1,2,3,4],[13,14,15,8],[9,10,11,12],[5,6,7,16]]
//
// Explanation:
//
// The diagram above shows the grid before and after the transformation.
//
// Example 2:
//
// ​Input: grid = [[3,4,2,3],[2,3,4,2]], x = 0, y = 2, k = 2
//
// Output: [[3,4,4,2],[2,3,2,3]]
//
// Explanation:
//
// The diagram above shows the grid before and after the transformation.
//
// Constraints:
//
// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 50
// 1 <= grid[i][j] <= 100
// 0 <= x < m
// 0 <= y < n
// 1 <= k <= min(m - x, n - y)
//

struct Solution;

impl Solution {
    pub fn reverse_submatrix(grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let mut grid = grid;
        let n = k / 2;
        let r_start = x as usize;
        let r_end = (x + k - 1) as usize;

        for r in 0..n {
            let r = r as usize;
            for c in y..(y + k) {
                let c = c as usize;
                let a = grid[r_start + r][c];
                let tmp = grid[r_end - r][c];
                grid[r_end - r][c] = a;
                grid[r_start + r][c] = tmp;
            }
        }

        grid
    }
}

#[test]
fn test() {
    let grid = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12], vec![13, 14, 15, 16]];
    let x = 1;
    let y = 0;
    let k = 3;
    let result = Solution::reverse_submatrix(grid, x, y, k);
    let expected = vec![vec![1, 2, 3, 4], vec![13, 14, 15, 8], vec![9, 10, 11, 12], vec![5, 6, 7, 16]];
    assert_eq!(result, expected);

    let grid = vec![vec![3, 4, 2, 3], vec![2, 3, 4, 2]];
    let x = 0;
    let y = 2;
    let k = 2;
    let result = Solution::reverse_submatrix(grid, x, y, k);
    let expected = vec![vec![3, 4, 4, 2], vec![2, 3, 2, 3]];
    assert_eq!(result, expected);
}
