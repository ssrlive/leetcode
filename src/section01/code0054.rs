#![allow(dead_code)]

// 54. Spiral Matrix
// https://leetcode.com/problems/spiral-matrix/
// https://leetcode.cn/problems/spiral-matrix/
//
// Given an m x n matrix, return all elements of the matrix in spiral order.
//
// Example 1:
//
// Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
// Output: [1,2,3,6,9,8,7,4,5]
//
// Example 2:
//
// Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
// Output: [1,2,3,4,8,12,11,10,9,5,6,7]
//
// Constraints:
//
// - m == matrix.length
// - n == matrix[i].length
// - 1 <= m, n <= 10
// - -100 <= matrix[i][j] <= 100
//

struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        if matrix.is_empty() {
            return result;
        }

        let compass = [vec![0, 1], vec![1, 0], vec![0, -1], vec![-1, 0]];
        let mut direction = 0;
        let mut steps = [matrix[0].len(), matrix.len() - 1];
        let mut row = 0;
        let mut col = -1;

        while steps[direction % 2] > 0 {
            for _ in 0..steps[direction % 2] {
                row += compass[direction][0];
                col += compass[direction][1];
                result.push(matrix[row as usize][col as usize]);
            }
            steps[direction % 2] -= 1;
            direction = (direction + 1) % 4;
        }

        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![]], vec![]),
        (vec![vec![1]], vec![1]),
        (vec![vec![1, 2, 3]], vec![1, 2, 3]),
        (vec![vec![1], vec![2], vec![3]], vec![1, 2, 3]),
        (vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], vec![1, 2, 3, 6, 9, 8, 7, 4, 5]),
        (
            vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]],
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
        ),
    ];

    for (matrix, expected) in cases {
        assert_eq!(Solution::spiral_order(matrix), expected);
    }
}
