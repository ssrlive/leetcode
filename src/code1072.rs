#![allow(dead_code)]

// 1072. Flip Columns For Maximum Number of Equal Rows
// https://leetcode.com/problems/flip-columns-for-maximum-number-of-equal-rows/
// https://leetcode.cn/problems/flip-columns-for-maximum-number-of-equal-rows/
//
// You are given an m x n binary matrix matrix.
//
// You can choose any number of columns in the matrix and flip every cell in that column (i.e., Change the value of the cell from 0 to 1 or vice versa).
//
// Return the maximum number of rows that have all values equal after some number of flips.
//
// Example 1:
//
// Input: matrix = [[0,1],[1,1]]
// Output: 1
// Explanation: After flipping no values, 1 row has all values equal.
//
// Example 2:
//
// Input: matrix = [[0,1],[1,0]]
// Output: 2
// Explanation: After flipping values in the first column, both rows have equal values.
//
// Example 3:
//
// Input: matrix = [[0,0,0],[0,0,1],[1,1,0]]
// Output: 2
// Explanation: After flipping values in the first two columns, the last two rows have equal values.
//
// Constraints:
//
// - m == matrix.length
// - n == matrix[i].length
// - 1 <= m, n <= 300
// - matrix[i][j] is either 0 or 1.
//

struct Solution;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let mut mymap = HashMap::new();
        for row in matrix {
            let mut row1 = row.clone();
            for num in &mut row1 {
                *num = 1 - *num;
            }
            let number = mymap.entry(row).or_insert(0);
            *number += 1;
            let number1 = mymap.entry(row1).or_insert(0);
            *number1 += 1;
        }
        let mut max: i32 = 0;
        for count in mymap.values() {
            if *count > max {
                max = *count;
            }
        }

        max
    }
}

#[test]
fn test() {
    let matrix = vec![vec![0, 1], vec![1, 1]];
    let result = Solution::max_equal_rows_after_flips(matrix);
    assert_eq!(result, 1);

    let matrix = vec![vec![0, 1], vec![1, 0]];
    let result = Solution::max_equal_rows_after_flips(matrix);
    assert_eq!(result, 2);

    let matrix = vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]];
    let result = Solution::max_equal_rows_after_flips(matrix);
    assert_eq!(result, 2);
}
