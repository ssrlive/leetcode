#![allow(dead_code)]

// 3462. Maximum Sum With at Most K Elements
// https://leetcode.com/problems/maximum-sum-with-at-most-k-elements/
// https://leetcode.cn/problems/maximum-sum-with-at-most-k-elements/
//
// Medium
//
// You are given a 2D integer matrix grid of size n x m, an integer array limits of length n,
// and an integer k. The task is to find the maximum sum of at most k elements from the matrix grid such that:
//
// The number of elements taken from the ith row of grid does not exceed limits[i].
//
// Return the maximum sum.
//
// Example 1:
//
// Input: grid = [[1,2],[3,4]], limits = [1,2], k = 2
//
// Output: 7
//
// Explanation:
//
// From the second row, we can take at most 2 elements. The elements taken are 4 and 3.
// The maximum possible sum of at most 2 selected elements is 4 + 3 = 7.
//
// Example 2:
//
// Input: grid = [[5,3,7],[8,2,6]], limits = [2,2], k = 3
//
// Output: 21
//
// Explanation:
//
// From the first row, we can take at most 2 elements. The element taken is 7.
// From the second row, we can take at most 2 elements. The elements taken are 8 and 6.
// The maximum possible sum of at most 3 selected elements is 7 + 8 + 6 = 21.
//
// Constraints:
//
// n == grid.length == limits.length
// m == grid[i].length
// 1 <= n, m <= 500
// 0 <= grid[i][j] <= 10^5
// 0 <= limits[i] <= m
// 0 <= k <= min(n * m, sum(limits))
//

struct Solution;

impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>, limits: Vec<i32>, k: i32) -> i64 {
        let mut arr: Vec<(i64, usize)> = Vec::new();
        for (i, grid_i) in grid.iter().enumerate() {
            for &grid_i_j in grid_i {
                arr.push((grid_i_j as i64, i));
            }
        }
        arr.sort_by(|a, b| b.0.cmp(&a.0));
        let mut count: Vec<i64> = vec![0; grid.len()];
        let mut sum: i64 = 0;
        let mut selected: i64 = 0;
        for it in arr.iter() {
            let value = it.0;
            let row = it.1;
            if count[row] < limits[row] as i64 && selected < k as i64 {
                sum += value;
                count[row] += 1;
                selected += 1;
            }
            if selected == k as i64 {
                break;
            }
        }
        sum
    }
}

#[test]
fn test() {
    let grid = vec![vec![1, 2], vec![3, 4]];
    let limits = vec![1, 2];
    let k = 2;
    let res = 7;
    assert_eq!(Solution::max_sum(grid, limits, k), res);

    let grid = vec![vec![5, 3, 7], vec![8, 2, 6]];
    let limits = vec![2, 2];
    let k = 3;
    let res = 21;
    assert_eq!(Solution::max_sum(grid, limits, k), res);
}
