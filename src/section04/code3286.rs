#![allow(dead_code)]

// 3286. Find a Safe Walk Through a Grid
// https://leetcode.com/problems/find-a-safe-walk-through-a-grid/
// https://leetcode.cn/problems/find-a-safe-walk-through-a-grid/
//
// Medium
//
// You are given an m x n binary matrix grid and an integer health.
//
// You start on the upper-left corner (0, 0) and would like to get to the lower-right corner (m - 1, n - 1).
//
// You can move up, down, left, or right from one cell to another adjacent cell as long as your health remains positive.
//
// Cells (i, j) with grid[i][j] = 1 are considered unsafe and reduce your health by 1.
//
// Return true if you can reach the final cell with a health value of 1 or more, and false otherwise.
//
// Example 1:
//
// Input: grid = [[0,1,0,0,0],[0,1,0,1,0],[0,0,0,1,0]], health = 1
//
// Output: true
//
// Explanation:
//
// The final cell can be reached safely by walking along the gray cells below.
//
// Example 2:
//
// Input: grid = [[0,1,1,0,0,0],[1,0,1,0,0,0],[0,1,1,1,0,1],[0,0,1,0,1,0]], health = 3
//
// Output: false
//
// Explanation:
//
// A minimum of 4 health points is needed to reach the final cell safely.
//
// Example 3:
//
// Input: grid = [[1,1,1],[1,0,1],[1,1,1]], health = 5
//
// Output: true
//
// Explanation:
//
// The final cell can be reached safely by walking along the gray cells below.
//
// Any path that does not go through the cell (1, 1) is unsafe since your health will drop to 0 when reaching the final cell.
//
// Constraints:
//
//     m == grid.length
//     n == grid[i].length
//     1 <= m, n <= 50
//     2 <= m * n
//     1 <= health <= m + n
//     grid[i][j] is either 0 or 1.
//

struct Solution;

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        let mut cnt = vec![vec![0; n as usize]; m as usize];
        let mut pq = std::collections::BinaryHeap::new();
        pq.push((health - grid[0][0], 0, 0));
        while let Some((h, i, j)) = pq.pop() {
            if i == m - 1 && j == n - 1 {
                return true;
            }
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let x = i + dx;
                let y = j + dy;
                if x >= 0 && y >= 0 && x < m && y < n && cnt[x as usize][y as usize] < h - grid[x as usize][y as usize] {
                    cnt[x as usize][y as usize] = h - grid[x as usize][y as usize];
                    pq.push((h - grid[x as usize][y as usize], x, y));
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    let grid = vec![vec![0, 1, 0, 0, 0], vec![0, 1, 0, 1, 0], vec![0, 0, 0, 1, 0]];
    let health = 1;
    let output = true;
    assert_eq!(Solution::find_safe_walk(grid, health), output);

    let grid = vec![
        vec![0, 1, 1, 0, 0, 0],
        vec![1, 0, 1, 0, 0, 0],
        vec![0, 1, 1, 1, 0, 1],
        vec![0, 0, 1, 0, 1, 0],
    ];
    let health = 3;
    let output = false;
    assert_eq!(Solution::find_safe_walk(grid, health), output);

    let grid = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    let health = 5;
    let output = true;
    assert_eq!(Solution::find_safe_walk(grid, health), output);
}
