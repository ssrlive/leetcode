#![allow(dead_code)]

// 840. Magic Squares In Grid
// https://leetcode.com/problems/magic-squares-in-grid/
// https://leetcode.cn/problems/magic-squares-in-grid/
//
// A 3 x 3 magic square is a 3 x 3 grid filled with distinct numbers from 1 to 9 such that each row, column, and both diagonals all have the same sum.
//
// Given a row x col grid of integers, how many 3 x 3 "magic square" subgrids are there?  (Each subgrid is contiguous).
//
// Example 1:
//
// Input: grid = [[4,3,8,4],[9,5,1,9],[2,7,6,2]]
// Output: 1
// Explanation:
// The following subgrid is a 3 x 3 magic square:
//
// while this one is not:
//
// In total, there is only one magic square inside the given grid.
//
// Example 2:
//
// Input: grid = [[8]]
// Output: 0
//
// Constraints:
//
// - row == grid.length
// - col == grid[i].length
// - 1 <= row, col <= 10
// - 0 <= grid[i][j] <= 15
//

struct Solution;

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        fn check(i: usize, j: usize, grid: &[Vec<i32>]) -> bool {
            let mut mp = std::collections::HashMap::new();
            for item in grid.iter().skip(i).take(3) {
                for &point in item.iter().skip(j).take(3) {
                    if mp.contains_key(&point) || !(1..=9).contains(&point) {
                        return false;
                    } else {
                        mp.insert(point, 1);
                    }
                }
            }
            if mp.len() != 9 {
                return false;
            }
            let sum = grid[i][j] + grid[i + 1][j] + grid[i + 2][j];

            if sum != grid[i][j + 1] + grid[i + 1][j + 1] + grid[i + 2][j + 1] {
                return false;
            }

            if sum != grid[i][j + 2] + grid[i + 1][j + 2] + grid[i + 2][j + 2] {
                return false;
            }
            if sum != grid[i][j] + grid[i][j + 1] + grid[i][j + 2] {
                return false;
            }

            if sum != grid[i + 1][j] + grid[i + 1][j + 1] + grid[i + 1][j + 2] {
                return false;
            }

            if sum != grid[i + 2][j] + grid[i + 2][j + 1] + grid[i + 2][j + 2] {
                return false;
            }

            if sum != grid[i][j] + grid[i + 1][j + 1] + grid[i + 2][j + 2] {
                return false;
            }

            if sum != grid[i][j + 2] + grid[i + 1][j + 1] + grid[i + 2][j] {
                return false;
            }
            true
        }

        let mut count = 0;
        if grid.len() < 3 || grid[0].len() < 3 {
            return 0;
        }
        for i in 0..grid.len() - 2 {
            for j in 0..grid[0].len() - 2 {
                if check(i, j, &grid) {
                    count += 1;
                }
            }
        }
        count
    }
}

#[test]
fn test() {
    let grid = vec![vec![4, 3, 8, 4], vec![9, 5, 1, 9], vec![2, 7, 6, 2]];
    assert_eq!(Solution::num_magic_squares_inside(grid), 1);

    assert_eq!(Solution::num_magic_squares_inside(vec![vec![8]]), 0);
}
