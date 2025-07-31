#![allow(dead_code)]

// 3619. Count Islands With Total Value Divisible by K
// https://leetcode.com/problems/count-islands-with-total-value-divisible-by-k/
// https://leetcode.cn/problems/count-islands-with-total-value-divisible-by-k/
//
// Medium
//
// You are given an m x n matrix grid and a positive integer k. An island is a group of positive integers
// (representing land) that are 4-directionally connected (horizontally or vertically).
//
// The total value of an island is the sum of the values of all cells in the island.
//
// Return the number of islands with a total value divisible by k.
//
// Example 1:
//
// Input: grid = [[0,2,1,0,0],[0,5,0,0,5],[0,0,1,0,0],[0,1,4,7,0],[0,2,0,0,8]], k = 5
//
// Output: 2
//
// Explanation:
//
// The grid contains four islands. The islands highlighted in blue have a total value that is divisible by 5, while the islands highlighted in red do not.
//
// Example 2:
//
// Input: grid = [[3,0,3,0], [0,3,0,3], [3,0,3,0]], k = 3
//
// Output: 6
//
// Explanation:
//
// The grid contains six islands, each with a total value that is divisible by 3.
//
// Constraints:
//
// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 1000
// 1 <= m * n <= 10^5
// 0 <= grid[i][j] <= 10^6
// 1 <= k <= 10^6
//

struct Solution;

impl Solution {
    pub fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut hit = 0;
        let mut grid = grid;
        let rows = grid.len();
        let cols = grid[0].len();
        let mut stack: Vec<(usize, usize)> = Vec::new();

        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] > 0 {
                    let mut sum = 0;
                    stack.push((row, col));

                    while let Some((r, c)) = stack.pop() {
                        let curr = grid[r][c];
                        if curr <= 0 || r >= rows || c >= cols {
                            continue;
                        }
                        sum += curr;
                        grid[r][c] = -1;

                        if r > 0 {
                            stack.push((r - 1, c));
                        }
                        if c > 0 {
                            stack.push((r, c - 1));
                        }
                        if r + 1 < rows {
                            stack.push((r + 1, c));
                        }
                        if c + 1 < cols {
                            stack.push((r, c + 1));
                        }
                    }

                    if sum % k == 0 {
                        hit += 1;
                    }
                }
            }
        }

        hit
    }
}

#[test]
fn test() {
    let grid = vec![
        vec![0, 2, 1, 0, 0],
        vec![0, 5, 0, 0, 5],
        vec![0, 0, 1, 0, 0],
        vec![0, 1, 4, 7, 0],
        vec![0, 2, 0, 0, 8],
    ];
    let k = 5;
    assert_eq!(Solution::count_islands(grid, k), 2);

    let grid = vec![vec![3, 0, 3, 0], vec![0, 3, 0, 3], vec![3, 0, 3, 0]];
    let k = 3;
    assert_eq!(Solution::count_islands(grid, k), 6);
}
