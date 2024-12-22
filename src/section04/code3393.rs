#![allow(dead_code)]

// 3393. Count Paths With the Given XOR Value
// https://leetcode.com/problems/count-paths-with-the-given-xor-value/
// https://leetcode.cn/problems/count-paths-with-the-given-xor-value/
//
// Medium
//
// You are given a 2D integer array grid with size m x n. You are also given an integer k.
//
// Your task is to calculate the number of paths you can take from the top-left cell (0, 0)
// to the bottom-right cell (m - 1, n - 1) satisfying the following constraints:
//
// You can either move to the right or down. Formally, from the cell (i, j) you may move to
// the cell (i, j + 1) or to the cell (i + 1, j) if the target cell exists.
//
// The XOR of all the numbers on the path must be equal to k.
// Return the total number of such paths.
//
// Since the answer can be very large, return the result modulo 109 + 7.
//
// Example 1:
//
// Input: grid = [[2, 1, 5], [7, 10, 0], [12, 6, 4]], k = 11
//
// Output: 3
//
// Explanation:
//
// The 3 paths are:
//
// (0, 0) → (1, 0) → (2, 0) → (2, 1) → (2, 2)
// (0, 0) → (1, 0) → (1, 1) → (1, 2) → (2, 2)
// (0, 0) → (0, 1) → (1, 1) → (2, 1) → (2, 2)
//
// Example 2:
//
// Input: grid = [[1, 3, 3, 3], [0, 3, 3, 2], [3, 0, 1, 1]], k = 2
//
// Output: 5
//
// Explanation:
//
// The 5 paths are:
//
// (0, 0) → (1, 0) → (2, 0) → (2, 1) → (2, 2) → (2, 3)
// (0, 0) → (1, 0) → (1, 1) → (2, 1) → (2, 2) → (2, 3)
// (0, 0) → (1, 0) → (1, 1) → (1, 2) → (1, 3) → (2, 3)
// (0, 0) → (0, 1) → (1, 1) → (1, 2) → (2, 2) → (2, 3)
// (0, 0) → (0, 1) → (0, 2) → (1, 2) → (2, 2) → (2, 3)
//
// Example 3:
//
// Input: grid = [[1, 1, 1, 2], [3, 0, 3, 2], [3, 0, 2, 2]], k = 10
//
// Output: 0
//
// Constraints:
//
// 1 <= m == grid.length <= 300
// 1 <= n == grid[r].length <= 300
// 0 <= grid[r][c] < 16
// 0 <= k < 16
//

struct Solution;

impl Solution {
    pub fn count_paths_with_xor_value(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        fn f(i: i32, j: i32, mut curr: i32, grid: &[Vec<i32>], k: i32, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
            let n = grid.len() as i32;
            let m = grid[0].len() as i32;
            if i >= n || j >= m {
                return 0;
            }
            curr ^= grid[i as usize][j as usize];
            if i == n - 1 && j == m - 1 {
                return (curr == k) as i32;
            }
            if dp[i as usize][j as usize][curr as usize] != -1 {
                return dp[i as usize][j as usize][curr as usize];
            }
            let right = f(i, j + 1, curr, grid, k, dp) % 1000000007;
            let down = f(i + 1, j, curr, grid, k, dp) % 1000000007;
            dp[i as usize][j as usize][curr as usize] = (right + down) % 1000000007;
            dp[i as usize][j as usize][curr as usize]
        }

        let n = grid.len() as i32;
        let m = grid[0].len() as i32;
        let mut dp = vec![vec![vec![-1; 16]; m as usize]; n as usize];
        f(0, 0, 0, &grid, k, &mut dp)
    }
}

#[test]
fn test() {
    let grid = vec![vec![2, 1, 5], vec![7, 10, 0], vec![12, 6, 4]];
    let k = 11;
    let res = 3;
    assert_eq!(Solution::count_paths_with_xor_value(grid, k), res);

    let grid = vec![vec![1, 3, 3, 3], vec![0, 3, 3, 2], vec![3, 0, 1, 1]];
    let k = 2;
    let res = 5;
    assert_eq!(Solution::count_paths_with_xor_value(grid, k), res);

    let grid = vec![vec![1, 1, 1, 2], vec![3, 0, 3, 2], vec![3, 0, 2, 2]];
    let k = 10;
    let res = 0;
    assert_eq!(Solution::count_paths_with_xor_value(grid, k), res);
}
