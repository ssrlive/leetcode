#![allow(dead_code)]

// 2812. Find the Safest Path in a Grid
// https://leetcode.com/problems/find-the-safest-path-in-a-grid/
// https://leetcode.cn/problems/find-the-safest-path-in-a-grid/
//
// Medium
//
// You are given a 0-indexed 2D matrix grid of size n x n, where (r, c) represents:
//
// A cell containing a thief if grid[r][c] = 1
// An empty cell if grid[r][c] = 0
// You are initially positioned at cell (0, 0). In one move, you can move to any adjacent cell in the grid, including cells containing thieves.
//
// The safeness factor of a path on the grid is defined as the minimum manhattan distance from any cell in the path to any thief in the grid.
//
// Return the maximum safeness factor of all paths leading to cell (n - 1, n - 1).
//
// An adjacent cell of cell (r, c), is one of the cells (r, c + 1), (r, c - 1), (r + 1, c) and (r - 1, c) if it exists.
//
// The Manhattan distance between two cells (a, b) and (x, y) is equal to |a - x| + |b - y|, where |val| denotes the absolute value of val.
//
// Example 1:
//
// Input: grid = [[1,0,0],[0,0,0],[0,0,1]]
// Output: 0
// Explanation: All paths from (0, 0) to (n - 1, n - 1) go through the thieves in cells (0, 0) and (n - 1, n - 1).
//
// Example 2:
//
// Input: grid = [[0,0,1],[0,0,0],[0,0,0]]
// Output: 2
// Explanation: The path depicted in the picture above has a safeness factor of 2 since:
// - The closest cell of the path to the thief at cell (0, 2) is cell (0, 0). The distance between them is | 0 - 0 | + | 0 - 2 | = 2.
// It can be shown that there are no other paths with a higher safeness factor.
//
// Example 3:
//
// Input: grid = [[0,0,0,1],[0,0,0,0],[0,0,0,0],[1,0,0,0]]
// Output: 2
// Explanation: The path depicted in the picture above has a safeness factor of 2 since:
// - The closest cell of the path to the thief at cell (0, 3) is cell (1, 2). The distance between them is | 0 - 1 | + | 3 - 2 | = 2.
// - The closest cell of the path to the thief at cell (3, 0) is cell (3, 2). The distance between them is | 3 - 3 | + | 0 - 2 | = 2.
// It can be shown that there are no other paths with a higher safeness factor.
//
// Constraints:
//
// 1 <= grid.length == n <= 400
// grid[i].length == n
// grid[i][j] is either 0 or 1.
// There is at least one thief in the grid.
//

struct Solution;

impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        let (n, mut grid) = (grid.len(), grid);
        let mut q = std::collections::VecDeque::new();

        for (i, item) in grid.iter_mut().enumerate().take(n) {
            for (j, inner_item) in item.iter_mut().enumerate().take(n) {
                if *inner_item == 0 {
                    *inner_item = -1;
                } else {
                    *inner_item = 0;
                    q.push_back((0, i, j));
                }
            }
        }

        let dirs = [[1, 0], [0, 1], [-1, 0], [0, -1]];
        while let Some((val, i, j)) = q.pop_front() {
            for d in &dirs {
                let (u, v) = (i as i32 + d[0], j as i32 + d[1]);
                if u < 0 || u == n as i32 || v < 0 || v == n as i32 {
                    continue;
                }
                let (u, v) = (u as usize, v as usize);
                if grid[u][v] != -1 {
                    continue;
                }
                grid[u][v] = val + 1;
                q.push_back((val + 1, u, v));
            }
        }

        let mut pq = std::collections::BinaryHeap::new();
        let mut dist = vec![vec![-1; n]; n];

        dist[0][0] = grid[0][0];
        pq.push((grid[0][0], 0, 0));
        while let Some((val, i, j)) = pq.pop() {
            if i == n - 1 && j == n - 1 {
                return val;
            }
            for d in &dirs {
                let (u, v) = (i as i32 + d[0], j as i32 + d[1]);
                if u < 0 || u == n as i32 || v < 0 || v == n as i32 {
                    continue;
                }
                let (u, v) = (u as usize, v as usize);
                if dist[u][v] != -1 {
                    continue;
                }
                dist[u][v] = grid[u][v].min(dist[i][j]);
                pq.push((dist[u][v], u, v));
            }
        }

        0
    }
}

#[test]
fn test() {
    let grid = vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]];
    assert_eq!(Solution::maximum_safeness_factor(grid), 0);

    let grid = vec![vec![0, 0, 1], vec![0, 0, 0], vec![0, 0, 0]];
    assert_eq!(Solution::maximum_safeness_factor(grid), 2);

    let grid = vec![vec![0, 0, 0, 1], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![1, 0, 0, 0]];
    assert_eq!(Solution::maximum_safeness_factor(grid), 2);
}
