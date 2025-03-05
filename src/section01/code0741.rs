#![allow(dead_code)]

// 741. Cherry Pickup
// https://leetcode.com/problems/cherry-pickup/
// https://leetcode.cn/problems/cherry-pickup/
//
// You are given an n x n grid representing a field of cherries, each cell is one of three possible integers.
//
// - 0 means the cell is empty, so you can pass through,
// - 1 means the cell contains a cherry that you can pick up and pass through, or
// - -1 means the cell contains a thorn that blocks your way.
//
// Return the maximum number of cherries you can collect by following the rules below:
//
// - Starting at the position (0, 0) and reaching (n - 1, n - 1) by moving right or down through valid path cells (cells with value 0 or 1).
// - After reaching (n - 1, n - 1), returning to (0, 0) by moving left or up through valid path cells.
// - When passing through a path cell containing a cherry, you pick it up, and the cell becomes an empty cell 0.
// - If there is no valid path between (0, 0) and (n - 1, n - 1), then no cherries can be collected.
//
// Example 1:
//
// Input: grid = [[0,1,-1],[1,0,-1],[1,1,1]]
// Output: 5
// Explanation: The player started at (0, 0) and went down, down, right right to reach (2, 2).
// 4 cherries were picked up during this single trip, and the matrix becomes [[0,1,-1],[0,0,-1],[0,0,0]].
// Then, the player went left, up, up, left to return home, picking up one more cherry.
// The total number of cherries picked up is 5, and this is the maximum possible.
//
// Example 2:
//
// Input: grid = [[1,1,-1],[1,-1,1],[-1,1,1]]
// Output: 0
//
// Constraints:
//
// - n == grid.length
// - n == grid[i].length
// - 1 <= n <= 50
// - grid[i][j] is -1, 0, or 1.
// - grid[0][0] != -1
// - grid[n - 1][n - 1] != -1
//

struct Solution;
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dp = vec![vec![vec![0; n]; n]; n];
        let answer = Self::solve(&grid, 0, 0, 0, 0, n, &mut dp).unwrap_or_default();
        if answer <= 0 { 0 } else { answer }
    }
    fn solve(grid: &Vec<Vec<i32>>, r1: usize, r2: usize, c1: usize, c2: usize, n: usize, dp: &mut Vec<Vec<Vec<i32>>>) -> Option<i32> {
        if r1 == n || r2 == n || c1 == n || c2 == n || grid[r1][c1] == -1 || grid[r2][c2] == -1 {
            return Some(i32::MIN);
        }
        if r1 == n - 1 && c1 == n - 1 {
            return Some(*grid.get(n - 1)?.get(n - 1)?);
        }
        let target = dp.get(r1)?.get(r2)?.get(c1)?;
        if *target != 0 {
            return Some(*target);
        }
        let mut answer = grid[r1][c1] + grid[r2][c2];
        if r1 == r2 && c1 == c2 {
            answer -= grid[r1][c1];
        }
        let l1 = Self::solve(grid, r1 + 1, r2 + 1, c1, c2, n, dp)?;
        let l2 = Self::solve(grid, r1 + 1, r2, c1, c2 + 1, n, dp)?;
        let l3 = Self::solve(grid, r1, r2 + 1, c1 + 1, c2, n, dp)?;
        let l4 = Self::solve(grid, r1, r2, c1 + 1, c2 + 1, n, dp)?;
        let temp = l1.max(l2).max(l3).max(l4);
        let target = dp.get_mut(r1)?.get_mut(r2)?.get_mut(c1)?;
        if temp == i32::MIN {
            *target = temp;
        } else {
            *target = temp + answer;
        }
        Some(*target)
    }
}

#[test]
fn test() {
    let grid = vec![vec![0, 1, -1], vec![1, 0, -1], vec![1, 1, 1]];
    assert_eq!(Solution::cherry_pickup(grid), 5);
    let grid = vec![vec![1, 1, -1], vec![1, -1, 1], vec![-1, 1, 1]];
    assert_eq!(Solution::cherry_pickup(grid), 0);
}
