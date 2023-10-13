#![allow(dead_code)]

// 240. Search a 2D Matrix II
// https://leetcode.com/problems/search-a-2d-matrix-ii/
// https://leetcode.cn/problems/search-a-2d-matrix-ii/
//
// Write an efficient algorithm that searches for a value in an m x n matrix.
// This matrix has the following properties:
//
// Integers in each row are sorted in ascending from left to right.
// Integers in each column are sorted in ascending from top to bottom.
//
// Example:
//
// Consider the following matrix:
//
// [
//   [1,   4,  7, 11, 15],
//   [2,   5,  8, 12, 19],
//   [3,   6,  9, 16, 22],
//   [10, 13, 14, 17, 24],
//   [18, 21, 23, 26, 30]
// ]
//
// Given target = 5, return true.
//
// Given target = 20, return false.
//

struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut row = 0;
        let mut col = matrix[0].len() - 1;
        while row < matrix.len() && col < matrix[0].len() {
            match matrix[row][col].cmp(&target) {
                std::cmp::Ordering::Less => row += 1,
                std::cmp::Ordering::Greater => {
                    if col == 0 {
                        break;
                    }
                    col -= 1;
                }
                std::cmp::Ordering::Equal => return true,
            }
        }
        false
    }
}

#[test]
fn test() {
    let matrix = vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ];

    assert!(Solution::search_matrix(matrix.clone(), 5));
    assert!(!Solution::search_matrix(matrix, 20));
}
