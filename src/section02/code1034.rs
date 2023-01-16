#![allow(dead_code)]

// 1034. Coloring A Border
// https://leetcode.com/problems/coloring-a-border/
// https://leetcode.cn/problems/coloring-a-border/
//
// You are given an m x n integer matrix grid, and three integers row, col, and color. Each value in the grid represents the color of the grid square at that location.
//
// Two squares belong to the same connected component if they have the same color and are next to each other in any of the 4 directions.
//
// The border of a connected component is all the squares in the connected component that are either 4-directionally adjacent to a square not in the component, or on the boundary of the grid (the first or last row or column).
//
// You should color the border of the connected component that contains the square grid[row][col] with color.
//
// Return the final grid.
//
// Example 1:
//
// Input: grid = [[1,1],[1,2]], row = 0, col = 0, color = 3
// Output: [[3,3],[3,2]]
//
// Example 2:
//
// Input: grid = [[1,2,2],[2,3,2]], row = 0, col = 1, color = 3
// Output: [[1,3,3],[2,3,3]]
//
// Example 3:
//
// Input: grid = [[1,1,1],[1,1,1],[1,1,1]], row = 1, col = 1, color = 2
// Output: [[2,2,2],[2,1,2],[2,2,2]]
//
// Constraints:
//
// - m == grid.length
// - n == grid[i].length
// - 1 <= m, n <= 50
// - 1 <= grid[i][j], color <= 1000
// - 0 <= row < m
// - 0 <= col < n
//

/*
// cpp solution
class Solution {
public:
    vector<vector<int>> colorBorder(vector<vector<int>>& grid, int row, int col, int color) {
        dfs(grid, row, col, grid[row][col]);
        for (auto i = 0; i < grid.size(); ++i) {
            for (auto j = 0; j < grid[i].size(); ++j) {
                grid[i][j] = grid[i][j] < 0 ? color : grid[i][j];
            }
        }
        return grid;
    }

    void dfs(vector<vector<int>>& g, int r, int c, int cl) {
        if (r < 0 || c < 0 || r >= g.size() || c >= g[r].size() || g[r][c] != cl) {
            return;
        }
        g[r][c] = -cl;
        dfs(g, r - 1, c, cl), dfs(g, r + 1, c, cl), dfs(g, r, c - 1, cl), dfs(g, r, c + 1, cl);
        if (r > 0 && r < g.size() - 1 && c > 0 && c < g[r].size() - 1 && cl == abs(g[r - 1][c]) &&
            cl == abs(g[r + 1][c]) && cl == abs(g[r][c - 1]) && cl == abs(g[r][c + 1])) {
            g[r][c] = cl;
        }
    }
};
*/

struct Solution;

impl Solution {
    pub fn color_border(grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        fn dfs(grid: &mut Vec<Vec<i32>>, r: i32, c: i32, cl: i32) {
            if r < 0
                || r >= grid.len() as i32
                || c < 0
                || c >= grid[r as usize].len() as i32
                || grid[r as usize][c as usize] != cl
            {
                return;
            }
            grid[r as usize][c as usize] = -cl;
            dfs(grid, r - 1, c, cl);
            dfs(grid, r + 1, c, cl);
            dfs(grid, r, c - 1, cl);
            dfs(grid, r, c + 1, cl);
            if r > 0
                && r < grid.len() as i32 - 1
                && c > 0
                && c < grid[r as usize].len() as i32 - 1
                && cl == grid[(r - 1) as usize][c as usize].abs()
                && cl == grid[(r + 1) as usize][c as usize].abs()
                && cl == grid[r as usize][(c - 1) as usize].abs()
                && cl == grid[r as usize][(c + 1) as usize].abs()
            {
                grid[r as usize][c as usize] = cl;
            }
        }

        let mut grid = grid;
        let cl = grid[row as usize][col as usize];
        dfs(&mut grid, row, col, cl);
        for item in grid.iter_mut() {
            for pt in item.iter_mut() {
                *pt = if *pt < 0 { color } else { *pt };
            }
        }
        grid
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 1], vec![1, 2]], 0, 0, 3, vec![vec![3, 3], vec![3, 2]]),
        (
            vec![vec![1, 2, 2], vec![2, 3, 2]],
            0,
            1,
            3,
            vec![vec![1, 3, 3], vec![2, 3, 3]],
        ),
        (
            vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]],
            1,
            1,
            2,
            vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]],
        ),
    ];
    for (grid, row, col, color, expect) in cases {
        assert_eq!(Solution::color_border(grid, row, col, color), expect);
    }
}
