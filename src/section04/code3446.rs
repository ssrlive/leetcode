#![allow(dead_code)]

// 3446. Sort Matrix by Diagonals
// https://leetcode.com/problems/sort-matrix-by-diagonals/
// https://leetcode.cn/problems/sort-matrix-by-diagonals/
//
// Medium
//
// You are given an n x n square matrix of integers grid. Return the matrix such that:
//
//     The diagonals in the bottom-left triangle (including the middle diagonal) are sorted in non-increasing order.
//     The diagonals in the top-right triangle are sorted in non-decreasing order.
//
// Example 1:
//
// Input: grid = [[1,7,3],[9,8,2],[4,5,6]]
//
// Output: [[8,2,3],[9,6,7],[4,5,1]]
//
// Explanation:
//
// The diagonals with a black arrow (bottom-left triangle) should be sorted in non-increasing order:
//
//     [1, 8, 6] becomes [8, 6, 1].
//     [9, 5] and [4] remain unchanged.
//
// The diagonals with a blue arrow (top-right triangle) should be sorted in non-decreasing order:
//
//     [7, 2] becomes [2, 7].
//     [3] remains unchanged.
//
// Example 2:
//
// Input: grid = [[0,1],[1,2]]
//
// Output: [[2,1],[1,0]]
//
// Explanation:
//
// The diagonals with a black arrow must be non-increasing, so [0, 2] is changed to [2, 0].
// The other diagonals are already in the correct order.
//
// Example 3:
//
// Input: grid = [[1]]
//
// Output: [[1]]
//
// Explanation:
//
// Diagonals with exactly one element are already in order, so no changes are needed.
//
// Constraints:
//
//     grid.length == grid[i].length == n
//     1 <= n <= 10
//     -10^5 <= grid[i][j] <= 10^5
//

struct Solution;

impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::cmp::Reverse;

        fn sort(g: &mut [Vec<i32>], i: usize, j: usize) {
            let n = g.len();
            let mut v = Vec::with_capacity(n);
            for l in 0.. {
                if i + l == n || j + l == n {
                    break;
                }
                v.push(g[i + l][j + l])
            }
            if j == 0 {
                v.sort_unstable_by_key(|x| Reverse(*x))
            } else {
                v.sort_unstable()
            }
            for l in 0.. {
                if i + l == n || j + l == n {
                    break;
                }
                g[i + l][j + l] = v[l]
            }
        }

        let n = grid.len();
        for d in 0..2 * n - 1 {
            if d < n {
                sort(&mut grid, 0, d);
            } else {
                sort(&mut grid, 1 + d - n, 0);
            }
        }
        grid
    }
}

#[test]
fn test() {
    let grid = vec![vec![1, 7, 3], vec![9, 8, 2], vec![4, 5, 6]];
    let res = vec![vec![8, 2, 3], vec![9, 6, 7], vec![4, 5, 1]];
    assert_eq!(Solution::sort_matrix(grid), res);

    let grid = vec![vec![0, 1], vec![1, 2]];
    let res = vec![vec![2, 1], vec![1, 0]];
    assert_eq!(Solution::sort_matrix(grid), res);

    let grid = vec![vec![1]];
    let res = vec![vec![1]];
    assert_eq!(Solution::sort_matrix(grid), res);
}
