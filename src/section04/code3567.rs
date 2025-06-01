#![allow(dead_code)]

// 3567. Minimum Absolute Difference in Sliding Submatrix
// https://leetcode.com/problems/minimum-absolute-difference-in-sliding-submatrix/
// https://leetcode.cn/problems/minimum-absolute-difference-in-sliding-submatrix/
//
// Medium
//
// You are given an m x n integer matrix grid and an integer k.
//
// For every contiguous k x k submatrix of grid, compute the minimum absolute difference between any two distinct values within that submatrix.
//
// Return a 2D array ans of size (m - k + 1) x (n - k + 1), where ans[i][j] is the minimum absolute difference in the submatrix whose top-left corner is (i, j) in grid.
//
// Note: If all elements in the submatrix have the same value, the answer will be 0.
// A submatrix (x1, y1, x2, y2) is a matrix that is formed by choosing all cells matrix[x][y] where x1 <= x <= x2 and y1 <= y <= y2.
//
// Example 1:
//
// Input: grid = [[1,8],[3,-2]], k = 2
//
// Output: [[2]]
//
// Explanation:
//
//     There is only one possible k x k submatrix: [[1, 8], [3, -2]].
//     Distinct values in the submatrix are [1, 8, 3, -2].
//     The minimum absolute difference in the submatrix is |1 - 3| = 2. Thus, the answer is [[2]].
//
// Example 2:
//
// Input: grid = [[3,-1]], k = 1
//
// Output: [[0,0]]
//
// Explanation:
//
//     Both k x k submatrix has only one distinct element.
//     Thus, the answer is [[0, 0]].
//
// Example 3:
//
// Input: grid = [[1,-2,3],[2,3,5]], k = 2
//
// Output: [[1,2]]
//
// Explanation:
//
//     There are two possible k × k submatrix:
//         Starting at (0, 0): [[1, -2], [2, 3]].
//             Distinct values in the submatrix are [1, -2, 2, 3].
//             The minimum absolute difference in the submatrix is |1 - 2| = 1.
//         Starting at (0, 1): [[-2, 3], [3, 5]].
//             Distinct values in the submatrix are [-2, 3, 5].
//             The minimum absolute difference in the submatrix is |3 - 5| = 2.
//     Thus, the answer is [[1, 2]].
//
// Constraints:
//
//     1 <= m == grid.length <= 30
//     1 <= n == grid[i].length <= 30
//     -10^5 <= grid[i][j] <= 10^5
//     1 <= k <= min(m, n)
//

struct Solution;

impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let rows = m - k as usize + 1;
        let cols = n - k as usize + 1;
        let mut ans = vec![vec![0; cols]; rows];

        for (i, ans_i) in ans.iter_mut().enumerate().take(rows) {
            for (j, ans_i_j) in ans_i.iter_mut().enumerate().take(cols) {
                let mut vals = Vec::with_capacity((k * k) as usize);
                for grid_x in grid.iter().skip(i).take(k as usize) {
                    for &grid_x_y in grid_x.iter().skip(j).take(k as usize) {
                        vals.push(grid_x_y);
                    }
                }
                vals.sort_unstable();
                let mut uniq = Vec::with_capacity((k * k) as usize);
                uniq.push(vals[0]);
                for &val in vals.iter().skip(1) {
                    if val != *uniq.last().unwrap() {
                        uniq.push(val);
                    }
                }
                if uniq.len() <= 1 {
                    *ans_i_j = 0;
                    continue;
                }
                let mut mini = i32::MAX;
                for t in 1..uniq.len() {
                    let dif = uniq[t] - uniq[t - 1];
                    if dif < mini {
                        mini = dif;
                    }
                    if mini == 0 {
                        break;
                    }
                }
                *ans_i_j = mini;
            }
        }
        ans
    }
}

#[test]
fn test() {
    let grid = vec![vec![1, 8], vec![3, -2]];
    let k = 2;
    let expected = vec![vec![2]];
    assert_eq!(Solution::min_abs_diff(grid, k), expected);

    let grid = vec![vec![3, -1]];
    let k = 1;
    let expected = vec![vec![0, 0]];
    assert_eq!(Solution::min_abs_diff(grid, k), expected);

    let grid = vec![vec![1, -2, 3], vec![2, 3, 5]];
    let k = 2;
    let expected = vec![vec![1, 2]];
    assert_eq!(Solution::min_abs_diff(grid, k), expected);
}
