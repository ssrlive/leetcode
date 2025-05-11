#![allow(dead_code)]

// 3546. Equal Sum Grid Partition I
// https://leetcode.com/problems/equal-sum-grid-partition-i/
// https://leetcode.cn/problems/equal-sum-grid-partition-i/
//
// Medium
//
// You are given an m x n matrix grid of positive integers. Your task is to determine if it is possible to make either
// one horizontal or one vertical cut on the grid such that:
//
//     Each of the two resulting sections formed by the cut is non-empty.
//     The sum of the elements in both sections is equal.
//
// Return true if such a partition exists; otherwise return false.
//
// Example 1:
//
// Input: grid = [[1,4],[2,3]]
//
// Output: true
//
// Explanation:
//
// A horizontal cut between row 0 and row 1 results in two non-empty sections, each with a sum of 5. Thus, the answer is true.
//
// Example 2:
//
// Input: grid = [[1,3],[2,4]]
//
// Output: false
//
// Explanation:
//
// No horizontal or vertical cut results in two non-empty sections with equal sums. Thus, the answer is false.
//
// Constraints:
//
//     1 <= m == grid.length <= 10^5
//     1 <= n == grid[i].length <= 10^5
//     2 <= m * n <= 10^5
//     1 <= grid[i][j] <= 10^5
//

struct Solution;

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let grid = grid
            .iter()
            .map(|row| row.iter().map(|&x| x as i64).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        if grid.len() == 1 {
            let sum: i64 = grid[0].iter().sum();
            let mut temp_sum = 0;
            for &num in &grid[0] {
                temp_sum += num;
                if temp_sum as f64 == sum as f64 / 2.0 {
                    return true;
                }
            }
            return false;
        }
        let n = grid.len();
        let m = grid[0].len();
        let total_sum: i64 = grid.iter().flat_map(|row| row.iter()).sum();
        // for horizontal lines
        let mut hori_sum = 0;
        for grid_i in grid.iter().take(n) {
            for &grid_i_j in grid_i.iter().take(m) {
                hori_sum += grid_i_j;
            }
            if hori_sum as f64 == total_sum as f64 / 2.0 {
                return true;
            }
        }
        // for vertical lines
        let mut verti_sum = 0;
        for i in 0..m {
            for grid_j in grid.iter().take(n) {
                verti_sum += grid_j[i];
            }
            if verti_sum as f64 == total_sum as f64 / 2.0 {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    assert!(Solution::can_partition_grid(vec![vec![1, 4], vec![2, 3]]));
    assert!(!Solution::can_partition_grid(vec![vec![1, 3], vec![2, 4]]));
}
