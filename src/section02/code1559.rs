#![allow(dead_code)]

/*

// 1559. Detect Cycles in 2D Grid
// https://leetcode.com/problems/detect-cycles-in-2d-grid/
// https://leetcode.cn/problems/detect-cycles-in-2d-grid/
//
// Medium
//
// Given a 2D array of characters grid of size m x n, you need to find if there exists any cycle consisting of the same value in grid.

A cycle is a path of length 4 or more in the grid that starts and ends at the same cell. From a given cell, you can move to one of the cells adjacent to it - in one of the four directions (up, down, left, or right), if it has the same value of the current cell.

Also, you cannot move to the cell that you visited in your last move. For example, the cycle (1, 1) -> (1, 2) -> (1, 1) is invalid because from (1, 2) we visited (1, 1) which was the last visited cell.

Return true if any cycle of the same value exists in grid, otherwise, return false.

Example 1:

Input: grid = [["a","a","a","a"],["a","b","b","a"],["a","b","b","a"],["a","a","a","a"]]
Output: true
Explanation: There are two valid cycles shown in different colors in the image below:

Example 2:

Input: grid = [["c","c","c","a"],["c","d","c","c"],["c","c","e","c"],["f","c","c","c"]]
Output: true
Explanation: There is only one valid cycle highlighted in the image below:

Example 3:

Input: grid = [["a","b","b"],["b","z","b"],["b","b","a"]]
Output: false

Constraints:

    m == grid.length
    n == grid[i].length
    1 <= m, n <= 500
    grid consists only of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut visited = vec![vec![false; n]; m];
        for i in 0..m {
            for j in 0..n {
                if !visited[i][j] && Self::dfs(&grid, &mut visited, i, j, i, j) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize, pi: usize, pj: usize) -> bool {
        if visited[i][j] {
            return true;
        }
        visited[i][j] = true;
        let m = grid.len();
        let n = grid[0].len();
        let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (di, dj) in dirs {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                let ni = ni as usize;
                let nj = nj as usize;
                if grid[ni][nj] == grid[i][j] && (ni != pi || nj != pj) && Self::dfs(grid, visited, ni, nj, i, j) {
                    return true;
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    let grid = vec![
        vec!['a', 'a', 'a', 'a'],
        vec!['a', 'b', 'b', 'a'],
        vec!['a', 'b', 'b', 'a'],
        vec!['a', 'a', 'a', 'a'],
    ];
    assert!(Solution::contains_cycle(grid));
    let grid = vec![
        vec!['c', 'c', 'c', 'a'],
        vec!['c', 'd', 'c', 'c'],
        vec!['c', 'c', 'e', 'c'],
        vec!['f', 'c', 'c', 'c'],
    ];
    assert!(Solution::contains_cycle(grid));
    let grid = vec![vec!['a', 'b', 'b'], vec!['b', 'z', 'b'], vec!['b', 'b', 'a']];
    assert!(!Solution::contains_cycle(grid));
}
