#![allow(dead_code)]

// 3127. Make a Square with the Same Color
// https://leetcode.com/problems/make-a-square-with-the-same-color/
// https://leetcode.cn/problems/make-a-square-with-the-same-color/
//
// Easy
//
// You are given a 2D matrix grid of size 3 x 3 consisting only of characters 'B' and 'W'.
// Character 'W' represents the white color, and character 'B' represents the black color.
//
// Your task is to change the color of at most one cell so that the matrix has a 2 x 2 square where all cells are of the same color.
//
// Return true if it is possible to create a 2 x 2 square of the same color, otherwise, return false.
//
// Example 1:
//
// Input: grid = [["B","W","B"],["B","W","W"],["B","W","B"]]
//
// Output: true
//
// Explanation:
//
// It can be done by changing the color of the grid[0][2].
//
// Example 2:
//
// Input: grid = [["B","W","B"],["W","B","W"],["B","W","B"]]
//
// Output: false
//
// Explanation:
//
// It cannot be done by changing at most one cell.
//
// Example 3:
//
// Input: grid = [["B","W","B"],["B","W","W"],["B","W","W"]]
//
// Output: true
//
// Explanation:
//
// The grid already contains a 2 x 2 square of the same color.
//
// Constraints:
//
// grid.length == 3
// grid[i].length == 3
// grid[i][j] is either 'W' or 'B'.
//

struct Solution;

impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        const POSITIONS: &[&[(usize, usize); 4]] = &[
            &[(0, 0), (0, 1), (1, 0), (1, 1)],
            &[(0, 1), (0, 2), (1, 1), (1, 2)],
            &[(1, 0), (1, 1), (2, 0), (2, 1)],
            &[(1, 1), (1, 2), (2, 1), (2, 2)],
        ];

        POSITIONS.iter().any(|&p| {
            p.iter()
                .fold([-2; 2], |mut ret, p| {
                    if grid[p.0][p.1].eq(&'W') {
                        ret[0] += 1
                    } else {
                        ret[1] += 1
                    }
                    ret
                })
                .iter()
                .any(|x| *x > 0)
        })
    }
}

#[test]
fn test() {
    let grid = vec![vec!['B', 'W', 'B'], vec!['B', 'W', 'W'], vec!['B', 'W', 'B']];
    assert!(Solution::can_make_square(grid));

    let grid = vec![vec!['B', 'W', 'B'], vec!['W', 'B', 'W'], vec!['B', 'W', 'B']];
    assert!(!Solution::can_make_square(grid));

    let grid = vec![vec!['B', 'W', 'B'], vec!['B', 'W', 'W'], vec!['B', 'W', 'W']];
    assert!(Solution::can_make_square(grid));
}
