#![allow(dead_code)]

// 1091. Shortest Path in Binary Matrix
// https://leetcode.com/problems/shortest-path-in-binary-matrix/
// https://leetcode.cn/problems/shortest-path-in-binary-matrix/
//
// Given an n x n binary matrix grid, return the length of the shortest clear path in the matrix. If there is no clear path, return -1.
//
// A clear path in a binary matrix is a path from the top-left cell (i.e., (0, 0)) to the bottom-right cell (i.e., (n - 1, n - 1)) such that:
//
// All the visited cells of the path are 0.
// All the adjacent cells of the path are 8-directionally connected (i.e., they are different and they share an edge or a corner).
// The length of a clear path is the number of visited cells of this path.
//
// Example 1:
//
// Input: grid = [[0,1],[1,0]]
// Output: 2
//
// Example 2:
//
// Input: grid = [[0,0,0],[1,1,0],[1,1,0]]
// Output: 4
//
// Example 3:
//
// Input: grid = [[1,0,0],[1,1,0],[1,1,0]]
// Output: -1
//
// Constraints:
//
// - n == grid.length
// - n == grid[i].length
// - 1 <= n <= 100
// - grid[i][j] is 0 or 1
//

struct Solution;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        let (r, c) = (grid.len(), grid[0].len());
        let mut answer = vec![vec![None; grid[0].len()]; grid.len()];
        let mut vd = VecDeque::new();
        if grid[0][0] == 0 {
            vd.push_back(((0, 0), 1));
            answer[0][0] = Some(1);
        }
        while let Some((p, len)) = vd.pop_front() {
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    if (0..r as i32).contains(&(p.0 + i)) && (0..c as i32).contains(&(p.1 + j)) {
                        let q = ((p.0 + i) as usize, (p.1 + j) as usize);
                        if grid[q.0][q.1] == 0 && answer[q.0][q.1].is_none() {
                            answer[q.0][q.1] = Some(len + 1);
                            vd.push_back(((p.0 + i, p.1 + j), len + 1));
                        }
                    }
                }
            }
        }
        answer[r - 1][c - 1].unwrap_or(-1)
    }
}

#[test]
fn test() {
    let grid = vec![vec![0, 1], vec![1, 0]];
    assert_eq!(Solution::shortest_path_binary_matrix(grid), 2);

    let grid = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
    assert_eq!(Solution::shortest_path_binary_matrix(grid), 4);

    let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
    assert_eq!(Solution::shortest_path_binary_matrix(grid), -1);
}
