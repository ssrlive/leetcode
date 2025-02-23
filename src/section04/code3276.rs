#![allow(dead_code)]

// 3276. Select Cells in Grid With Maximum Score
// https://leetcode.com/problems/select-cells-in-grid-with-maximum-score/
// https://leetcode.cn/problems/select-cells-in-grid-with-maximum-score/
//
// Hard
//
// You are given a 2D matrix grid consisting of positive integers.
//
// You have to select one or more cells from the matrix such that the following conditions are satisfied:
//
//     No two selected cells are in the same row of the matrix.
//     The values in the set of selected cells are unique.
//
// Your score will be the sum of the values of the selected cells.
//
// Return the maximum score you can achieve.
//
// Example 1:
//
// Input: grid = [[1,2,3],[4,3,2],[1,1,1]]
//
// Output: 8
//
// Explanation:
//
// We can select the cells with values 1, 3, and 4 that are colored above.
//
// Example 2:
//
// Input: grid = [[8,7,6],[8,3,2]]
//
// Output: 15
//
// Explanation:
//
// We can select the cells with values 7 and 8 that are colored above.
//
// Constraints:
//
//     1 <= grid.length, grid[i].length <= 10
//     1 <= grid[i][j] <= 100
//

struct Solution;

use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Cell {
    loc: (usize, usize),
    val: i32,
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val).then(self.loc.cmp(&other.loc))
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut cells = Vec::new();
        for (i, grid_i) in grid.iter().enumerate().take(m) {
            for (j, &grid_i_j) in grid_i.iter().enumerate().take(n) {
                cells.push(Cell {
                    loc: (i, j),
                    val: grid_i_j,
                });
            }
        }
        cells.sort_unstable();

        let mut i = 0;
        let mut dp = vec![0; 1 << m];
        while i < cells.len() {
            let mut j = i;
            while j < cells.len() && cells[i].val == cells[j].val {
                j += 1;
            }
            let mut tmp = dp.clone();
            for cells_k in cells.iter().take(j).skip(i) {
                let c = cells_k;
                for m in 0..dp.len() {
                    if m & (1 << c.loc.0) != 0 {
                        continue;
                    }
                    tmp[m | (1 << c.loc.0)] = tmp[m | (1 << c.loc.0)].max(dp[m] + c.val);
                }
            }

            dp = tmp;
            i = j;
        }
        dp[dp.len() - 1]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_score(vec![vec![1, 2, 3], vec![4, 3, 2], vec![1, 1, 1]]), 8);
    assert_eq!(Solution::max_score(vec![vec![8, 7, 6], vec![8, 3, 2]]), 15);
}
