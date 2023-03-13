#![allow(dead_code)]

/*

// 1572. Matrix Diagonal Sum
Easy
1.9K
26
Companies

Given a square matrix mat, return the sum of the matrix diagonals.

Only include the sum of all the elements on the primary diagonal and all the elements on the secondary diagonal that are not part of the primary diagonal.

Example 1:

Input: mat = [[1,2,3],
              [4,5,6],
              [7,8,9]]
Output: 25
Explanation: Diagonals sum: 1 + 5 + 9 + 3 + 7 = 25
Notice that element mat[1][1] = 5 is counted only once.

Example 2:

Input: mat = [[1,1,1,1],
              [1,1,1,1],
              [1,1,1,1],
              [1,1,1,1]]
Output: 8

Example 3:

Input: mat = [[5]]
Output: 5

Constraints:

    n == mat.length == mat[i].length
    1 <= n <= 100
    1 <= mat[i][j] <= 100
*/

struct Solution;

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut sum = 0;
        let n = mat.len();
        for (i, item) in mat.iter().enumerate() {
            sum += item[i];
            sum += item[n - i - 1];
        }
        if n % 2 == 1 {
            sum -= mat[n / 2][n / 2];
        }
        sum
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 25),
        (
            vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 1]],
            8,
        ),
        (vec![vec![5]], 5),
    ];
    for (mat, expected) in cases {
        assert_eq!(Solution::diagonal_sum(mat), expected);
    }
}
