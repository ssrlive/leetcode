#![allow(dead_code)]

/*

// 2133. Check if Every Row and Column Contains All Numbers
// https://leetcode.com/problems/check-if-every-row-and-column-contains-all-numbers/
// https://leetcode.cn/problems/check-if-every-row-and-column-contains-all-numbers/
//
// Easy
//
// An n x n matrix is valid if every row and every column contains all the integers from 1 to n (inclusive).

Given an n x n integer matrix matrix, return true if the matrix is valid. Otherwise, return false.

Example 1:

Input: matrix = [[1,2,3],[3,1,2],[2,3,1]]
Output: true
Explanation: In this case, n = 3, and every row and column contains the numbers 1, 2, and 3.
Hence, we return true.

Example 2:

Input: matrix = [[1,1,1],[1,2,3],[1,2,3]]
Output: false
Explanation: In this case, n = 3, but the first row and the first column do not contain the numbers 2 or 3.
Hence, we return false.

Constraints:

    n == matrix.length == matrix[i].length
    1 <= n <= 100
    1 <= matrix[i][j] <= n
*/

struct Solution;

impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        use std::cmp::Ordering;
        let n = matrix.len();
        for row in 0..n {
            let (mut count, mut count_col) = ([0; 100], [0; 100]);
            for col in 0..n {
                count[matrix[row][col] as usize - 1] += 1;
                count_col[matrix[col][row] as usize - 1] += 1;
            }
            for i in 0..n {
                match (count[i].cmp(&1), count_col[i].cmp(&1)) {
                    (Ordering::Equal, Ordering::Equal) => {}
                    _ => {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 2, 3], vec![3, 1, 2], vec![2, 3, 1]], true),
        (vec![vec![1, 1, 1], vec![1, 2, 3], vec![1, 2, 3]], false),
    ];
    for (matrix, expected) in cases {
        assert_eq!(Solution::check_valid(matrix), expected);
    }
}
