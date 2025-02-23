#![allow(dead_code)]

// 2850. Minimum Moves to Spread Stones Over Grid
// https://leetcode.com/problems/minimum-moves-to-spread-stones-over-grid/
// https://leetcode.cn/problems/minimum-moves-to-spread-stones-over-grid/
//
// Medium
//
// You are given a 0-indexed 2D integer matrix grid of size 3 * 3, representing the number of stones in each cell. The grid contains exactly 9 stones, and there can be multiple stones in a single cell.
//
// In one move, you can move a single stone from its current cell to any other cell if the two cells share a side.
//
// Return the minimum number of moves required to place one stone in each cell.
//
// Example 1:
//
// Input: grid = [[1,1,0],[1,1,1],[1,2,1]]
// Output: 3
// Explanation: One possible sequence of moves to place one stone in each cell is:
// 1- Move one stone from cell (2,1) to cell (2,2).
// 2- Move one stone from cell (2,2) to cell (1,2).
// 3- Move one stone from cell (1,2) to cell (0,2).
// In total, it takes 3 moves to place one stone in each cell of the grid.
// It can be shown that 3 is the minimum number of moves required to place one stone in each cell.
//
// Example 2:
//
// Input: grid = [[1,3,0],[1,0,0],[1,0,3]]
// Output: 4
// Explanation: One possible sequence of moves to place one stone in each cell is:
// 1- Move one stone from cell (0,1) to cell (0,2).
// 2- Move one stone from cell (0,1) to cell (1,1).
// 3- Move one stone from cell (2,2) to cell (1,2).
// 4- Move one stone from cell (2,2) to cell (2,1).
// In total, it takes 4 moves to place one stone in each cell of the grid.
// It can be shown that 4 is the minimum number of moves required to place one stone in each cell.
//
// Constraints:
//
//     grid.length == grid[i].length == 3
//     0 <= grid[i][j] <= 9
//     Sum of grid is equal to 9.
//

struct Solution;

struct Helper {
    from: Vec<(i32, i32)>,
    to: Vec<(i32, i32)>,
}

impl Helper {
    fn new(from: Vec<(i32, i32)>, to: Vec<(i32, i32)>) -> Self {
        Self { from, to }
    }

    fn backtracking(&self, ret: &mut i32, i: usize, val: i32, flag: usize) {
        if i == self.from.len() {
            *ret = val.min(*ret);
            return;
        }

        for k in 0..self.from.len() {
            if (flag & (1 << k)) > 0 {
                continue;
            }
            let d = i32::abs(self.from[k].0 - self.to[i].0) + i32::abs(self.from[k].1 - self.to[i].1);
            self.backtracking(ret, i + 1, val + d, flag | (1 << k));
        }
    }
}

impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let (mut from, mut to) = (vec![], vec![]);

        for (i, item) in grid.iter().enumerate().take(3) {
            for (j, &item_j) in item.iter().enumerate().take(3) {
                if item_j == 0 {
                    to.push((i as i32, j as i32));
                }
                for _ in 0..item_j - 1 {
                    from.push((i as i32, j as i32));
                }
            }
        }

        let h = Helper::new(from, to);
        let mut ret = i32::MAX;
        h.backtracking(&mut ret, 0, 0, 0);

        ret
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_moves(vec![vec![1, 1, 0], vec![1, 1, 1], vec![1, 2, 1]]), 3);
    assert_eq!(Solution::minimum_moves(vec![vec![1, 3, 0], vec![1, 0, 0], vec![1, 0, 3]]), 4);
}
