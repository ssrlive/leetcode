#![allow(dead_code)]

// 3651. Minimum Cost Path with Teleportations
// https://leetcode.com/problems/minimum-cost-path-with-teleportations/
// https://leetcode.cn/problems/minimum-cost-path-with-teleportations/
//
// Hard
//
// You are given a m x n 2D integer array grid and an integer k. You start at the top-left cell (0, 0) and your goal is to reach the bottom‐right cell (m - 1, n - 1).
//
// There are two types of moves available:
//
// Normal move: You can move right or down from your current cell (i, j), i.e. you can move to (i, j + 1) (right) or (i + 1, j) (down). The cost is the value of the destination cell.
//
// Teleportation: You can teleport from any cell (i, j), to any cell (x, y) such that grid[x][y] <= grid[i][j]; the cost of this move is 0. You may teleport at most k times.
//
// Return the minimum total cost to reach cell (m - 1, n - 1) from (0, 0).
//
// Example 1:
//
// Input: grid = [[1,3,3],[2,5,4],[4,3,5]], k = 2
//
// Output: 7
//
// Explanation:
//
// Initially we are at (0, 0) and cost is 0.
//
// Current Position	Move	New Position	Total Cost
// (0, 0)	Move Down	(1, 0)	0 + 2 = 2
// (1, 0)	Move Right	(1, 1)	2 + 5 = 7
// (1, 1)	Teleport to (2, 2)	(2, 2)	7 + 0 = 7
// The minimum cost to reach bottom-right cell is 7.
//
// Example 2:
//
// Input: grid = [[1,2],[2,3],[3,4]], k = 1
//
// Output: 9
//
// Explanation:
//
// Initially we are at (0, 0) and cost is 0.
//
// Current Position	Move	New Position	Total Cost
// (0, 0)	Move Down	(1, 0)	0 + 2 = 2
// (1, 0)	Move Right	(1, 1)	2 + 3 = 5
// (1, 1)	Move Down	(2, 1)	5 + 4 = 9
// The minimum cost to reach bottom-right cell is 9.
//
// Constraints:
//
// 2 <= m, n <= 80
// m == grid.length
// n == grid[i].length
// 0 <= grid[i][j] <= 10^4
// 0 <= k <= 10
//

struct Solution;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        fn relax(dp: &mut [Vec<i32>], grid: &[Vec<i32>]) {
            let m = grid.len();
            let n = grid[0].len();
            dp[0][0] = 0;
            for i in 0..m {
                for j in 0..n {
                    if i > 0 {
                        dp[i][j] = dp[i][j].min(dp[i - 1][j] + grid[i][j]);
                    }
                    if j > 0 {
                        dp[i][j] = dp[i][j].min(dp[i][j - 1] + grid[i][j]);
                    }
                }
            }
        }

        let m = grid.len();
        let n = grid[0].len();
        let inf = 1_000_000_000;
        let mut values = Vec::new();
        for row in &grid {
            for &x in row {
                values.push(x);
            }
        }
        values.sort_unstable();
        values.dedup();
        use std::collections::HashMap;
        let mut idx = HashMap::new();
        for (i, &v) in values.iter().enumerate() {
            idx.insert(v, i);
        }
        let sz = values.len();
        let mut prev = vec![vec![inf; n]; m];
        relax(&mut prev, &grid);
        let mut ans = prev[m - 1][n - 1];
        for _t in 1..=k {
            let mut best = vec![inf; sz];
            let mut curr = vec![vec![inf; n]; m];
            for i in 0..m {
                for j in 0..n {
                    let v = grid[i][j];
                    let index = idx[&v];
                    best[index] = best[index].min(prev[i][j]);
                }
            }
            for i in (0..sz - 1).rev() {
                best[i] = best[i].min(best[i + 1]);
            }
            for i in 0..m {
                for j in 0..n {
                    let v = grid[i][j];
                    let index = idx[&v];
                    curr[i][j] = curr[i][j].min(best[index]);
                }
            }
            relax(&mut curr, &grid);
            prev = curr;
            ans = prev[m - 1][n - 1];
        }
        ans
    }
}

#[test]
fn test() {
    let grid1 = vec![vec![1, 3, 3], vec![2, 5, 4], vec![4, 3, 5]];
    let k1 = 2;
    assert_eq!(Solution::min_cost(grid1, k1), 7);

    let grid2 = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
    let k2 = 1;
    assert_eq!(Solution::min_cost(grid2, k2), 9);
}
