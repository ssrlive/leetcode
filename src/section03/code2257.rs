#![allow(dead_code)]

/*

// 2257. Count Unguarded Cells in the Grid
// https://leetcode.com/problems/count-unguarded-cells-in-the-grid/
// https://leetcode.cn/problems/count-unguarded-cells-in-the-grid/
//
// Medium
//
// You are given two integers m and n representing a 0-indexed m x n grid. You are also given two 2D integer arrays guards and walls where guards[i] = [rowi, coli] and walls[j] = [rowj, colj] represent the positions of the ith guard and jth wall respectively.

A guard can see every cell in the four cardinal directions (north, east, south, or west) starting from their position unless obstructed by a wall or another guard. A cell is guarded if there is at least one guard that can see it.

Return the number of unoccupied cells that are not guarded.

Example 1:

Input: m = 4, n = 6, guards = [[0,0],[1,1],[2,3]], walls = [[0,1],[2,2],[1,4]]
Output: 7
Explanation: The guarded and unguarded cells are shown in red and green respectively in the above diagram.
There are a total of 7 unguarded cells, so we return 7.

Example 2:

Input: m = 3, n = 3, guards = [[1,1]], walls = [[0,1],[1,0],[2,1],[1,2]]
Output: 4
Explanation: The unguarded cells are shown in green in the above diagram.
There are a total of 4 unguarded cells, so we return 4.

Constraints:

    1 <= m, n <= 10^5
    2 <= m * n <= 10^5
    1 <= guards.length, walls.length <= 5 * 10^4
    2 <= guards.length + walls.length <= m * n
    guards[i].length == walls[j].length == 2
    0 <= rowi, rowj < m
    0 <= coli, colj < n
    All the positions in guards and walls are unique.
*/

struct Solution;

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let (unguarded, guarded, _guard, _wall) = (0, 1, 2, 3);
        let mut cnt = (m * n) - guards.len() as i32 - walls.len() as i32;
        let mut matrix: Vec<Vec<i32>> = vec![vec![0; n as usize]; m as usize];
        for wall in walls {
            let i = wall[0] as usize;
            let j = wall[1] as usize;
            matrix[i][j] = _wall;
        }
        for guard in guards.iter() {
            let i = guard[0] as usize;
            let j = guard[1] as usize;
            matrix[i][j] = _guard;
        }
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for guard in guards {
            for &(dx, dy) in dirs.iter() {
                let mut x = guard[0] + dx;
                let mut y = guard[1] + dy;
                while x >= 0 && y >= 0 && x < m && y < n {
                    let (i, j) = (x as usize, y as usize);
                    if matrix[i][j] == _wall || matrix[i][j] == _guard {
                        break;
                    }
                    if matrix[i][j] == unguarded {
                        cnt -= 1;
                    }
                    matrix[i][j] = guarded;
                    x += dx;
                    y += dy;
                }
            }
        }
        cnt
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            4,
            6,
            vec![vec![0, 0], vec![1, 1], vec![2, 3]],
            vec![vec![0, 1], vec![2, 2], vec![1, 4]],
            7,
        ),
        (
            3,
            3,
            vec![vec![1, 1]],
            vec![vec![0, 1], vec![1, 0], vec![2, 1], vec![1, 2]],
            4,
        ),
    ];
    for (m, n, guards, walls, expected) in cases {
        assert_eq!(Solution::count_unguarded(m, n, guards, walls), expected);
    }
}
