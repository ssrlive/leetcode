#![allow(dead_code)]

// 329. Longest Increasing Path in a Matrix
// https://leetcode.com/problems/longest-increasing-path-in-a-matrix/
//
// Given an integer matrix, find the length of the longest increasing path.
//
// From each cell, you can either move to four directions: left, right, up or
// down. You may NOT move diagonally or move outside of the boundary (i.e.
// wrap-around is not allowed).
//
// Example 1:
//
// Input: nums =
// [
//   [9,9,4],
//   [6,6,8],
//   [2,1,1]
// ]
// Output: 4
// Explanation: The longest increasing path is [1, 2, 6, 9].
//
// Example 2:
//
// Input: nums =
// [
//   [3,4,5],
//   [3,2,6],
//   [2,2,1]
// ]
// Output: 4
// Explanation: The longest increasing path is [3, 4, 5, 6]. Moving diagonally is
// not allowed.
//

struct Solution;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let mut cache = vec![vec![0; matrix[0].len()]; matrix.len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                max = std::cmp::max(max, Self::dfs(&matrix, i, j, &mut cache));
            }
        }

        max
    }

    fn dfs(matrix: &Vec<Vec<i32>>, i: usize, j: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
        if cache[i][j] != 0 {
            return cache[i][j];
        }

        let mut max = 1;

        if i > 0 && matrix[i - 1][j] > matrix[i][j] {
            max = std::cmp::max(max, 1 + Self::dfs(matrix, i - 1, j, cache));
        }

        if i < matrix.len() - 1 && matrix[i + 1][j] > matrix[i][j] {
            max = std::cmp::max(max, 1 + Self::dfs(matrix, i + 1, j, cache));
        }

        if j > 0 && matrix[i][j - 1] > matrix[i][j] {
            max = std::cmp::max(max, 1 + Self::dfs(matrix, i, j - 1, cache));
        }

        if j < matrix[0].len() - 1 && matrix[i][j + 1] > matrix[i][j] {
            max = std::cmp::max(max, 1 + Self::dfs(matrix, i, j + 1, cache));
        }

        cache[i][j] = max;
        max
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::longest_increasing_path(vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]),
        4
    );

    assert_eq!(
        Solution::longest_increasing_path(vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]]),
        4
    );
}
