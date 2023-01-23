#![allow(dead_code)]

// 1293. Shortest Path in a Grid with Obstacles Elimination
// https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/
// https://leetcode.cn/problems/shortest-path-in-a-grid-with-obstacles-elimination/
//
// Hard
//
// You are given an m x n integer matrix grid where each cell is either 0 (empty) or 1 (obstacle).
// You can move up, down, left, or right from and to an empty cell in one step.
//
// Return the minimum number of steps to walk from the upper left corner (0, 0) to the lower right corner
// (m - 1, n - 1) given that you can eliminate at most k obstacles.
// If it is not possible to find such walk return -1.
//
// Example 1:
//
// Input: grid = [[0,0,0],[1,1,0],[0,0,0],[0,1,1],[0,0,0]], k = 1
// Output: 6
// Explanation:
// The shortest path without eliminating any obstacle is 10.
// The shortest path with one obstacle elimination at position (3,2) is 6. Such path is (0,0) -> (0,1) -> (0,2) -> (1,2) -> (2,2) -> (3,2) -> (4,2).
//
// Example 2:
//
// Input: grid = [[0,1,1],[1,1,1],[1,0,0]], k = 1
// Output: -1
// Explanation: We need to eliminate at least two obstacles to find such a walk.
//
// Constraints:
//
// -    m == grid.length
// -    n == grid[i].length
// -    1 <= m, n <= 40
// -    1 <= k <= m * n
// -    grid[i][j] is either 0 or 1.
// -    grid[0][0] == grid[m - 1][n - 1] == 0
//

struct Solution;

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::collections::VecDeque;
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut visited = vec![vec![None; cols]; rows];
        visited[0][0] = Some(k);
        let mut vd = VecDeque::new();
        vd.push_back(((0, 0), 0, k));
        while let Some(((i, j), steps, k)) = vd.pop_front() {
            if i == rows - 1 && j == cols - 1 {
                return steps;
            }
            for d in [0, 1, 0, !0, 0].windows(2) {
                let i = i.wrapping_add(d[0]);
                let j = j.wrapping_add(d[1]);
                if (0..rows).contains(&i) && (0..cols).contains(&j) && k - grid[i][j] >= 0 {
                    if let Some(v) = visited[i][j] {
                        if k - grid[i][j] <= v {
                            continue;
                        }
                    }
                    visited[i][j] = Some(k - grid[i][j]);
                    vd.push_back(((i, j), steps + 1, k - grid[i][j]));
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![
                vec![0, 0, 0],
                vec![1, 1, 0],
                vec![0, 0, 0],
                vec![0, 1, 1],
                vec![0, 0, 0],
            ],
            1,
            6,
        ),
        (vec![vec![0, 1, 1], vec![1, 1, 1], vec![1, 0, 0]], 1, -1),
    ];
    for (grid, k, expect) in cases {
        assert_eq!(Solution::shortest_path(grid, k), expect);
    }
}
