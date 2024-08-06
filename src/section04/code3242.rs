#![allow(dead_code)]

// 3242. Design Neighbor Sum Service
// https://leetcode.com/problems/design-neighbor-sum-service/
// https://leetcode.cn/problems/design-neighbor-sum-service/
//
// Easy
//
// You are given a n x n 2D array grid containing distinct elements in the range [0, n2 - 1].
//
// Implement the neighborSum class:
//
// - neighborSum(int [][]grid) initializes the object.
// - int adjacentSum(int value) returns the sum of elements which are adjacent neighbors of value,
//   that is either to the top, left, right, or bottom of value in grid.
// - int diagonalSum(int value) returns the sum of elements which are diagonal neighbors of value,
//   that is either to the top-left, top-right, bottom-left, or bottom-right of value in grid.
//
// Example 1:
//
// Input:
//
// ["neighborSum", "adjacentSum", "adjacentSum", "diagonalSum", "diagonalSum"]
//
// [[[[0, 1, 2], [3, 4, 5], [6, 7, 8]]], [1], [4], [4], [8]]
//
// Output: [null, 6, 16, 16, 4]
//
// Explanation:
//
//     The adjacent neighbors of 1 are 0, 2, and 4.
//     The adjacent neighbors of 4 are 1, 3, 5, and 7.
//     The diagonal neighbors of 4 are 0, 2, 6, and 8.
//     The diagonal neighbor of 8 is 4.
//
// Example 2:
//
// Input:
//
// ["neighborSum", "adjacentSum", "diagonalSum"]
//
// [[[[1, 2, 0, 3], [4, 7, 15, 6], [8, 9, 10, 11], [12, 13, 14, 5]]], [15], [9]]
//
// Output: [null, 23, 45]
//
// Explanation:
//
//     The adjacent neighbors of 15 are 0, 10, 7, and 6.
//     The diagonal neighbors of 9 are 4, 12, 14, and 15.
//
// Constraints:
//
//     3 <= n == grid.length == grid[0].length <= 10
//     0 <= grid[i][j] <= n^2 - 1
//     All grid[i][j] are distinct.
//     value in adjacentSum and diagonalSum will be in the range [0, n^2 - 1].
//     At most 2 * n^2 calls will be made to adjacentSum and diagonalSum.
//

#[allow(non_camel_case_types)]
struct neighborSum {
    grid: Vec<Vec<i32>>,
    n: i32,
}

impl neighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        let n = grid.len() as i32;
        Self { grid, n }
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        for i in 0..self.n {
            for j in 0..self.n {
                if self.grid[i as usize][j as usize] == value {
                    let left = if j > 0 { self.grid[i as usize][(j - 1) as usize] } else { 0 };
                    let right = if j < self.grid[i as usize].len() as i32 - 1 {
                        self.grid[i as usize][(j + 1) as usize]
                    } else {
                        0
                    };
                    let up = if i > 0 { self.grid[(i - 1) as usize][j as usize] } else { 0 };
                    let bottom = if i < self.n - 1 {
                        self.grid[(i + 1) as usize][j as usize]
                    } else {
                        0
                    };
                    return left + right + up + bottom;
                }
            }
        }
        -1
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        for i in 0..self.n {
            for j in 0..self.n {
                if self.grid[i as usize][j as usize] == value {
                    let mut sum = 0;
                    if i > 0 && j > 0 {
                        sum += self.grid[(i - 1) as usize][(j - 1) as usize];
                    }
                    if i + 1 < self.n && j + 1 < self.n {
                        sum += self.grid[(i + 1) as usize][(j + 1) as usize];
                    }
                    if i + 1 < self.n && j > 0 {
                        sum += self.grid[(i + 1) as usize][(j - 1) as usize];
                    }
                    if i > 0 && j + 1 < self.n {
                        sum += self.grid[(i - 1) as usize][(j + 1) as usize];
                    }

                    return sum;
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    let grid = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
    let ns = neighborSum::new(grid);
    assert_eq!(ns.adjacent_sum(1), 6);
    assert_eq!(ns.adjacent_sum(4), 16);
    assert_eq!(ns.diagonal_sum(4), 16);
    assert_eq!(ns.diagonal_sum(8), 4);

    let grid = vec![vec![1, 2, 0, 3], vec![4, 7, 15, 6], vec![8, 9, 10, 11], vec![12, 13, 14, 5]];
    let ns = neighborSum::new(grid);
    assert_eq!(ns.adjacent_sum(15), 23);
    assert_eq!(ns.diagonal_sum(9), 45);
}
