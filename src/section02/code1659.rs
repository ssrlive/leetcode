#![allow(dead_code)]

/*

// 1659. Maximize Grid Happiness
// https://leetcode.com/problems/maximize-grid-happiness/
// https://leetcode.cn/problems/maximize-grid-happiness/
//
// Hard
//
// You are given four integers, m, n, introvertsCount, and extrovertsCount. You have an m x n grid, and there are two types of people: introverts and extroverts. There are introvertsCount introverts and extrovertsCount extroverts.

You should decide how many people you want to live in the grid and assign each of them one grid cell. Note that you do not have to have all the people living in the grid.

The happiness of each person is calculated as follows:

    Introverts start with 120 happiness and lose 30 happiness for each neighbor (introvert or extrovert).
    Extroverts start with 40 happiness and gain 20 happiness for each neighbor (introvert or extrovert).

Neighbors live in the directly adjacent cells north, east, south, and west of a person's cell.

The grid happiness is the sum of each person's happiness. Return the maximum possible grid happiness.

Example 1:

Input: m = 2, n = 3, introvertsCount = 1, extrovertsCount = 2
Output: 240
Explanation: Assume the grid is 1-indexed with coordinates (row, column).
We can put the introvert in cell (1,1) and put the extroverts in cells (1,3) and (2,3).
- Introvert at (1,1) happiness: 120 (starting happiness) - (0 * 30) (0 neighbors) = 120
- Extrovert at (1,3) happiness: 40 (starting happiness) + (1 * 20) (1 neighbor) = 60
- Extrovert at (2,3) happiness: 40 (starting happiness) + (1 * 20) (1 neighbor) = 60
The grid happiness is 120 + 60 + 60 = 240.
The above figure shows the grid in this example with each person's happiness. The introvert stays in the light green cell while the extroverts live on the light purple cells.

Example 2:

Input: m = 3, n = 1, introvertsCount = 2, extrovertsCount = 1
Output: 260
Explanation: Place the two introverts in (1,1) and (3,1) and the extrovert at (2,1).
- Introvert at (1,1) happiness: 120 (starting happiness) - (1 * 30) (1 neighbor) = 90
- Extrovert at (2,1) happiness: 40 (starting happiness) + (2 * 20) (2 neighbors) = 80
- Introvert at (3,1) happiness: 120 (starting happiness) - (1 * 30) (1 neighbor) = 90
The grid happiness is 90 + 80 + 90 = 260.

Example 3:

Input: m = 2, n = 2, introvertsCount = 4, extrovertsCount = 0
Output: 240

Constraints:

    1 <= m, n <= 5
    0 <= introvertsCount, extrovertsCount <= min(m * n, 6)
*/

struct Solution;

impl Solution {
    pub fn get_max_grid_happiness(m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
        let mut dp = vec![vec![vec![vec![vec![0; 64]; 64]; 7]; 7]; 25];
        Self::dfs(m, n, 0, introverts_count, extroverts_count, 0, 0, &mut dp)
    }

    fn n_cost(_m: i32, n: i32, i: i32, j: i32, mask_in: i32, mask_ex: i32, d: i32) -> i32 {
        let mut diff = 0;
        let up = 1 << (n - 1);
        if j > 0 && (mask_in & 1) > 0 {
            diff += d - 30;
        }
        if i > 0 && (mask_in & up) > 0 {
            diff += d - 30;
        }
        if j > 0 && (mask_ex & 1) > 0 {
            diff += d + 20;
        }
        if i > 0 && (mask_ex & up) > 0 {
            diff += d + 20;
        }
        diff
    }

    #[allow(clippy::too_many_arguments)]
    fn dfs(m: i32, n: i32, p: i32, in_: i32, ex: i32, mask_in: i32, mask_ex: i32, dp: &mut Vec<Vec<Vec<Vec<Vec<i32>>>>>) -> i32 {
        let i = p / n;
        let j = p % n;
        if i >= m {
            return 0;
        }
        if dp[p as usize][in_ as usize][ex as usize][mask_in as usize][mask_ex as usize] > 0 {
            return dp[p as usize][in_ as usize][ex as usize][mask_in as usize][mask_ex as usize] - 1;
        }
        let n_mask_in = (mask_in << 1) & 63;
        let n_mask_ex = (mask_ex << 1) & 63;
        let mut res = Self::dfs(m, n, p + 1, in_, ex, n_mask_in, n_mask_ex, dp);
        if in_ > 0 {
            let diff = 120 + Self::n_cost(m, n, i, j, mask_in, mask_ex, -30);
            res = res.max(diff + Self::dfs(m, n, p + 1, in_ - 1, ex, n_mask_in + 1, n_mask_ex, dp));
        }
        if ex > 0 {
            let diff = 40 + Self::n_cost(m, n, i, j, mask_in, mask_ex, 20);
            res = res.max(diff + Self::dfs(m, n, p + 1, in_, ex - 1, n_mask_in, n_mask_ex + 1, dp));
        }
        dp[p as usize][in_ as usize][ex as usize][mask_in as usize][mask_ex as usize] = res + 1;
        res
    }
}

#[test]
fn test() {
    let cases = vec![(2, 3, 1, 2, 240), (3, 1, 2, 1, 260), (2, 2, 4, 0, 240)];
    for (m, n, i, ex, expected) in cases {
        assert_eq!(Solution::get_max_grid_happiness(m, n, i, ex), expected);
    }
}
