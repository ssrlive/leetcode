#![allow(dead_code)]

/*

// 2245. Maximum Trailing Zeros in a Cornered Path
// https://leetcode.com/problems/maximum-trailing-zeros-in-a-cornered-path/
// https://leetcode.cn/problems/maximum-trailing-zeros-in-a-cornered-path/
//
// Medium
//
// You are given a 2D integer array grid of size m x n, where each cell contains a positive integer.

A cornered path is defined as a set of adjacent cells with at most one turn. More specifically, the path should exclusively move either horizontally or vertically up to the turn (if there is one), without returning to a previously visited cell. After the turn, the path will then move exclusively in the alternate direction: move vertically if it moved horizontally, and vice versa, also without returning to a previously visited cell.

The product of a path is defined as the product of all the values in the path.

Return the maximum number of trailing zeros in the product of a cornered path found in grid.

Note:

    Horizontal movement means moving in either the left or right direction.
    Vertical movement means moving in either the up or down direction.

Example 1:

Input: grid = [[23,17,15,3,20],[8,1,20,27,11],[9,4,6,2,21],[40,9,1,10,6],[22,7,4,5,3]]
Output: 3
Explanation: The grid on the left shows a valid cornered path.
It has a product of 15 * 20 * 6 * 1 * 10 = 18000 which has 3 trailing zeros.
It can be shown that this is the maximum trailing zeros in the product of a cornered path.

The grid in the middle is not a cornered path as it has more than one turn.
The grid on the right is not a cornered path as it requires a return to a previously visited cell.

Example 2:

Input: grid = [[4,3,2],[7,6,1],[8,8,8]]
Output: 0
Explanation: The grid is shown in the figure above.
There are no cornered paths in the grid that result in a product with a trailing zero.

Constraints:

    m == grid.length
    n == grid[i].length
    1 <= m, n <= 10^5
    1 <= m * n <= 10^5
    1 <= grid[i][j] <= 1000
*/

struct Solution;

impl Solution {
    pub fn max_trailing_zeros(grid: Vec<Vec<i32>>) -> i32 {
        use std::ops::{Add, Sub};
        let m = grid.len();
        let n = grid[0].len();

        #[derive(Clone, Copy, Debug, Default)]
        struct Counts {
            count_2: i32,
            count_5: i32,
        }

        impl Counts {
            fn zeros(&self) -> i32 {
                self.count_2.min(self.count_5)
            }
        }

        impl Add for Counts {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    count_2: self.count_2 + rhs.count_2,
                    count_5: self.count_5 + rhs.count_5,
                }
            }
        }

        impl Sub for Counts {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    count_2: self.count_2 - rhs.count_2,
                    count_5: self.count_5 - rhs.count_5,
                }
            }
        }

        let mut prime_count = vec![Counts::default(); 1001];
        for (i, item) in prime_count.iter_mut().enumerate().skip(2) {
            let mut count_2 = 0;
            let mut count_5 = 0;
            let mut k = i;
            while k.is_multiple_of(2) {
                k /= 2;
                count_2 += 1;
            }
            while k.is_multiple_of(5) {
                k /= 5;
                count_5 += 1;
            }
            *item = Counts { count_2, count_5 };
        }
        let mut h_acc = vec![vec![Counts::default(); n + 1]; m + 1];
        let mut v_acc = vec![vec![Counts::default(); m + 1]; n + 1];

        for row in 0..m {
            for col in 0..n {
                let count = prime_count[grid[row][col] as usize];
                h_acc[row][col + 1] = h_acc[row][col] + count;
                v_acc[col][row + 1] = v_acc[col][row] + count;
            }
        }

        let mut res = 0;
        for row in 0..m {
            for col in 0..n {
                let top = v_acc[col][row];
                let bottom = v_acc[col][m] - v_acc[col][row + 1];
                let left = h_acc[row][col];
                let right = h_acc[row][n] - h_acc[row][col + 1];
                let current = prime_count[grid[row][col] as usize];
                res = res
                    .max((top + left + current).zeros())
                    .max((top + right + current).zeros())
                    .max((bottom + left + current).zeros())
                    .max((bottom + right + current).zeros());
            }
        }
        res
    }
}

#[test]
fn test() {
    let grid = vec![
        vec![23, 17, 15, 3, 20],
        vec![8, 1, 20, 27, 11],
        vec![9, 4, 6, 2, 21],
        vec![40, 9, 1, 10, 6],
        vec![22, 7, 4, 5, 3],
    ];
    assert_eq!(Solution::max_trailing_zeros(grid), 3);

    let grid = vec![vec![4, 3, 2], vec![7, 6, 1], vec![8, 8, 8]];
    assert_eq!(Solution::max_trailing_zeros(grid), 0);
}
