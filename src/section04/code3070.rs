#![allow(dead_code)]

/*

// 3070. Count Submatrices with Top-Left Element and Sum Less Than k
// https://leetcode.com/problems/count-submatrices-with-top-left-element-and-sum-less-than-k/
// https://leetcode.cn/problems/count-submatrices-with-top-left-element-and-sum-less-than-k/
//
// Medium
//
// You are given a 0-indexed integer matrix grid and an integer k.
//
// Return the number of submatrices that contain the top-left element of the grid, and have a sum less than or equal to k.
//
// Example 1:
//
// Input: grid = [[7,6,3],[6,6,1]], k = 18
// Output: 4
// Explanation: There are only 4 submatrices, shown in the image above, that contain the top-left element of grid, and have a sum less than or equal to 18.
//
// Example 2:
//
// Input: grid = [[7,2,9],[1,5,0],[2,6,6]], k = 20
// Output: 6
// Explanation: There are only 6 submatrices, shown in the image above, that contain the top-left element of grid, and have a sum less than or equal to 20.
//
// Constraints:
//
// m == grid.length
// n == grid[i].length
// 1 <= n, m <= 1000
// 0 <= grid[i][j] <= 1000
// 1 <= k <= 109
//
// */

struct Solution;

impl Solution {
    pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut grid = grid;
        let mut s = 0;
        let (yy, xx) = (grid.len(), grid[0].len());
        for cell in grid[0].iter_mut() {
            s += *cell;
            *cell = s;
        }

        for y in 1..yy {
            s = 0;
            let (prev, curr) = grid.split_at_mut(y);
            for (curr_cell, &prev_cell) in curr[0].iter_mut().zip(prev[y - 1].iter()) {
                s += *curr_cell;
                *curr_cell = prev_cell + s;
            }
        }

        let mut res = 0;
        for grid_y in grid.iter().take(yy) {
            for &grid_y_x in grid_y.iter().take(xx) {
                if grid_y_x <= k {
                    res += 1;
                }
            }
        }

        res
    }
}

#[test]
fn test() {
    let grid = vec![vec![7, 6, 3], vec![6, 6, 1]];
    let k = 18;
    assert_eq!(Solution::count_submatrices(grid, k), 4);

    let grid = vec![vec![7, 2, 9], vec![1, 5, 0], vec![2, 6, 6]];
    let k = 20;
    assert_eq!(Solution::count_submatrices(grid, k), 6);
}
