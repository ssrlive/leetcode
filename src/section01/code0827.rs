#![allow(dead_code)]

// 827. Making A Large Island
// https://leetcode.com/problems/making-a-large-island/
// https://leetcode.cn/problems/making-a-large-island/
//
// You are given an n x n binary matrix grid. You are allowed to change at most one 0 to be 1.
//
// Return the size of the largest island in grid after applying this operation.
//
// An island is a 4-directionally connected group of 1s.
//
// Example 1:
//
// Input: grid = [[1,0],[0,1]]
// Output: 3
// Explanation: Change one 0 to 1 and connect two 1s, then we get an island with area = 3.
//
// Example 2:
//
// Input: grid = [[1,1],[1,0]]
// Output: 4
// Explanation: Change the 0 to 1 and make the island bigger, only one island with area = 4.
//
// Example 3:
//
// Input: grid = [[1,1],[1,1]]
// Output: 4
// Explanation: Can't change any 0 to 1, only one island with area = 4.
//
// Constraints:
//
// - n == grid.length
// - n == grid[i].length
// - 1 <= n <= 500
// - grid[i][j] is either 0 or 1.
//

struct Solution;

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &[Vec<i32>], ids: &mut Vec<Vec<Option<usize>>>, p: (usize, usize), idx: usize) -> i32 {
            ids[p.0][p.1] = Some(idx);
            let mut ret = 1;
            for d in [0, 1, 0, !0, 0].windows(2) {
                let i = p.0.wrapping_add(d[0]);
                let j = p.1.wrapping_add(d[1]);
                if i < grid.len() && j < grid.len() && grid[i][j] == 1 && ids[i][j].is_none() {
                    ret += dfs(grid, ids, (i, j), idx);
                }
            }
            ret
        }

        let n = grid.len();
        let mut ids = vec![vec![None; n]; n];
        let mut sizes = Vec::new();
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 && ids[i][j].is_none() {
                    let size = dfs(&grid, &mut ids, (i, j), sizes.len());
                    sizes.push(size);
                }
            }
        }
        let mut answer = *sizes.iter().max().unwrap_or(&0);
        for (i, row) in grid.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                let mut v = Vec::new();
                if *col == 0 {
                    for d in [0, 1, 0, !0, 0].windows(2) {
                        let i = i.wrapping_add(d[0]);
                        let j = j.wrapping_add(d[1]);
                        if i < n
                            && j < n
                            && let Some(idx) = ids[i][j]
                        {
                            v.push(idx);
                        }
                    }
                }
                v.sort_unstable();
                v.dedup();
                answer = answer.max(1 + v.iter().map(|&k| sizes[k]).sum::<i32>());
            }
        }
        answer
    }
}

#[test]
fn test() {
    let grid = vec![vec![1, 0], vec![0, 1]];
    assert_eq!(Solution::largest_island(grid), 3);

    let grid = vec![vec![1, 1], vec![1, 0]];
    assert_eq!(Solution::largest_island(grid), 4);

    let grid = vec![vec![1, 1], vec![1, 1]];
    assert_eq!(Solution::largest_island(grid), 4);
}
