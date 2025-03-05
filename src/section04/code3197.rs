#![allow(dead_code)]

// 3197. Find the Minimum Area to Cover All Ones II
// https://leetcode.com/problems/find-the-minimum-area-to-cover-all-ones-ii/
// https://leetcode.cn/problems/find-the-minimum-area-to-cover-all-ones-ii/
//
// Hard
//
// You are given a 2D binary array grid. You need to find 3 non-overlapping rectangles having non-zero
// areas with horizontal and vertical sides such that all the 1's in grid lie inside these rectangles.
//
// Return the minimum possible sum of the area of these rectangles.
//
// Note that the rectangles are allowed to touch.
//
// Example 1:
//
// Input: grid = [[1,0,1],[1,1,1]]
//
// Output: 5
//
// Explanation:
//
//     The 1's at (0, 0) and (1, 0) are covered by a rectangle of area 2.
//     The 1's at (0, 2) and (1, 2) are covered by a rectangle of area 2.
//     The 1 at (1, 1) is covered by a rectangle of area 1.
//
// Example 2:
//
// Input: grid = [[1,0,1,0],[0,1,0,1]]
//
// Output: 5
//
// Explanation:
//
//     The 1's at (0, 0) and (0, 2) are covered by a rectangle of area 3.
//     The 1 at (1, 1) is covered by a rectangle of area 1.
//     The 1 at (1, 3) is covered by a rectangle of area 1.
//
// Constraints:
//
//     1 <= grid.length, grid[i].length <= 30
//     grid[i][j] is either 0 or 1.
//     The input is generated such that there are at least three 1's in grid.
//

struct Solution;

use std::cmp::{max, min};

impl Solution {
    pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        let mut rows = Vec::new();
        for (i, grid_i) in grid.iter().enumerate().take(m) {
            let mut tmp = (i, Vec::new());
            for (j, &grid_i_j) in grid_i.iter().enumerate().take(n) {
                if grid_i_j != 0 {
                    tmp.1.push(j);
                }
            }
            if !tmp.1.is_empty() {
                rows.push(tmp);
            }
        }

        let mut cols = Vec::new();
        for j in 0..n {
            let mut tmp = (j, Vec::new());
            for (i, grid_i) in grid.iter().enumerate().take(m) {
                if grid_i[j] != 0 {
                    tmp.1.push(i);
                }
            }
            if !tmp.1.is_empty() {
                cols.push(tmp);
            }
        }

        let mut row_diff = vec![(0, 0); rows.len()];
        let mut diff = (n, 0);
        for i in 0..rows.len() {
            diff.0 = diff.0.min(rows[i].1[0]);
            diff.1 = max(diff.1, *rows[i].1.last().unwrap());
            row_diff[i].0 = diff.1 - diff.0 + 1;
        }
        diff = (n, 0);
        for i in (0..rows.len()).rev() {
            diff.0 = diff.0.min(rows[i].1[0]);
            diff.1 = max(diff.1, *rows[i].1.last().unwrap());
            row_diff[i].1 = diff.1 - diff.0 + 1;
        }

        let mut col_diff = vec![(0, 0); cols.len()];
        diff = (m, 0);
        for i in 0..cols.len() {
            diff.0 = diff.0.min(cols[i].1[0]);
            diff.1 = max(diff.1, *cols[i].1.last().unwrap());
            col_diff[i].0 = diff.1 - diff.0 + 1;
        }
        diff = (m, 0);
        for i in (0..cols.len()).rev() {
            diff.0 = min(diff.0, cols[i].1[0]);
            diff.1 = max(diff.1, *cols[i].1.last().unwrap());
            col_diff[i].1 = diff.1 - diff.0 + 1;
        }

        let mut res = m * n;
        for i in 0..rows.len() - 1 {
            let upper = (rows[i].0 - rows[0].0 + 1) * row_diff[i].0;
            let downer = (rows.last().unwrap().0 - rows[i + 1].0 + 1) * row_diff[i + 1].1;

            let mut up_diff_left = Vec::new();
            let mut down_diff_left = Vec::new();
            let mut diff = ((m, 0), (m, 0));
            for cols_j in &cols {
                let c = &cols_j.1;
                let s = c.binary_search(&rows[i].0);
                match s {
                    Ok(sc) => {
                        diff.0.0 = min(diff.0.0, c[0]);
                        diff.0.1 = max(diff.0.1, c[sc]);
                        up_diff_left.push((cols_j.0, diff.0.1 - diff.0.0 + 1));
                        if sc + 1 < c.len() {
                            diff.1.0 = min(diff.1.0, c[sc + 1]);
                            diff.1.1 = max(diff.1.1, *c.last().unwrap());
                            down_diff_left.push((cols_j.0, diff.1.1 - diff.1.0 + 1));
                        }
                    }
                    Err(sc) => {
                        if sc > 0 {
                            diff.0.0 = min(diff.0.0, c[0]);
                            diff.0.1 = max(diff.0.1, c[sc - 1]);
                            up_diff_left.push((cols_j.0, diff.0.1 - diff.0.0 + 1));
                        }
                        if sc < c.len() {
                            diff.1.0 = min(diff.1.0, c[sc]);
                            diff.1.1 = max(diff.1.1, *c.last().unwrap());
                            down_diff_left.push((cols_j.0, diff.1.1 - diff.1.0 + 1));
                        }
                    }
                }
            }
            let mut up_diff_right = Vec::new();
            let mut down_diff_right = Vec::new();
            diff = ((m, 0), (m, 0));
            for j in (0..cols.len()).rev() {
                let c = &cols[j].1;
                let s = c.binary_search(&rows[i].0);
                match s {
                    Ok(sc) => {
                        diff.0.0 = min(diff.0.0, c[0]);
                        diff.0.1 = max(diff.0.1, c[sc]);
                        up_diff_right.push((cols[j].0, diff.0.1 - diff.0.0 + 1));
                        if sc + 1 < c.len() {
                            diff.1.0 = min(diff.1.0, c[sc + 1]);
                            diff.1.1 = max(diff.1.1, *c.last().unwrap());
                            down_diff_right.push((cols[j].0, diff.1.1 - diff.1.0 + 1));
                        }
                    }
                    Err(sc) => {
                        if sc > 0 {
                            diff.0.0 = min(diff.0.0, c[0]);
                            diff.0.1 = max(diff.0.1, c[sc - 1]);
                            up_diff_right.push((cols[j].0, diff.0.1 - diff.0.0 + 1));
                        }
                        if sc < c.len() {
                            diff.1.0 = min(diff.1.0, c[sc]);
                            diff.1.1 = max(diff.1.1, *c.last().unwrap());
                            down_diff_right.push((cols[j].0, diff.1.1 - diff.1.0 + 1));
                        }
                    }
                }
            }

            let mut up_sep = m * n;
            for j in 0..up_diff_left.len() - 1 {
                let jj = up_diff_left.len() - 2 - j;
                up_sep = min(
                    up_sep,
                    up_diff_left[j].1 * (up_diff_left[j].0 - up_diff_left[0].0 + 1)
                        + up_diff_right[jj].1 * (up_diff_right[0].0 - up_diff_right[jj].0 + 1),
                );
            }
            let mut down_sep = m * n;
            for j in 0..down_diff_left.len() - 1 {
                let jj = down_diff_left.len() - 2 - j;
                down_sep = min(
                    down_sep,
                    down_diff_left[j].1 * (down_diff_left[j].0 - down_diff_left[0].0 + 1)
                        + down_diff_right[jj].1 * (down_diff_right[0].0 - down_diff_right[jj].0 + 1),
                );
            }
            res = min(res, min(up_sep + downer, down_sep + upper));

            let mut diff = (n, 0);
            let mut down_sep_h = m * n;
            for ii in i + 1..rows.len() - 1 {
                diff.0 = min(diff.0, rows[ii].1[0]);
                diff.1 = max(diff.1, *rows[ii].1.last().unwrap());
                down_sep_h = min(
                    down_sep_h,
                    (diff.1 - diff.0 + 1) * (rows[ii].0 - rows[i + 1].0 + 1)
                        + row_diff[ii + 1].1 * (rows.last().unwrap().0 - rows[ii + 1].0 + 1),
                );
            }
            res = min(res, down_sep_h + upper);
        }

        for j in 0..cols.len() - 1 {
            let lefter = (cols[j].0 - cols[0].0 + 1) * col_diff[j].0;
            let righter = (cols.last().unwrap().0 - cols[j + 1].0 + 1) * col_diff[j + 1].1;

            let mut left_diff_up = Vec::new();
            let mut right_diff_up = Vec::new();
            let mut diff = ((n, 0), (n, 0));
            for rows_i in &rows {
                let r = &rows_i.1;
                let s = r.binary_search(&cols[j].0);
                match s {
                    Ok(sc) => {
                        diff.0.0 = min(diff.0.0, r[0]);
                        diff.0.1 = max(diff.0.1, r[sc]);
                        left_diff_up.push((rows_i.0, diff.0.1 - diff.0.0 + 1));
                        if sc + 1 < r.len() {
                            diff.1.0 = min(diff.1.0, r[sc + 1]);
                            diff.1.1 = max(diff.1.1, *r.last().unwrap());
                            right_diff_up.push((rows_i.0, diff.1.1 - diff.1.0 + 1));
                        }
                    }
                    Err(sc) => {
                        if sc > 0 {
                            diff.0.0 = min(diff.0.0, r[0]);
                            diff.0.1 = max(diff.0.1, r[sc - 1]);
                            left_diff_up.push((rows_i.0, diff.0.1 - diff.0.0 + 1));
                        }
                        if sc < r.len() {
                            diff.1.0 = min(diff.1.0, r[sc]);
                            diff.1.1 = max(diff.1.1, *r.last().unwrap());
                            right_diff_up.push((rows_i.0, diff.1.1 - diff.1.0 + 1));
                        }
                    }
                }
            }
            let mut left_diff_down = Vec::new();
            let mut right_diff_down = Vec::new();
            diff = ((n, 0), (n, 0));
            for i in (0..rows.len()).rev() {
                let r = &rows[i].1;
                let s = r.binary_search(&cols[j].0);
                match s {
                    Ok(sc) => {
                        diff.0.0 = min(diff.0.0, r[0]);
                        diff.0.1 = max(diff.0.1, r[sc]);
                        left_diff_down.push((rows[i].0, diff.0.1 - diff.0.0 + 1));
                        if sc + 1 < r.len() {
                            diff.1.0 = min(diff.1.0, r[sc + 1]);
                            diff.1.1 = max(diff.1.1, *r.last().unwrap());
                            right_diff_down.push((rows[i].0, diff.1.1 - diff.1.0 + 1));
                        }
                    }
                    Err(sc) => {
                        if sc > 0 {
                            diff.0.0 = min(diff.0.0, r[0]);
                            diff.0.1 = max(diff.0.1, r[sc - 1]);
                            left_diff_down.push((rows[i].0, diff.0.1 - diff.0.0 + 1));
                        }
                        if sc < r.len() {
                            diff.1.0 = min(diff.1.0, r[sc]);
                            diff.1.1 = max(diff.1.1, *r.last().unwrap());
                            right_diff_down.push((rows[i].0, diff.1.1 - diff.1.0 + 1));
                        }
                    }
                }
            }

            let mut left_sep = m * n;
            for i in 0..left_diff_up.len() - 1 {
                let ii = left_diff_up.len() - 2 - i;
                left_sep = min(
                    left_sep,
                    left_diff_up[i].1 * (left_diff_up[i].0 - left_diff_up[0].0 + 1)
                        + left_diff_down[ii].1 * (left_diff_down[0].0 - left_diff_down[ii].0 + 1),
                );
            }
            let mut right_sep = m * n;
            for i in 0..right_diff_up.len() - 1 {
                let ii = right_diff_up.len() - 2 - i;
                right_sep = min(
                    right_sep,
                    right_diff_up[i].1 * (right_diff_up[i].0 - right_diff_up[0].0 + 1)
                        + right_diff_down[ii].1 * (right_diff_down[0].0 - right_diff_down[ii].0 + 1),
                );
            }
            res = min(res, min(left_sep + righter, right_sep + lefter));

            let mut diff = (m, 0);
            let mut right_sep_v = m * n;
            for jj in j + 1..cols.len() - 1 {
                diff.0 = min(diff.0, cols[jj].1[0]);
                diff.1 = max(diff.1, *cols[jj].1.last().unwrap());
                right_sep_v = min(
                    right_sep_v,
                    (diff.1 - diff.0 + 1) * (cols[jj].0 - cols[j + 1].0 + 1)
                        + col_diff[jj + 1].1 * (cols.last().unwrap().0 - cols[jj + 1].0 + 1),
                );
            }
            res = res.min(right_sep_v + lefter);
        }
        res as i32
    }
}

#[test]
fn test() {
    let grid = vec![vec![1, 0, 1], vec![1, 1, 1]];
    let res = 5;
    assert_eq!(Solution::minimum_sum(grid), res);

    let grid = vec![vec![1, 0, 1, 0], vec![0, 1, 0, 1]];
    let res = 5;
    assert_eq!(Solution::minimum_sum(grid), res);
}
