#![allow(dead_code)]

// 3225. Maximum Score From Grid Operations
// https://leetcode.com/problems/maximum-score-from-grid-operations/
// https://leetcode.cn/problems/maximum-score-from-grid-operations/
//
// Hard
//
// You are given a 2D matrix grid of size n x n. Initially, all cells of the grid are colored white.
// In one operation, you can select any cell of indices (i, j), and color black all the cells of
// the jth column starting from the top row down to the ith row.
//
// The grid score is the sum of all grid[i][j] such that cell (i, j) is white and it has a horizontally adjacent black cell.
//
// Return the maximum score that can be achieved after some number of operations.
//
// Example 1:
//
// Input: grid = [[0,0,0,0,0],[0,0,3,0,0],[0,1,0,0,0],[5,0,0,3,0],[0,0,0,0,2]]
//
// Output: 11
//
// Explanation:
//
// In the first operation, we color all cells in column 1 down to row 3, and in the second operation,
// we color all cells in column 4 down to the last row.
// The score of the resulting grid is grid[3][0] + grid[1][2] + grid[3][3] which is equal to 11.
//
// Example 2:
//
// Input: grid = [[10,9,0,0,15],[7,1,0,8,0],[5,20,0,11,0],[0,0,0,1,2],[8,12,1,10,3]]
//
// Output: 94
//
// Explanation:
//
// We perform operations on 1, 2, and 3 down to rows 1, 4, and 0, respectively.
// The score of the resulting grid is grid[0][0] + grid[1][0] + grid[2][1] + grid[4][1]
//   + grid[1][3] + grid[2][3] + grid[3][3] + grid[4][3] + grid[0][4] which is equal to 94.
//
// Constraints:
//
//     1 <= n == grid.length <= 100
//     n == grid[i].length
//     0 <= grid[i][j] <= 10^9
//

struct Solution;

impl Solution {
    pub fn maximum_score(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid.len();
        if n == 1 {
            return 0;
        }

        let mut pref_sum = Vec::new();
        for i in 0..n {
            let mut tmp = vec![0];
            for grid_j in grid.iter().take(n) {
                let b = tmp[tmp.len() - 1];
                tmp.push(b + grid_j[i] as i64);
            }
            pref_sum.push(tmp);
        }
        let mut prev = vec![vec![0; n + 1]; n + 1];
        for a in 0..=n {
            for (b, prev_b) in prev.iter_mut().enumerate().take(n + 1) {
                prev_b[a] = if a > b {
                    pref_sum[1][a] - pref_sum[1][b]
                } else {
                    pref_sum[0][b] - pref_sum[0][a]
                };
            }
        }
        for j in 2..n {
            let mut dp = vec![vec![0; n + 1]; n + 1];
            for b in 0..=n {
                for (c, dp_c) in dp.iter_mut().enumerate().take(n + 1) {
                    let mut m = 0;
                    for a in 0..=n {
                        let tmp = prev[b][a]
                            + if c > a && c > b {
                                pref_sum[j - 1][c] - pref_sum[j - 1][a.max(b)]
                            } else if c < b {
                                pref_sum[j][b] - pref_sum[j][c]
                            } else {
                                0
                            };
                        m = m.max(tmp);
                    }
                    dp_c[b] = m;
                }
            }
            prev = dp;
        }
        *prev.iter().map(|v| v.iter().max().unwrap()).max().unwrap()
    }
}

#[test]
fn test() {
    let grid = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 3, 0, 0],
        vec![0, 1, 0, 0, 0],
        vec![5, 0, 0, 3, 0],
        vec![0, 0, 0, 0, 2],
    ];
    let res = 11;
    assert_eq!(Solution::maximum_score(grid), res);

    let grid = vec![
        vec![10, 9, 0, 0, 15],
        vec![7, 1, 0, 8, 0],
        vec![5, 20, 0, 11, 0],
        vec![0, 0, 0, 1, 2],
        vec![8, 12, 1, 10, 3],
    ];
    let res = 94;
    assert_eq!(Solution::maximum_score(grid), res);
}
