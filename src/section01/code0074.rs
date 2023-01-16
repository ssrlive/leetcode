#![allow(dead_code)]

// 74. Search a 2D Matrix
// https://leetcode.com/problems/search-a-2d-matrix/
// https://leetcode.cn/problems/search-a-2d-matrix/
//
// Write an efficient algorithm that searches for a value target in an m x n integer matrix matrix.
// This matrix has the following properties:
//
// Integers in each row are sorted from left to right.
// The first integer of each row is greater than the last integer of the previous row.
//
// Example 1:
//
// Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
// Output: true
//
// Example 2:
//
// Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
// Output: false
//
// Constraints:
//
// - m == matrix.length
// - n == matrix[i].length
// - 1 <= m, n <= 100
// - -10^4 <= matrix[i][j], target <= 10^4
//

struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        matrix.concat().binary_search(&target).is_ok()
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3,
            true,
        ),
        (
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            13,
            false,
        ),
    ];
    for (matrix, target, expected) in cases {
        assert_eq!(Solution::search_matrix(matrix, target), expected);
    }
}
