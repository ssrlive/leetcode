#![allow(dead_code)]

/*

// 1582. Special Positions in a Binary Matrix
Easy
582
21
Companies

Given an m x n binary matrix mat, return the number of special positions in mat.

A position (i, j) is called special if mat[i][j] == 1 and all other elements in row i and column j are 0 (rows and columns are 0-indexed).

Example 1:

Input: mat = [[1,0,0],[0,0,1],[1,0,0]]
Output: 1
Explanation: (1, 2) is a special position because mat[1][2] == 1 and all other elements in row 1 and column 2 are 0.

Example 2:

Input: mat = [[1,0,0],[0,1,0],[0,0,1]]
Output: 3
Explanation: (0, 0), (1, 1) and (2, 2) are special positions.

Constraints:

    m == mat.length
    n == mat[i].length
    1 <= m, n <= 100
    mat[i][j] is either 0 or 1.
*/

struct Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut row = vec![0; m];
        let mut col = vec![0; n];
        for i in 0..m {
            for (j, item_c) in col.iter_mut().enumerate().take(n) {
                if mat[i][j] == 1 {
                    row[i] += 1;
                    *item_c += 1;
                }
            }
        }
        let mut ans = 0;
        for i in 0..m {
            for (j, &item_c) in col.iter().enumerate().take(n) {
                if mat[i][j] == 1 && row[i] == 1 && item_c == 1 {
                    ans += 1;
                }
            }
        }
        ans
    }
}

#[test]
fn test() {
    let mat = vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]];
    let ans = 1;
    assert_eq!(Solution::num_special(mat), ans);
    let mat = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
    let ans = 3;
    assert_eq!(Solution::num_special(mat), ans);
    let mat = vec![vec![0, 0, 0, 1], vec![1, 0, 0, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]];
    let ans = 2;
    assert_eq!(Solution::num_special(mat), ans);
    let mat = vec![
        vec![0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 0, 1, 1],
    ];
    let ans = 3;
    assert_eq!(Solution::num_special(mat), ans);
}
