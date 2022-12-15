#![allow(dead_code)]

// 764. Largest Plus Sign
// https://leetcode.com/problems/largest-plus-sign/
//
// You are given an integer n. You have an n x n binary grid grid with all values initially 1's except for some indices given in the array mines.
// The i-th element of the array mines is defined as mines[i] = [xi, yi] where grid[xi][yi] == 0.
//
// Return the order of the largest axis-aligned plus sign of 1's contained in grid. If there is none, return 0.
//
// An axis-aligned plus sign of 1's of order k has some center grid[r][c] == 1 along with four arms of length k - 1 going up, down, left, and right,
// and made of 1's. Note that there could be 0's or 1's beyond the arms of the plus sign, only the relevant area of the plus sign is checked for 1's.
//
// Example 1:
//
// Input: n = 5, mines = [[4,2]]
// Output: 2
// Explanation: In the above grid, the largest plus sign can only be of order 2. One of them is shown.
//
// Example 2:
//
// Input: n = 1, mines = [[0,0]]
// Output: 0
// Explanation: There is no plus sign, so return 0.
//
// Constraints:
//
// - 1 <= n <= 500
// - 1 <= mines.length <= 5000
// - 0 <= xi, yi < n
// - All the pairs (xi, yi) are unique.
//

struct Solution;

impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut grid = vec![vec![1; n]; n];
        for mine in mines {
            grid[mine[0] as usize][mine[1] as usize] = 0;
        }

        let mut left = vec![vec![0; n]; n];
        let mut right = vec![vec![0; n]; n];
        let mut up = vec![vec![0; n]; n];
        let mut down = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    left[i][j] = if j == 0 { 1 } else { left[i][j - 1] + 1 };
                    up[i][j] = if i == 0 { 1 } else { up[i - 1][j] + 1 };
                }
            }
        }

        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if grid[i][j] == 1 {
                    right[i][j] = if j == n - 1 { 1 } else { right[i][j + 1] + 1 };
                    down[i][j] = if i == n - 1 { 1 } else { down[i + 1][j] + 1 };
                }
            }
        }

        let mut ans = 0;
        for i in 0..n {
            for j in 0..n {
                let len = std::cmp::min(
                    left[i][j],
                    std::cmp::min(right[i][j], std::cmp::min(up[i][j], down[i][j])),
                );
                ans = std::cmp::max(ans, len);
            }
        }

        ans as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::order_of_largest_plus_sign(5, vec![vec![4, 2]]), 2);
    assert_eq!(Solution::order_of_largest_plus_sign(1, vec![vec![0, 0]]), 0);
}
