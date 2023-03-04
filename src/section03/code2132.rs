#![allow(dead_code)]

/*

// 2132. Stamping the Grid
// https://leetcode.com/problems/stamping-the-grid/
// https://leetcode.cn/problems/stamping-the-grid/
//
// Hard
//
// You are given an m x n binary matrix grid where each cell is either 0 (empty) or 1 (occupied).

You are then given stamps of size stampHeight x stampWidth. We want to fit the stamps such that they follow the given restrictions and requirements:

    Cover all the empty cells.
    Do not cover any of the occupied cells.
    We can put as many stamps as we want.
    Stamps can overlap with each other.
    Stamps are not allowed to be rotated.
    Stamps must stay completely inside the grid.

Return true if it is possible to fit the stamps while following the given restrictions and requirements. Otherwise, return false.

Example 1:

Input: grid = [[1,0,0,0],[1,0,0,0],[1,0,0,0],[1,0,0,0],[1,0,0,0]], stampHeight = 4, stampWidth = 3
Output: true
Explanation: We have two overlapping stamps (labeled 1 and 2 in the image) that are able to cover all the empty cells.

Example 2:

Input: grid = [[1,0,0,0],[0,1,0,0],[0,0,1,0],[0,0,0,1]], stampHeight = 2, stampWidth = 2
Output: false
Explanation: There is no way to fit the stamps onto all the empty cells without the stamps going outside the grid.

Constraints:

    m == grid.length
    n == grid[r].length
    1 <= m, n <= 10^5
    1 <= m * n <= 2 * 10^5
    grid[r][c] is either 0 or 1.
    1 <= stampHeight, stampWidth <= 10^5
*/

struct Solution;

impl Solution {
    pub fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
        fn pref2d(v: &[Vec<i32>], m: usize, n: usize) -> Vec<Vec<i32>> {
            let mut p = vec![vec![0; n + 1]; m + 1];
            for i in 0..m {
                for j in 0..n {
                    p[i + 1][j + 1] = v[i][j] + p[i + 1][j] + p[i][j + 1] - p[i][j];
                }
            }
            p
        }

        fn sum2d(p: &[Vec<i32>], c1: usize, r1: usize, c2: usize, r2: usize) -> i32 {
            p[c2 + 1][r2 + 1] + p[c1][r1] - p[c1][r2 + 1] - p[c2 + 1][r1]
        }

        let (stamp_height, stamp_width) = (stamp_height as usize, stamp_width as usize);
        let (m, n) = (grid.len(), grid[0].len());
        let mut stamp = vec![vec![0; n]; m];
        let pref = pref2d(&grid, m, n);
        for (i, stamp_i) in stamp.iter_mut().enumerate().skip(stamp_height - 1) {
            for (j, stamp_i_j) in stamp_i.iter_mut().enumerate().skip(stamp_width - 1) {
                let v = sum2d(&pref, i + 1 - stamp_height, j + 1 - stamp_width, i, j);
                *stamp_i_j = if v == 0 { 1 } else { 0 };
            }
        }
        let pref2 = pref2d(&stamp, m, n);
        for (i, grid_i) in grid.iter().enumerate() {
            for (j, &grid_i_j) in grid_i.iter().enumerate() {
                let c2 = (m - 1).min(i + stamp_height - 1);
                let r2 = (n - 1).min(j + stamp_width - 1);
                if grid_i_j == 0 && sum2d(&pref2, i, j, c2, r2) == 0 {
                    return false;
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0],
            ],
            4,
            3,
            true,
        ),
        (
            vec![vec![1, 0, 0, 0], vec![0, 1, 0, 0], vec![0, 0, 1, 0], vec![0, 0, 0, 1]],
            2,
            2,
            false,
        ),
    ];
    for (grid, h, w, ex) in cases {
        assert_eq!(Solution::possible_to_stamp(grid, h, w), ex);
    }
}
