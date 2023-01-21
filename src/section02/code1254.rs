#![allow(dead_code)]

// 1254. Number of Closed Islands
// https://leetcode.com/problems/number-of-closed-islands/
// https://leetcode.cn/problems/number-of-closed-islands/
//
// Medium
//
// Given a 2D grid consists of 0s (land) and 1s (water).  An island is a maximal 4-directionally connected group
// of 0s and a closed island is an island totally (all left, top, right, bottom) surrounded by 1s.
//
// Return the number of closed islands.
//
// Example 1:
//
// Input: grid = [[1,1,1,1,1,1,1,0],[1,0,0,0,0,1,1,0],[1,0,1,0,1,1,1,0],[1,0,0,0,0,1,0,1],[1,1,1,1,1,1,1,0]]
// Output: 2
// Explanation:
// Islands in gray are closed because they are completely surrounded by water (group of 1s).
//
// Example 2:
//
// Input: grid = [[0,0,1,0,0],[0,1,0,1,0],[0,1,1,1,0]]
// Output: 1
//
// Example 3:
//
// Input: grid = [[1,1,1,1,1,1,1],
//                [1,0,0,0,0,0,1],
//                [1,0,1,1,1,0,1],
//                [1,0,1,0,1,0,1],
//                [1,0,1,1,1,0,1],
//                [1,0,0,0,0,0,1],
//                [1,1,1,1,1,1,1]]
// Output: 2
//
// Constraints:
//
// -    1 <= grid.length, grid[0].length <= 100
// -    0 <= grid[i][j] <=1
//

struct Solution;

impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let grid_len = grid.len() as i32;
        let row_len = grid[0].len() as i32;

        for i in 0..grid_len {
            if grid[i as usize][0] == 0 {
                Solution::convert_island(&mut grid, i, 0);
            }

            if grid[i as usize][(row_len - 1) as usize] == 0 {
                Solution::convert_island(&mut grid, i, row_len - 1);
            }
        }

        for i in 0..row_len {
            if grid[0][i as usize] == 0 {
                Solution::convert_island(&mut grid, 0, i);
            }

            if grid[(grid_len - 1) as usize][i as usize] == 0 {
                Solution::convert_island(&mut grid, grid_len - 1, i);
            }
        }

        let mut counter = 0;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 0 {
                    counter += 1;
                    Solution::convert_island(&mut grid, i as i32, j as i32);
                }
            }
        }

        counter
    }

    fn convert_island(grid: &mut Vec<Vec<i32>>, x: i32, y: i32) {
        if x < 0 || y < 0 || x >= grid.len() as i32 || y >= grid[0].len() as i32 || grid[x as usize][y as usize] == 1 {
            return;
        }

        grid[x as usize][y as usize] = 1;

        Solution::convert_island(grid, x + 1, y);
        Solution::convert_island(grid, x - 1, y);
        Solution::convert_island(grid, x, y + 1);
        Solution::convert_island(grid, x, y - 1);
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![
                vec![1, 1, 1, 1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0, 1, 1, 0],
                vec![1, 0, 1, 0, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0, 1, 0, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 0],
            ],
            2,
        ),
        (vec![vec![0, 0, 1, 0, 0], vec![0, 1, 0, 1, 0], vec![0, 1, 1, 1, 0]], 1),
        (
            vec![
                vec![1, 1, 1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 1, 1, 1, 0, 1],
                vec![1, 0, 1, 0, 1, 0, 1],
                vec![1, 0, 1, 1, 1, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1, 1, 1],
            ],
            2,
        ),
    ];
    for (grid, expect) in cases {
        assert_eq!(Solution::closed_island(grid), expect);
    }
}
