#![allow(dead_code)]

/*

// 1905. Count Sub Islands
// https://leetcode.com/problems/count-sub-islands/
// https://leetcode.cn/problems/count-sub-islands/
//
// Medium
//
// You are given two m x n binary matrices grid1 and grid2 containing only 0's (representing water) and 1's (representing land). An island is a group of 1's connected 4-directionally (horizontal or vertical). Any cells outside of the grid are considered water cells.

An island in grid2 is considered a sub-island if there is an island in grid1 that contains all the cells that make up this island in grid2.

Return the number of islands in grid2 that are considered sub-islands.

Example 1:

Input: grid1 = [[1,1,1,0,0],[0,1,1,1,1],[0,0,0,0,0],[1,0,0,0,0],[1,1,0,1,1]], grid2 = [[1,1,1,0,0],[0,0,1,1,1],[0,1,0,0,0],[1,0,1,1,0],[0,1,0,1,0]]
Output: 3
Explanation: In the picture above, the grid on the left is grid1 and the grid on the right is grid2.
The 1s colored red in grid2 are those considered to be part of a sub-island. There are three sub-islands.

Example 2:

Input: grid1 = [[1,0,1,0,1],[1,1,1,1,1],[0,0,0,0,0],[1,1,1,1,1],[1,0,1,0,1]], grid2 = [[0,0,0,0,0],[1,1,1,1,1],[0,1,0,1,0],[0,1,0,1,0],[1,0,0,0,1]]
Output: 2
Explanation: In the picture above, the grid on the left is grid1 and the grid on the right is grid2.
The 1s colored red in grid2 are those considered to be part of a sub-island. There are two sub-islands.

Constraints:

    m == grid1.length == grid2.length
    n == grid1[i].length == grid2[i].length
    1 <= m, n <= 500
    grid1[i][j] and grid2[i][j] are either 0 or 1.
*/

struct Solution;

impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid1: &Vec<Vec<i32>>, grid2: &mut Vec<Vec<i32>>, i: i32, j: i32) -> bool {
            if i < 0 || j < 0 || i >= grid2.len() as i32 || j >= grid2[0].len() as i32 {
                return true;
            }
            if grid2[i as usize][j as usize] == 0 {
                return true;
            }
            if grid1[i as usize][j as usize] == 0 {
                return false;
            }
            grid2[i as usize][j as usize] = 0;
            let up = dfs(grid1, grid2, i + 1, j);
            let down = dfs(grid1, grid2, i - 1, j);
            let left = dfs(grid1, grid2, i, j + 1);
            let right = dfs(grid1, grid2, i, j - 1);
            up && down && left && right
        }

        let mut grid2 = grid2;
        let (m, n) = (grid2.len(), grid2[0].len());
        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                if grid2[i][j] == 1 && dfs(&grid1, &mut grid2, i as i32, j as i32) {
                    res += 1;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let grid1 = vec![
        vec![1, 1, 1, 0, 0],
        vec![0, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0],
        vec![1, 1, 0, 1, 1],
    ];
    let grid2 = vec![
        vec![1, 1, 1, 0, 0],
        vec![0, 0, 1, 1, 1],
        vec![0, 1, 0, 0, 0],
        vec![1, 0, 1, 1, 0],
        vec![0, 1, 0, 1, 0],
    ];
    assert_eq!(Solution::count_sub_islands(grid1, grid2), 3);

    let grid1 = vec![
        vec![1, 0, 1, 0, 1],
        vec![1, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1],
        vec![1, 0, 1, 0, 1],
    ];
    let grid2 = vec![
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1],
        vec![0, 1, 0, 1, 0],
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 0, 0, 1],
    ];
    assert_eq!(Solution::count_sub_islands(grid1, grid2), 2);
}
