#![allow(dead_code)]

// 2711. Difference of Number of Distinct Values on Diagonals
// https://leetcode.com/problems/difference-of-number-of-distinct-values-on-diagonals/
// https://leetcode.cn/problems/difference-of-number-of-distinct-values-on-diagonals/
//
// Medium
//
// Given a 0-indexed 2D grid of size m x n, you should find the matrix answer of size m x n.
//
// The value of each cell (r, c) of the matrix answer is calculated in the following way:
//
//     - Let topLeft[r][c] be the number of distinct values in the top-left diagonal of the cell (r, c) in the matrix grid.
//     - Let bottomRight[r][c] be the number of distinct values in the bottom-right diagonal of the cell (r, c) in the matrix grid.
//
// Then answer[r][c] = |topLeft[r][c] - bottomRight[r][c]|.
//
// Return the matrix answer.
//
// A matrix diagonal is a diagonal line of cells starting from some cell in either the topmost row
// or leftmost column and going in the bottom-right direction until reaching the matrix's end.
//
// A cell (r1, c1) belongs to the top-left diagonal of the cell (r, c), if both belong
// to the same diagonal and r1 < r. Similarly is defined bottom-right diagonal.
//
// Example 1:
//
// Input: grid = [[1,2,3],[3,1,5],[3,2,1]]
// Output: [[1,1,0],[1,0,1],[0,1,1]]
// Explanation: The 1st diagram denotes the initial grid.
// The 2nd diagram denotes a grid for cell (0,0), where blue-colored cells are cells on its bottom-right diagonal.
// The 3rd diagram denotes a grid for cell (1,2), where red-colored cells are cells on its top-left diagonal.
// The 4th diagram denotes a grid for cell (1,1), where blue-colored cells are cells on its bottom-right diagonal and red-colored cells are cells on its top-left diagonal.
// - The cell (0,0) contains [1,1] on its bottom-right diagonal and [] on its top-left diagonal. The answer is |1 - 0| = 1.
// - The cell (1,2) contains [] on its bottom-right diagonal and [2] on its top-left diagonal. The answer is |0 - 1| = 1.
// - The cell (1,1) contains [1] on its bottom-right diagonal and [1] on its top-left diagonal. The answer is |1 - 1| = 0.
// The answers of other cells are similarly calculated.
//
// Example 2:
//
// Input: grid = [[1]]
// Output: [[0]]
// Explanation: - The cell (0,0) contains [] on its bottom-right diagonal and [] on its top-left diagonal. The answer is |0 - 0| = 0.
//
// Constraints:
//
//     m == grid.length
//     n == grid[i].length
//     1 <= m, n, grid[i][j] <= 50
//

struct Solution;

impl Solution {
    pub fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        let (m, n) = (grid.len(), grid[0].len());
        let mut ret = vec![vec![0; n]; m];

        for (i, item) in ret.iter_mut().enumerate().take(m) {
            for (j, inner_item) in item.iter_mut().enumerate().take(n) {
                let (mut s1, mut s2) = (HashSet::new(), HashSet::new());
                let (mut a1, mut b1) = (i, j);
                while a1 > 0 && b1 > 0 {
                    s1.insert(grid[a1 - 1][b1 - 1]);
                    a1 -= 1;
                    b1 -= 1;
                }
                let (mut a2, mut b2) = (i, j);
                while a2 + 1 < m && b2 + 1 < n {
                    s2.insert(grid[a2 + 1][b2 + 1]);
                    a2 += 1;
                    b2 += 1;
                }
                *inner_item = i32::abs(s1.len() as i32 - s2.len() as i32);
            }
        }

        ret
    }
}

#[test]
fn test() {
    let grid = vec![vec![1, 2, 3], vec![3, 1, 5], vec![3, 2, 1]];
    let expect = vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 1, 1]];
    let actual = Solution::difference_of_distinct_values(grid);
    assert_eq!(expect, actual);

    let grid = vec![vec![1]];
    let expect = vec![vec![0]];
    let actual = Solution::difference_of_distinct_values(grid);
    assert_eq!(expect, actual);
}
