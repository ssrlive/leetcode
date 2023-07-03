#![allow(dead_code)]

// 1260. Shift 2D Grid
// https://leetcode.com/problems/shift-2d-grid/
// https://leetcode.cn/problems/shift-2d-grid/
//
// Easy
//
// Given a 2D grid of size m x n and an integer k. You need to shift the grid k times.
//
// In one shift operation:
//
//     Element at grid[i][j] moves to grid[i][j + 1].
//     Element at grid[i][n - 1] moves to grid[i + 1][0].
//     Element at grid[m - 1][n - 1] moves to grid[0][0].
//
// Return the 2D grid after applying shift operation k times.
//
// Example 1:
//
// Input: grid = [[1,2,3],[4,5,6],[7,8,9]], k = 1
// Output: [[9,1,2],[3,4,5],[6,7,8]]
//
// Example 2:
//
// Input: grid = [[3,8,1,9],[19,7,2,5],[4,6,11,10],[12,0,21,13]], k = 4
// Output: [[12,0,21,13],[3,8,1,9],[19,7,2,5],[4,6,11,10]]
//
// Example 3:
//
// Input: grid = [[1,2,3],[4,5,6],[7,8,9]], k = 9
// Output: [[1,2,3],[4,5,6],[7,8,9]]
//
// Constraints:
//
// -    m == grid.length
// -    n == grid[i].length
// -    1 <= m <= 50
// -    1 <= n <= 50
// -    -1000 <= grid[i][j] <= 1000
// -    0 <= k <= 100
//

struct Solution;

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let (m, n) = (grid[0].len(), grid.len());
        let len = m * n;

        grid.into_iter()
            .flatten()
            .cycle()
            .skip(len - (k as usize) % len)
            .take(len)
            .collect::<Vec<_>>()
            .chunks(m)
            .map(|row| row.to_owned())
            .collect::<Vec<_>>()
    }

    pub fn shift_grid_2(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let k = k as usize;
        let mut tmp = vec![vec![0; n]; m];
        for (i, item) in grid.iter().enumerate() {
            for (j, &item2) in item.iter().enumerate() {
                let idx = (i * n + j + k) % (m * n);
                tmp[idx / n][idx % n] = item2;
            }
        }
        tmp
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            1,
            vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]],
        ),
        (
            vec![vec![3, 8, 1, 9], vec![19, 7, 2, 5], vec![4, 6, 11, 10], vec![12, 0, 21, 13]],
            4,
            vec![vec![12, 0, 21, 13], vec![3, 8, 1, 9], vec![19, 7, 2, 5], vec![4, 6, 11, 10]],
        ),
        (
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            9,
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        ),
    ];
    for (grid, k, expect) in cases {
        assert_eq!(Solution::shift_grid(grid, k), expect);
    }
}
