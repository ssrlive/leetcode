#![allow(dead_code)]

// 221. Maximal Square
// https://leetcode.com/problems/maximal-square/
// https://leetcode.cn/problems/maximal-square/
//
// Given a 2D binary matrix filled with 0's and 1's, find the largest square
// containing only 1's and return its area.
//
// For example, given the following matrix:
//
// 1 0 1 0 0
// 1 0 1 1 1
// 1 1 1 1 1
// 1 0 0 1 0
//
// Return 4.
//

struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let mut max = 0;
        let mut dp = vec![vec![0; matrix[0].len()]; matrix.len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == '1' {
                    if i == 0 || j == 0 {
                        dp[i][j] = 1;
                    } else {
                        dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i - 1][j].min(dp[i][j - 1]));
                    }
                    max = max.max(dp[i][j]);
                }
            }
        }

        max * max
    }
}

#[test]
fn test_maximal_square() {
    let matrix = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ];
    assert_eq!(Solution::maximal_square(matrix), 4);

    let matrix = vec![vec!['0', '1'], vec!['1', '0']];
    assert_eq!(Solution::maximal_square(matrix), 1);

    let matrix = vec![vec!['0']];
    assert_eq!(Solution::maximal_square(matrix), 0);
}
