#![allow(dead_code)]

// 3122. Minimum Number of Operations to Satisfy Conditions
// https://leetcode.com/problems/minimum-number-of-operations-to-satisfy-conditions/
// https://leetcode.cn/problems/minimum-number-of-operations-to-satisfy-conditions/
//
// Medium
//
// You are given a 2D matrix grid of size m x n. In one operation, you can change the value of any cell
// to any non-negative number. You need to perform some operations such that each cell grid[i][j] is:
//
// Equal to the cell below it, i.e. grid[i][j] == grid[i + 1][j] (if it exists).
// Different from the cell to its right, i.e. grid[i][j] != grid[i][j + 1] (if it exists).
// Return the minimum number of operations needed.
//
// Example 1:
//
// Input: grid = [[1,0,2],[1,0,2]]
//
// Output: 0
//
// Explanation:
//
// All the cells in the matrix already satisfy the properties.
//
// Example 2:
//
// Input: grid = [[1,1,1],[0,0,0]]
//
// Output: 3
//
// Explanation:
//
// The matrix becomes [[1,0,1],[1,0,1]] which satisfies the properties, by doing these 3 operations:
//
// Change grid[1][0] to 1.
// Change grid[0][1] to 0.
// Change grid[1][2] to 1.
//
// Example 3:
//
// Input: grid = [[1],[2],[3]]
//
// Output: 2
//
// Explanation:
//
// There is a single column. We can change the value to 1 in each cell using 2 operations.
//
// Constraints:
//
// 1 <= n, m <= 1000
// 0 <= grid[i][j] <= 9
//

struct Solution;

impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let (yy, xx) = (grid.len(), grid[0].len());

        let mut dp_prev = [yy; 10];
        for y in 0..yy {
            dp_prev[grid[y][0] as usize] -= 1;
        }

        for x in 1..xx {
            let mut dp = [yy; 10];
            for y in 0..yy {
                dp[grid[y][x] as usize] -= 1
            }

            for (i, dp_i) in dp.iter_mut().enumerate() {
                let mut min_val = usize::MAX;
                for (j, &dp_prev_j) in dp_prev.iter().enumerate() {
                    if i != j {
                        min_val = min_val.min(dp_prev_j);
                    }
                }
                *dp_i += min_val;
            }

            dp_prev = dp;
        }

        *dp_prev.iter().min().unwrap() as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_operations(vec![vec![1, 0, 2], vec![1, 0, 2]]), 0);
    assert_eq!(Solution::minimum_operations(vec![vec![1, 1, 1], vec![0, 0, 0]]), 3);
    assert_eq!(Solution::minimum_operations(vec![vec![1], vec![2], vec![3]]), 2);
}
