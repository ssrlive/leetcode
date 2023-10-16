#![allow(dead_code)]

// 2768. Number of Black Blocks
// https://leetcode.com/problems/number-of-black-blocks/
// https://leetcode.cn/problems/number-of-black-blocks/
//
// Medium
//
// You are given two integers m and n representing the dimensions of a 0-indexed m x n grid.
//
// You are also given a 0-indexed 2D integer matrix coordinates,
// where coordinates[i] = [x, y] indicates that the cell with coordinates [x, y] is colored black.
// All cells in the grid that do not appear in coordinates are white.
//
// A block is defined as a 2 x 2 submatrix of the grid. More formally,
// a block with cell [x, y] as its top-left corner where 0 <= x < m - 1 and 0 <= y < n - 1 contains
// the coordinates [x, y], [x + 1, y], [x, y + 1], and [x + 1, y + 1].
//
// Return a 0-indexed integer array arr of size 5 such that arr[i] is the number of blocks that contains exactly i black cells.
//
// Example 1:
//
// Input: m = 3, n = 3, coordinates = [[0,0]]
// Output: [3,1,0,0,0]
// Explanation: The grid looks like this:
//
// There is only 1 block with one black cell, and it is the block starting with cell [0,0].
// The other 3 blocks start with cells [0,1], [1,0] and [1,1]. They all have zero black cells.
// Thus, we return [3,1,0,0,0].
//
// Example 2:
//
// Input: m = 3, n = 3, coordinates = [[0,0],[1,1],[0,2]]
// Output: [0,2,2,0,0]
// Explanation: The grid looks like this:
//
// There are 2 blocks with two black cells (the ones starting with cell coordinates [0,0] and [0,1]).
// The other 2 blocks have starting cell coordinates of [1,0] and [1,1]. They both have 1 black cell.
// Therefore, we return [0,2,2,0,0].
//
// Constraints:
//
//     2 <= m <= 10^5
//     2 <= n <= 10^5
//     0 <= coordinates.length <= 10^4
//     coordinates[i].length == 2
//     0 <= coordinates[i][0] < m
//     0 <= coordinates[i][1] < n
//     It is guaranteed that coordinates contains pairwise distinct coordinates.
//

struct Solution;

impl Solution {
    pub fn count_black_blocks(m: i32, n: i32, coordinates: Vec<Vec<i32>>) -> Vec<i64> {
        let mut mp = std::collections::HashMap::<(i32, i32), i32>::new();
        let dirs = [[0, 0], [0, -1], [-1, 0], [-1, -1]];

        for c in coordinates {
            for d in &dirs {
                let (i, j) = (c[0] + d[0], c[1] + d[1]);
                if i < 0 || i + 1 == m || j < 0 || j + 1 == n {
                    continue;
                }
                *mp.entry((i, j)).or_insert(0) += 1;
            }
        }

        let mut ret = vec![0; 5];
        ret[0] = (m as i64 - 1) * (n as i64 - 1) - mp.len() as i64;
        for (_, cnt) in mp {
            ret[cnt as usize] += 1;
        }

        ret
    }
}

#[test]
fn test() {
    let m = 3;
    let n = 3;
    let coordinates = vec![vec![0, 0]];
    let res = vec![3, 1, 0, 0, 0];
    assert_eq!(Solution::count_black_blocks(m, n, coordinates), res);

    let m = 3;
    let n = 3;
    let coordinates = vec![vec![0, 0], vec![1, 1], vec![0, 2]];
    let res = vec![0, 2, 2, 0, 0];
    assert_eq!(Solution::count_black_blocks(m, n, coordinates), res);
}
