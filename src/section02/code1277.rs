#![allow(dead_code)]

// 1277. Count Square Submatrices with All Ones
// https://leetcode.com/problems/count-square-submatrices-with-all-ones/
// https://leetcode.cn/problems/count-square-submatrices-with-all-ones/
//
// Medium
//
// Given a m * n matrix of ones and zeros, return how many square submatrices have all ones.
//
// Example 1:
//
// Input: matrix =
// [
//   [0,1,1,1],
//   [1,1,1,1],
//   [0,1,1,1]
// ]
// Output: 15
// Explanation:
// There are 10 squares of side 1.
// There are 4 squares of side 2.
// There is  1 square of side 3.
// Total number of squares = 10 + 4 + 1 = 15.
//
// Example 2:
//
// Input: matrix =
// [
//   [1,0,1],
//   [1,1,0],
//   [1,1,0]
// ]
// Output: 7
// Explanation:
// There are 6 squares of side 1.
// There is 1 square of side 2.
// Total number of squares = 6 + 1 = 7.
//
// Constraints:
//
// -    1 <= arr.length <= 300
// -    1 <= arr[0].length <= 300
// -    0 <= arr[i][j] <= 1
//

struct Solution;

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0; matrix[0].len()]; matrix.len()];
        let mut sum = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 1 {
                    if i == 0 || j == 0 {
                        dp[i][j] = 1;
                    } else {
                        dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i - 1][j].min(dp[i][j - 1]));
                    }
                    sum += dp[i][j];
                }
            }
        }
        sum
    }
}

#[test]
fn test() {
    let matrix = vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]];
    assert_eq!(Solution::count_squares(matrix), 15);
    let matrix = vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]];
    assert_eq!(Solution::count_squares(matrix), 7);
}
