#![allow(dead_code)]

// 861. Score After Flipping Matrix
// https://leetcode.com/problems/score-after-flipping-matrix/
// https://leetcode.cn/problems/score-after-flipping-matrix/
//
// You are given an m x n binary matrix grid.
//
// A move consists of choosing any row or column and toggling each value in that row or column (i.e., changing all 0's to 1's, and all 1's to 0's).
//
// Every row of the matrix is interpreted as a binary number, and the score of the matrix is the sum of these numbers.
//
// Return the highest possible score after making any number of moves (including zero moves).
//
// Example 1:
//
// Input: grid = [[0,0,1,1],[1,0,1,0],[1,1,0,0]]
// Output: 39
// Explanation: 0b1111 + 0b1001 + 0b1111 = 15 + 9 + 15 = 39
//
// Example 2:
//
// Input: grid = [[0]]
// Output: 1
//
// Constraints:
//
// - m == grid.length
// - n == grid[i].length
// - 1 <= m, n <= 20
// - grid[i][j] is either 0 or 1.
//

struct Solution;

impl Solution {
    pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        for row in grid.iter_mut() {
            if row[0] == 0 {
                for col in row.iter_mut() {
                    *col = 1 - *col;
                }
            }
        }
        for col in 1..grid[0].len() {
            let mut n1 = 0;
            for row in grid.iter() {
                if row[col] == 1 {
                    n1 += 1;
                }
            }
            if n1 < grid.len() - n1 {
                for row in grid.iter_mut() {
                    row[col] = 1 - row[col];
                }
            }
        }
        let mut score = 0;
        for row in grid.iter() {
            let mut current = 0;
            for col in row.iter() {
                current = current * 2 + col;
            }
            score += current;
        }
        score
    }
}

#[test]
fn test() {
    let grid = vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]];
    assert_eq!(Solution::matrix_score(grid), 39);

    let grid = vec![vec![0]];
    assert_eq!(Solution::matrix_score(grid), 1);
}
