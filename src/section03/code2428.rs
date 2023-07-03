#![allow(dead_code)]

// 2428. Maximum Sum of an Hourglass
// https://leetcode.com/problems/maximum-sum-of-an-hourglass/
// https://leetcode.cn/problems/maximum-sum-of-an-hourglass/
//
// You are given an m x n integer matrix grid.
//
// We define an hourglass as a part of the matrix with the following form:
//
// Return the maximum sum of the elements of an hourglass.
//
// Note that an hourglass cannot be rotated and must be entirely contained within the matrix.
//
// Example 1:
//
// Input: grid = [[6,2,1,3],[4,2,1,5],[9,2,8,7],[4,1,2,9]]
// Output: 30
// Explanation: The cells shown above represent the hourglass with the maximum sum: 6 + 2 + 1 + 2 + 9 + 2 + 8 = 30.
//
// Example 2:
//
// Input: grid = [[1,2,3],[4,5,6],[7,8,9]]
// Output: 35
// Explanation: There is only one hourglass in the matrix, with the sum: 1 + 2 + 3 + 5 + 7 + 8 + 9 = 35.
//
// Constraints:
//
// - m == grid.length
// - n == grid[i].length
// - 3 <= m, n <= 150
// - 0 <= grid[i][j] <= 10^6
//

struct Solution;

impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
        let ends: Vec<Vec<i32>> = grid.iter().map(|row| row.windows(3).map(|e| e.iter().sum()).collect()).collect();
        let middles: Vec<Vec<i32>> = grid.iter().map(|row| row.windows(3).map(|e| e[1]).collect()).collect();
        ends.iter()
            .zip(middles.iter().skip(1))
            .zip(ends.iter().skip(2))
            .flat_map(|((top, middle), bottom)| top.iter().zip(middle.iter()).zip(bottom.iter()).map(|((t, m), b)| *t + *m + *b))
            .max()
            .unwrap()
    }
}

#[test]
fn test() {
    let grid = vec![vec![6, 2, 1, 3], vec![4, 2, 1, 5], vec![9, 2, 8, 7], vec![4, 1, 2, 9]];
    assert_eq!(Solution::max_sum(grid), 30);

    let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    assert_eq!(Solution::max_sum(grid), 35);
}
