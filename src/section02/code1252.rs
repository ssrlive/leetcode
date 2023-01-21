#![allow(dead_code)]

// 1252. Cells with Odd Values in a Matrix
// https://leetcode.com/problems/cells-with-odd-values-in-a-matrix/
// https://leetcode.cn/problems/cells-with-odd-values-in-a-matrix/
//
// Easy
//
// There is an m x n matrix that is initialized to all 0's. There is also a 2D array indices where each indices[i] = [ri, ci]
// represents a 0-indexed location to perform some increment operations on the matrix.
//
// For each location indices[i], do both of the following:
//
//     Increment all the cells on row ri.
//     Increment all the cells on column ci.
//
// Given m, n, and indices, return the number of odd-valued cells in the matrix after applying the increment to all locations in indices.
//
// Example 1:
//
// Input: m = 2, n = 3, indices = [[0,1],[1,1]]
// Output: 6
// Explanation: Initial matrix = [[0,0,0],[0,0,0]].
// After applying first increment it becomes [[1,2,1],[0,1,0]].
// The final matrix is [[1,3,1],[1,3,1]], which contains 6 odd numbers.
//
// Example 2:
//
// Input: m = 2, n = 2, indices = [[1,1],[0,0]]
// Output: 0
// Explanation: Final matrix = [[2,2],[2,2]]. There are no odd numbers in the final matrix.
//
// Constraints:
//
// -    1 <= m, n <= 50
// -    1 <= indices.length <= 100
// -    0 <= ri < m
// -    0 <= ci < n
//
// Follow up: Could you solve this in O(n + m + indices.length) time with only O(n + m) extra space?
//

struct Solution;

impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut rows = vec![0; m as usize];
        let mut cols = vec![0; n as usize];
        for index in indices {
            rows[index[0] as usize] += 1;
            cols[index[1] as usize] += 1;
        }
        let mut odd = 0;
        for i in 0..m {
            for j in 0..n {
                if (rows[i as usize] + cols[j as usize]) % 2 == 1 {
                    odd += 1;
                }
            }
        }
        odd
    }
}

#[test]
fn test() {
    assert_eq!(Solution::odd_cells(2, 3, vec![vec![0, 1], vec![1, 1]]), 6);
    assert_eq!(Solution::odd_cells(2, 2, vec![vec![1, 1], vec![0, 0]]), 0);
}
