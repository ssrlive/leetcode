#![allow(dead_code)]

/*

// 1970. Last Day Where You Can Still Cross
// https://leetcode.com/problems/last-day-where-you-can-still-cross/
// https://leetcode.cn/problems/last-day-where-you-can-still-cross/
//
// Hard
//
// There is a 1-based binary matrix where 0 represents land and 1 represents water. You are given integers row and col representing the number of rows and columns in the matrix, respectively.

Initially on day 0, the entire matrix is land. However, each day a new cell becomes flooded with water. You are given a 1-based 2D array cells, where cells[i] = [ri, ci] represents that on the ith day, the cell on the rith row and cith column (1-based coordinates) will be covered with water (i.e., changed to 1).

You want to find the last day that it is possible to walk from the top to the bottom by only walking on land cells. You can start from any cell in the top row and end at any cell in the bottom row. You can only travel in the four cardinal directions (left, right, up, and down).

Return the last day where it is possible to walk from the top to the bottom by only walking on land cells.

Example 1:

Input: row = 2, col = 2, cells = [[1,1],[2,1],[1,2],[2,2]]
Output: 2
Explanation: The above image depicts how the matrix changes each day starting from day 0.
The last day where it is possible to cross from top to bottom is on day 2.

Example 2:

Input: row = 2, col = 2, cells = [[1,1],[1,2],[2,1],[2,2]]
Output: 1
Explanation: The above image depicts how the matrix changes each day starting from day 0.
The last day where it is possible to cross from top to bottom is on day 1.

Example 3:

Input: row = 3, col = 3, cells = [[1,2],[2,1],[3,3],[2,2],[1,1],[1,3],[2,3],[3,2],[3,1]]
Output: 3
Explanation: The above image depicts how the matrix changes each day starting from day 0.
The last day where it is possible to cross from top to bottom is on day 3.

Constraints:

    2 <= row, col <= 2 * 10^4
    4 <= row * col <= 2 * 10^4
    cells.length == row * col
    1 <= ri <= row
    1 <= ci <= col
    All the values of cells are unique.
*/

struct Solution;

impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &mut Vec<Vec<i32>>, visited: &mut Vec<Vec<i32>>, i: usize, j: usize) -> bool {
            let dirs = [[1, 0], [0, 1], [-1, 0], [0, -1]];
            let (m, n) = (grid.len(), grid[0].len());
            for d in dirs {
                let (u, v) = (i as i32 + d[0], j as i32 + d[1]);
                if u < 0 || u == m as i32 || v < 0 || v == n as i32 {
                    continue;
                }
                let (u, v) = (u as usize, v as usize);
                if grid[u][v] == 1 {
                    continue;
                }
                if u == m - 1 {
                    return true;
                }
                if visited[u][v] == 1 {
                    continue;
                }
                visited[u][v] = 1;
                if dfs(grid, visited, u, v) {
                    return true;
                }
            }
            false
        }

        let dirs = [[1, 0], [0, 1], [-1, 0], [0, -1]];
        let (m, n) = (row as usize, col as usize);
        let (mut grid, mut visited) = (vec![vec![1i32; n]; m], vec![vec![0i32; n]; m]);
        for k in (0..cells.len()).rev() {
            let (i, j) = (cells[k][0] as usize - 1, cells[k][1] as usize - 1);
            grid[i][j] = 0;
            let mut good = false;
            if i == 0 {
                good = true;
            }
            for d in dirs {
                let (u, v) = (i as i32 + d[0], j as i32 + d[1]);
                if u < 0 || u == m as i32 || v < 0 || v == n as i32 {
                    continue;
                }
                let (u, v) = (u as usize, v as usize);
                if grid[u][v] != 1 && visited[u][v] == 1 {
                    good = true;
                }
            }
            if !good {
                continue;
            }
            visited[i][j] = 1;
            if i == m - 1 {
                return k as i32;
            }
            if dfs(&mut grid, &mut visited, i, j) {
                return k as i32;
            }
        }
        panic![]
    }
}

#[test]
fn test() {
    let cases = vec![
        (2, 2, vec![vec![1, 1], vec![2, 1], vec![1, 2], vec![2, 2]], 2),
        (2, 2, vec![vec![1, 1], vec![1, 2], vec![2, 1], vec![2, 2]], 1),
        (
            3,
            3,
            vec![
                vec![1, 2],
                vec![2, 1],
                vec![3, 3],
                vec![2, 2],
                vec![1, 1],
                vec![1, 3],
                vec![2, 3],
                vec![3, 2],
                vec![3, 1],
            ],
            3,
        ),
    ];
    for (row, col, cells, expected) in cases {
        assert_eq!(Solution::latest_day_to_cross(row, col, cells), expected);
    }
}
