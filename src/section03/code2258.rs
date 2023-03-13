#![allow(dead_code)]

/*

// 2258. Escape the Spreading Fire
// https://leetcode.com/problems/escape-the-spreading-fire/
// https://leetcode.cn/problems/escape-the-spreading-fire/
//
// Hard
//
// You are given a 0-indexed 2D integer array grid of size m x n which represents a field. Each cell has one of three values:

    0 represents grass,
    1 represents fire,
    2 represents a wall that you and fire cannot pass through.

You are situated in the top-left cell, (0, 0), and you want to travel to the safehouse at the bottom-right cell, (m - 1, n - 1). Every minute, you may move to an adjacent grass cell. After your move, every fire cell will spread to all adjacent cells that are not walls.

Return the maximum number of minutes that you can stay in your initial position before moving while still safely reaching the safehouse. If this is impossible, return -1. If you can always reach the safehouse regardless of the minutes stayed, return 109.

Note that even if the fire spreads to the safehouse immediately after you have reached it, it will be counted as safely reaching the safehouse.

A cell is adjacent to another cell if the former is directly north, east, south, or west of the latter (i.e., their sides are touching).

Example 1:

Input: grid = [[0,2,0,0,0,0,0],[0,0,0,2,2,1,0],[0,2,0,0,1,2,0],[0,0,2,2,2,0,2],[0,0,0,0,0,0,0]]
Output: 3
Explanation: The figure above shows the scenario where you stay in the initial position for 3 minutes.
You will still be able to safely reach the safehouse.
Staying for more than 3 minutes will not allow you to safely reach the safehouse.

Example 2:

Input: grid = [[0,0,0,0],[0,1,2,0],[0,2,0,0]]
Output: -1
Explanation: The figure above shows the scenario where you immediately move towards the safehouse.
Fire will spread to any cell you move towards and it is impossible to safely reach the safehouse.
Thus, -1 is returned.

Example 3:

Input: grid = [[0,0,0],[2,2,0],[1,2,0]]
Output: 1000000000
Explanation: The figure above shows the initial grid.
Notice that the fire is contained by walls and you will always be able to safely reach the safehouse.
Thus, 109 is returned.

Constraints:

    m == grid.length
    n == grid[i].length
    2 <= m, n <= 300
    4 <= m * n <= 2 * 10^4
    grid[i][j] is either 0, 1, or 2.
    grid[0][0] == grid[m - 1][n - 1] == 0
*/

struct Solution;

impl Solution {
    pub fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        let (m, n) = (grid.len(), grid[0].len());
        let mut fire: VecDeque<(usize, usize)> = VecDeque::new();
        let mut person: VecDeque<(usize, usize)> = VecDeque::new();
        for (i, grid_i) in grid.iter().enumerate() {
            for (j, &grid_i_j) in grid_i.iter().enumerate() {
                if grid_i_j == 1 {
                    fire.push_back((i, j));
                }
            }
        }
        person.push_back((0, 0));
        let steps = |pos: &mut VecDeque<(usize, usize)>| {
            let mut st = vec![vec![0; n]; m];
            while let Some((i, j)) = pos.pop_front() {
                for (di, dj) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let (x, y) = (i as i32 + di, j as i32 + dj);
                    if x >= 0 && y >= 0 && x < m as i32 && y < n as i32 {
                        let (x, y) = (x as usize, y as usize);
                        if grid[x][y] == 0 && st[x][y] == 0 {
                            st[x][y] = st[i][j] + 1;
                            pos.push_back((x, y));
                        }
                    }
                }
            }
            let (a, b, c) = (st[m - 1][n - 1], st[m - 2][n - 1], st[m - 1][n - 2]);
            [a, b, c]
        };
        let f = steps(&mut fire);
        let p = steps(&mut person);
        if f[0] == 0 && p[0] != 0 {
            return 1000000000;
        }
        let diff = f[0] - p[0];
        if p[0] != 0 && diff >= 0 {
            return diff - (f[1] - p[1] <= diff && f[2] - p[2] <= diff) as i32;
        }
        -1
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![
                vec![0, 2, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 2, 2, 1, 0],
                vec![0, 2, 0, 0, 1, 2, 0],
                vec![0, 0, 2, 2, 2, 0, 2],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
            3,
        ),
        (vec![vec![0, 0, 0, 0], vec![0, 1, 2, 0], vec![0, 2, 0, 0]], -1),
        (vec![vec![0, 0, 0], vec![2, 2, 0], vec![1, 2, 0]], 1000000000),
    ];
    for (grid, expected) in cases {
        assert_eq!(Solution::maximum_minutes(grid), expected);
    }
}
