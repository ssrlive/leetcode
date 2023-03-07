#![allow(dead_code)]

/*

// 2373. Largest Local Values in a Matrix
// https://leetcode.com/problems/largest-local-values-in-a-matrix/
// https://leetcode.cn/problems/largest-local-values-in-a-matrix/
//
// Easy
//
// You are given an n x n integer matrix grid.

Generate an integer matrix maxLocal of size (n - 2) x (n - 2) such that:

    maxLocal[i][j] is equal to the largest value of the 3 x 3 matrix in grid centered around row i + 1 and column j + 1.

In other words, we want to find the largest value in every contiguous 3 x 3 matrix in grid.

Return the generated matrix.

Example 1:

Input: grid = [[9,9,8,1],[5,6,2,6],[8,2,6,4],[6,2,2,2]]
Output: [[9,9],[8,6]]
Explanation: The diagram above shows the original matrix and the generated matrix.
Notice that each value in the generated matrix corresponds to the largest value of a contiguous 3 x 3 matrix in grid.

Example 2:

Input: grid = [[1,1,1,1,1],[1,1,1,1,1],[1,1,2,1,1],[1,1,1,1,1],[1,1,1,1,1]]
Output: [[2,2,2],[2,2,2],[2,2,2]]
Explanation: Notice that the 2 is contained within every contiguous 3 x 3 matrix in grid.

Constraints:

    n == grid.length == grid[i].length
    3 <= n <= 100
    1 <= grid[i][j] <= 100
*/

struct Solution;

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn largest_in_area(start_x: usize, start_y: usize, grid: &[Vec<i32>]) -> i32 {
            let mut largest: i32 = i32::MIN;
            for x_index in start_x..(start_x + 3) {
                for grid_y in grid.iter().skip(start_y).take(3) {
                    if grid_y[x_index] > largest {
                        largest = grid_y[x_index];
                    }
                }
            }
            largest
        }

        let mut res: Vec<Vec<i32>> = vec![vec![i32::MIN; grid.len() - 2]; grid.len() - 2];
        for (y, res_y) in res.iter_mut().enumerate().take(grid.len() - 2) {
            for (x, res_y_x) in res_y.iter_mut().enumerate().take(grid.len() - 2) {
                *res_y_x = largest_in_area(x, y, &grid);
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![9, 9, 8, 1], vec![5, 6, 2, 6], vec![8, 2, 6, 4], vec![6, 2, 2, 2]],
            vec![vec![9, 9], vec![8, 6]],
        ),
        (
            vec![
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 2, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1],
            ],
            vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 2]],
        ),
    ];
    for (grid, expect) in cases {
        assert_eq!(Solution::largest_local(grid), expect);
    }
}
