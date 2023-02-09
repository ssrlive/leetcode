#![allow(dead_code)]

/*

// 1504. Count Submatrices With All Ones
// https://leetcode.com/problems/count-submatrices-with-all-ones/
// https://leetcode.cn/problems/count-submatrices-with-all-ones/
//
// Medium
//
// Given an m x n binary matrix mat, return the number of submatrices that have all ones.
//
// Example 1:
//
Input: mat = [[1,0,1],[1,1,0],[1,1,0]]
Output: 13
Explanation:
There are 6 rectangles of side 1x1.
There are 2 rectangles of side 1x2.
There are 3 rectangles of side 2x1.
There is 1 rectangle of side 2x2.
There is 1 rectangle of side 3x1.
Total number of rectangles = 6 + 2 + 3 + 1 + 1 = 13.

Example 2:

Input: mat = [[0,1,1,0],[0,1,1,1],[1,1,1,0]]
Output: 24
Explanation:
There are 8 rectangles of side 1x1.
There are 5 rectangles of side 1x2.
There are 2 rectangles of side 1x3.
There are 4 rectangles of side 2x1.
There are 2 rectangles of side 2x2.
There are 2 rectangles of side 3x1.
There is 1 rectangle of side 3x2.
Total number of rectangles = 8 + 5 + 2 + 4 + 2 + 2 + 1 = 24.

Constraints:

    1 <= m, n <= 150
    mat[i][j] is either 0 or 1.
*/

struct Solution;

impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let m = mat.len();
        let n = mat[0].len();
        let mut dp = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 {
                    dp[i][j] = if j == 0 { 1 } else { dp[i][j - 1] + 1 };
                    let mut width = dp[i][j];
                    for k in (0..=i).rev() {
                        width = width.min(dp[k][j]);
                        ans += width;
                    }
                }
            }
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]], 13),
        (vec![vec![0, 1, 1, 0], vec![0, 1, 1, 1], vec![1, 1, 1, 0]], 24),
    ];
    for (mat, expected) in cases {
        assert_eq!(Solution::num_submat(mat), expected);
    }
}
