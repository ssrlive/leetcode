#![allow(dead_code)]

// 778. Swim in Rising Water
// https://leetcode.com/problems/swim-in-rising-water/
// https://leetcode.cn/problems/swim-in-rising-water/
//
// You are given an n x n integer matrix grid where each value grid[i][j] represents the elevation at that point (i, j).
//
// The rain starts to fall. At time t, the depth of the water everywhere is t. You can swim from a square to another 4-directionally adjacent square
// if and only if the elevation of both squares individually are at most t. You can swim infinite distances in zero time. Of course,
// you must stay within the boundaries of the grid during your swim.
//
// Return the least time until you can reach the bottom right square (n - 1, n - 1) if you start at the top left square (0, 0).
//
// Example 1:
//
// Input: grid = [[0,2],[1,3]]
// Output: 3
// Explanation:
// At time 0, you are in grid location (0, 0).
// You cannot go anywhere else because 4-directionally adjacent neighbors have a higher elevation than t = 0.
// You cannot reach point (1, 1) until time 3.
// When the depth of water is 3, we can swim anywhere inside the grid.
//
// Example 2:
//
// Input: grid = [[0,1,2,3,4],[24,23,22,21,5],[12,13,14,15,16],[11,17,18,19,20],[10,9,8,7,6]]
// Output: 16
// Explanation: The final route is shown.
// We need to wait until time 16 so that (0, 0) and (4, 4) are connected.
//
// Constraints:
//
// - n == grid.length
// - n == grid[i].length
// - 1 <= n <= 50
// - 0 <= grid[i][j] < n^2
// - Each value grid[i][j] is unique.
//

struct Solution;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut visited = vec![vec![false; n]; n];
        let mut queue = std::collections::BinaryHeap::new();
        queue.push(std::cmp::Reverse((grid[0][0], 0, 0)));
        let mut max = 0;
        while let Some(std::cmp::Reverse((t, i, j))) = queue.pop() {
            if i == n - 1 && j == n - 1 {
                return std::cmp::max(max, t);
            }
            if visited[i][j] {
                continue;
            }
            visited[i][j] = true;
            max = std::cmp::max(max, t);
            if i > 0 {
                queue.push(std::cmp::Reverse((grid[i - 1][j], i - 1, j)));
            }
            if i < n - 1 {
                queue.push(std::cmp::Reverse((grid[i + 1][j], i + 1, j)));
            }
            if j > 0 {
                queue.push(std::cmp::Reverse((grid[i][j - 1], i, j - 1)));
            }
            if j < n - 1 {
                queue.push(std::cmp::Reverse((grid[i][j + 1], i, j + 1)));
            }
        }
        unreachable!()
    }

    pub fn swim_in_water2(grid: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut ret = grid[0][0];
        let n = grid.len();
        let mut pq = BinaryHeap::from([Reverse((grid[0][0], 0, 0))]);
        let dirs = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        let mut flag = vec![vec![0; n]; n];

        flag[0][0] = 1;
        while let Some(Reverse((h, u, v))) = pq.pop() {
            ret = h;
            if u == n - 1 && v == n - 1 {
                break;
            }
            for d in dirs {
                let (x, y) = (u as i32 + d[0], v as i32 + d[1]);
                if x < 0 || x == n as i32 || y < 0 || y == n as i32 {
                    continue;
                }

                let (x, y) = (x as usize, y as usize);
                if flag[x][y] == 1 {
                    continue;
                }
                flag[x][y] = 1;

                pq.push(Reverse((h.max(grid[x][y]), x, y)));
            }
        }

        ret
    }
}

#[test]
fn test() {
    assert_eq!(Solution::swim_in_water2(vec![vec![0, 2], vec![1, 3]]), 3);
    let grid = vec![
        vec![0, 1, 2, 3, 4],
        vec![24, 23, 22, 21, 5],
        vec![12, 13, 14, 15, 16],
        vec![11, 17, 18, 19, 20],
        vec![10, 9, 8, 7, 6],
    ];
    assert_eq!(Solution::swim_in_water2(grid), 16);
}
