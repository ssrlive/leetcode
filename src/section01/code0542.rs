#![allow(dead_code)]

// 542. 01 Matrix
// https://leetcode.com/problems/01-matrix/
// https://leetcode.cn/problems/01-matrix/
//
// Given an m x n binary matrix mat, return the distance of the nearest 0 for each cell.
//
// The distance between two adjacent cells is 1.
//
// Example 1:
//
// Input: mat = [[0,0,0],[0,1,0],[0,0,0]]
// Output: [[0,0,0],[0,1,0],[0,0,0]]
//
// Example 2:
//
// Input: mat = [[0,0,0],[0,1,0],[1,1,1]]
// Output: [[0,0,0],[0,1,0],[1,2,1]]
//
// Constraints:
//
// - m == mat.length
// - n == mat[i].length
// - 1 <= m, n <= 10^4
// - 1 <= m * n <= 10^4
// - mat[i][j] is either 0 or 1.
// - There is at least one 0 in mat.
//

struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        let mut result = vec![vec![0; mat[0].len()]; mat.len()];

        for (i, row) in mat.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == 0 {
                    queue.push_back((i, j));
                } else {
                    result[i][j] = -1;
                }
            }
        }

        let mut distance = 0;
        while !queue.is_empty() {
            distance += 1;
            for _ in 0..queue.len() {
                let (i, j) = queue.pop_front().unwrap();
                for (di, dj) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let (ni, nj) = (i as i32 + di, j as i32 + dj);
                    if ni >= 0
                        && ni < mat.len() as i32
                        && nj >= 0
                        && nj < mat[0].len() as i32
                        && result[ni as usize][nj as usize] == -1
                    {
                        result[ni as usize][nj as usize] = distance;
                        queue.push_back((ni as usize, nj as usize));
                    }
                }
            }
        }

        result
    }
}

#[test]
fn test() {
    let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    let expected = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    assert_eq!(Solution::update_matrix(mat), expected);

    let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
    let expected = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]];
    assert_eq!(Solution::update_matrix(mat), expected);
}
