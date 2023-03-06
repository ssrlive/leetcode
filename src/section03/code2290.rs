#![allow(dead_code)]

/*

// 2290. Minimum Obstacle Removal to Reach Corner
// https://leetcode.com/problems/minimum-obstacle-removal-to-reach-corner/
// https://leetcode.cn/problems/minimum-obstacle-removal-to-reach-corner/
//
// Hard
//
// You are given a 0-indexed 2D integer array grid of size m x n. Each cell has one of two values:

    0 represents an empty cell,
    1 represents an obstacle that may be removed.

You can move up, down, left, or right from and to an empty cell.

Return the minimum number of obstacles to remove so you can move from the upper left corner (0, 0) to the lower right corner (m - 1, n - 1).

Example 1:

Input: grid = [[0,1,1],[1,1,0],[1,1,0]]
Output: 2
Explanation: We can remove the obstacles at (0, 1) and (0, 2) to create a path from (0, 0) to (2, 2).
It can be shown that we need to remove at least 2 obstacles, so we return 2.
Note that there may be other ways to remove 2 obstacles to create a path.

Example 2:

Input: grid = [[0,1,0,0,0],[0,1,0,1,0],[0,0,0,1,0]]
Output: 0
Explanation: We can move from (0, 0) to (2, 4) without removing any obstacles, so we return 0.

Constraints:

    m == grid.length
    n == grid[i].length
    1 <= m, n <= 10^5
    2 <= m * n <= 10^5
    grid[i][j] is either 0 or 1.
    grid[0][0] == grid[m - 1][n - 1] == 0
*/

struct Solution;

impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        const DIR: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
        let rows = grid.len();
        assert!(rows >= 1);
        let cols = grid[0].len();
        assert!(cols >= 1);
        let mut visited = vec![vec![i32::MAX; cols]; rows];
        visited[0][0] = grid[0][0];
        let mut pq = BinaryHeap::new();
        pq.push((Reverse(grid[0][0]), 0, 0));
        while let Some((Reverse(obs), r, c)) = pq.pop() {
            if r == rows - 1 && c == cols - 1 {
                return obs;
            }
            for (dr, dc) in DIR {
                let rx = r as isize + dr;
                let cx = c as isize + dc;
                if rx < 0 || cx < 0 {
                    continue;
                }
                let rx = rx as usize;
                let cx = cx as usize;
                if rx >= rows || cx >= cols {
                    continue;
                }
                let next_obs = obs + grid[rx][cx];
                if visited[rx][cx] <= next_obs {
                    continue;
                }
                visited[rx][cx] = next_obs;
                pq.push((Reverse(next_obs), rx, cx));
            }
        }
        unreachable!()
    }
}

#[test]
fn test() {
    let grid = vec![vec![0, 1, 1], vec![1, 1, 0], vec![1, 1, 0]];
    assert_eq!(Solution::minimum_obstacles(grid), 2);

    let grid = vec![vec![0, 1, 0, 0, 0], vec![0, 1, 0, 1, 0], vec![0, 0, 0, 1, 0]];
    assert_eq!(Solution::minimum_obstacles(grid), 0);
}
