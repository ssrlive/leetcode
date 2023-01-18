#![allow(dead_code)]

// 1219. Path with Maximum Gold
// https://leetcode.com/problems/path-with-maximum-gold/
// https://leetcode.cn/problems/path-with-maximum-gold/
//
// Medium
//
// In a gold mine grid of size m x n, each cell in this mine has an integer representing the amount of gold in that cell, 0 if it is empty.
//
// Return the maximum amount of gold you can collect under the conditions:
//
//     Every time you are located in a cell you will collect all the gold in that cell.
//     From your position, you can walk one step to the left, right, up, or down.
//     You can't visit the same cell more than once.
//     Never visit a cell with 0 gold.
//     You can start and stop collecting gold from any position in the grid that has some gold.
//
// Example 1:
//
// Input: grid = [[0,6,0],[5,8,7],[0,9,0]]
// Output: 24
// Explanation:
// [[0,6,0],
//  [5,8,7],
//  [0,9,0]]
// Path to get the maximum gold, 9 -> 8 -> 7.
//
// Example 2:
//
// Input: grid = [[1,0,7],[2,0,6],[3,4,5],[0,3,0],[9,0,20]]
// Output: 28
// Explanation:
// [[1,0,7],
//  [2,0,6],
//  [3,4,5],
//  [0,3,0],
//  [9,0,20]]
// Path to get the maximum gold, 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7.
//
// Constraints:
//
// -    m == grid.length
// -    n == grid[i].length
// -    1 <= m, n <= 15
// -    0 <= grid[i][j] <= 100
// -    There are at most 25 cells containing gold.
//

struct Solution;

impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        fn find_max_gold(grid: &mut Vec<Vec<i32>>, m: i32, n: i32, r: i32, c: i32) -> i32 {
            const DIR: [i32; 5] = [0, 1, 0, -1, 0];
            if r < 0 || r == m || c < 0 || c == n || grid[r as usize][c as usize] == 0 {
                return 0;
            }
            let origin = grid[r as usize][c as usize];
            grid[r as usize][c as usize] = 0;
            let mut max_gold = 0;
            for i in 0..4 {
                max_gold = max_gold.max(find_max_gold(grid, m, n, DIR[i] + r, DIR[i + 1] + c));
            }
            grid[r as usize][c as usize] = origin;
            max_gold + origin
        }

        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        let mut max_gold = 0;
        for r in 0..m {
            for c in 0..n {
                max_gold = max_gold.max(find_max_gold(&mut grid.clone(), m, n, r, c));
            }
        }
        max_gold
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]], 24),
        (
            vec![
                vec![1, 0, 7],
                vec![2, 0, 6],
                vec![3, 4, 5],
                vec![0, 3, 0],
                vec![9, 0, 20],
            ],
            28,
        ),
    ];
    for (grid, expected) in cases {
        assert_eq!(Solution::get_maximum_gold(grid), expected);
    }
}
