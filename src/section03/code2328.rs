#![allow(dead_code)]

/*

// 2328. Number of Increasing Paths in a Grid
// https://leetcode.com/problems/number-of-increasing-paths-in-a-grid/
// https://leetcode.cn/problems/number-of-increasing-paths-in-a-grid/
//
// Hard
//
// You are given an m x n integer matrix grid, where you can move from a cell to any adjacent cell in all 4 directions.

Return the number of strictly increasing paths in the grid such that you can start from any cell and end at any cell. Since the answer may be very large, return it modulo 109 + 7.

Two paths are considered different if they do not have exactly the same sequence of visited cells.

Example 1:

Input: grid = [[1,1],[3,4]]
Output: 8
Explanation: The strictly increasing paths are:
- Paths with length 1: [1], [1], [3], [4].
- Paths with length 2: [1 -> 3], [1 -> 4], [3 -> 4].
- Paths with length 3: [1 -> 3 -> 4].
The total number of paths is 4 + 3 + 1 = 8.

Example 2:

Input: grid = [[1],[2]]
Output: 3
Explanation: The strictly increasing paths are:
- Paths with length 1: [1], [2].
- Paths with length 2: [1 -> 2].
The total number of paths is 2 + 1 = 3.

Constraints:

    m == grid.length
    n == grid[i].length
    1 <= m, n <= 1000
    1 <= m * n <= 10^5
    1 <= grid[i][j] <= 10^5
*/

struct Solution;

impl Solution {
    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        const DIRS: &[isize] = &[0, -1, 0, 1, 0];
        const MOD: i64 = 1e9 as i64 + 7;
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let mut memo: Vec<Vec<Option<i64>>> = vec![vec![None; len_cs]; len_rs];
        let mut cnt: i64 = 0;

        fn dfs(coord: (usize, usize), memo: &mut Vec<Vec<Option<i64>>>, grid: &Vec<Vec<i32>>) -> i64 {
            let len_rs: usize = grid.len();
            let len_cs: usize = grid[0].len();
            let mut cnt: i64 = 1;
            let (r, c) = coord;
            if let Some(m) = memo[r][c] {
                return m;
            }
            for d in 0..4 {
                let r_nxt: isize = r as isize + DIRS[d];
                let c_nxt: isize = c as isize + DIRS[d + 1];
                if r_nxt >= 0
                    && c_nxt >= 0
                    && (r_nxt as usize) < len_rs
                    && (c_nxt as usize) < len_cs
                    && grid[r_nxt as usize][c_nxt as usize] > grid[r][c]
                {
                    cnt = (cnt + dfs((r_nxt as usize, c_nxt as usize), memo, grid)) % MOD;
                }
            }
            memo[r][c] = Some(cnt);
            cnt
        }

        for r in 0..len_rs {
            for c in 0..len_cs {
                cnt = (cnt + dfs((r, c), &mut memo, &grid)) % MOD;
            }
        }
        cnt as i32
    }
}

#[test]
fn test() {
    let grid: Vec<Vec<i32>> = vec![vec![1, 1], vec![3, 4]];
    assert_eq!(Solution::count_paths(grid), 8);

    let grid: Vec<Vec<i32>> = vec![vec![1], vec![2]];
    assert_eq!(Solution::count_paths(grid), 3);
}
