#![allow(dead_code)]

// 934. Shortest Bridge
// https://leetcode.com/problems/shortest-bridge/
// https://leetcode.cn/problems/shortest-bridge/
//
// You are given an n x n binary matrix grid where 1 represents land and 0 represents water.
//
// An island is a 4-directionally connected group of 1's not connected to any other 1's. There are exactly two islands in grid.
//
// You may change 0's to 1's to connect the two islands to form one island.
//
// Return the smallest number of 0's you must flip to connect the two islands.
//
// Example 1:
//
// Input: grid = [[0,1],[1,0]]
// Output: 1
//
// Example 2:
//
// Input: grid = [[0,1,0],[0,0,0],[0,0,1]]
// Output: 2
//
// Example 3:
//
// Input: grid = [[1,1,1,1,1],[1,0,0,0,1],[1,0,1,0,1],[1,0,0,0,1],[1,1,1,1,1]]
// Output: 1
//
// Constraints:
//
// - n == grid.length == grid[i].length
// - 2 <= n <= 100
// - grid[i][j] is either 0 or 1.
// - There are exactly two islands in grid.
//

struct Solution;

impl Solution {
    pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        'outer: for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    Self::mark(&mut grid, i as i32, j as i32);
                    break 'outer;
                }
            }
        }

        // bfs from marked island until find the other land
        let mut queue = std::collections::VecDeque::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 2 {
                    queue.push_back((i, j, 0_i32));
                }
            }
        }

        while !queue.is_empty() {
            let (x, y, level) = queue.pop_front().unwrap();
            if grid[x][y] == 1 {
                return level - 1;
            }

            if x > 0 && grid[x - 1][y] != 2 {
                grid[x - 1][y] = 2 - grid[x - 1][y];
                queue.push_back((x - 1, y, level + 1));
            }

            if x + 1 < grid.len() && grid[x + 1][y] != 2 {
                grid[x + 1][y] = 2 - grid[x + 1][y];
                queue.push_back((x + 1, y, level + 1));
            }

            if y > 0 && grid[x][y - 1] != 2 {
                grid[x][y - 1] = 2 - grid[x][y - 1];
                queue.push_back((x, y - 1, level + 1));
            }

            if y + 1 < grid[0].len() && grid[x][y + 1] != 2 {
                grid[x][y + 1] = 2 - grid[x][y + 1];
                queue.push_back((x, y + 1, level + 1));
            }
        }
        unreachable!()
    }

    fn mark(grid: &mut Vec<Vec<i32>>, x: i32, y: i32) {
        if !(x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32) {
            return;
        }

        let (x, y) = (x as usize, y as usize);
        if grid[x][y] != 1 {
            return;
        }
        grid[x][y] = 2;

        Self::mark(grid, x as i32 - 1, y as i32);
        Self::mark(grid, x as i32 + 1, y as i32);
        Self::mark(grid, x as i32, y as i32 - 1);
        Self::mark(grid, x as i32, y as i32 + 1);
    }
}

#[test]
fn test() {
    let grid = vec![vec![0, 1], vec![1, 0]];
    assert_eq!(Solution::shortest_bridge(grid), 1);

    let grid = vec![vec![0, 1, 0], vec![0, 0, 0], vec![0, 0, 1]];
    assert_eq!(Solution::shortest_bridge(grid), 2);

    let grid = vec![
        vec![1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 1],
        vec![1, 0, 1, 0, 1],
        vec![1, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1],
    ];
    assert_eq!(Solution::shortest_bridge(grid), 1);
}
