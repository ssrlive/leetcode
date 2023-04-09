#![allow(dead_code)]

/*
// 2617. Minimum Number of Visited Cells in a Grid
// https://leetcode.com/problems/minimum-number-of-visited-cells-in-a-grid/
// https://leetcode.cn/problems/minimum-number-of-visited-cells-in-a-grid/
//
// Hard
//
// You are given a 0-indexed m x n integer matrix grid. Your initial position is at the top-left cell (0, 0).

Starting from the cell (i, j), you can move to one of the following cells:

Cells (i, k) with j < k <= grid[i][j] + j (rightward movement), or
Cells (k, j) with i < k <= grid[i][j] + i (downward movement).
Return the minimum number of cells you need to visit to reach the bottom-right cell (m - 1, n - 1). If there is no valid path, return -1.

Example 1:

Input: grid = [[3,4,2,1],[4,2,3,1],[2,1,0,0],[2,4,0,0]]
Output: 4
Explanation: The image above shows one of the paths that visits exactly 4 cells.

Example 2:

Input: grid = [[3,4,2,1],[4,2,1,1],[2,1,1,0],[3,4,1,0]]
Output: 3
Explanation: The image above shows one of the paths that visits exactly 3 cells.

Example 3:

Input: grid = [[2,1,0],[1,0,0]]
Output: -1
Explanation: It can be proven that no path exists.

Constraints:

m == grid.length
n == grid[i].length
1 <= m, n <= 10^5
1 <= m * n <= 10^5
0 <= grid[i][j] < m * n
grid[m - 1][n - 1] == 0
*/

struct Solution;

impl Solution {
    pub fn minimum_visited_cells(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut steps = 0;
        let mut max_i = vec![0; n];
        let mut max_j = vec![0; m];
        let mut q = vec![(0, 0)];
        let mut q1 = vec![];
        while !q.is_empty() {
            steps += 1;
            for &(i, j) in q.iter() {
                if i == m - 1 && j == n - 1 {
                    return steps;
                }
                for k in (max_i[j].max(i) + 1)..m {
                    if (k - i) as i32 > grid[i][j] {
                        break;
                    }
                    q1.push((k, j));
                }
                for k in (max_j[i].max(j) + 1)..n {
                    if (k - j) as i32 > grid[i][j] {
                        break;
                    }
                    q1.push((i, k));
                }
                max_i[j] = max_i[j].max(i + grid[i][j] as usize);
                max_j[i] = max_j[i].max(j + grid[i][j] as usize);
            }
            std::mem::swap(&mut q, &mut q1);
            q1.clear();
        }
        -1
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (vec![vec![3, 4, 2, 1], vec![4, 2, 3, 1], vec![2, 1, 0, 0], vec![2, 4, 0, 0]], 4),
        (vec![vec![3, 4, 2, 1], vec![4, 2, 1, 1], vec![2, 1, 1, 0], vec![3, 4, 1, 0]], 3),
        (vec![vec![2, 1, 0], vec![1, 0, 0]], -1),
    ];
    for (grid, expected) in cases {
        assert_eq!(Solution::minimum_visited_cells(grid), expected);
    }
}
