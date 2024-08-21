#![allow(dead_code)]

// 3257. Maximum Value Sum by Placing Three Rooks II
// https://leetcode.com/problems/maximum-value-sum-by-placing-three-rooks-ii/
// https://leetcode.cn/problems/maximum-value-sum-by-placing-three-rooks-ii/
//
// Hard
//
// You are given a m x n 2D array board representing a chessboard,
// where board[i][j] represents the value of the cell (i, j).
//
// Rooks in the same row or column attack each other. You need to place three
// rooks on the chessboard such that the rooks do not attack each other.
//
// Return the maximum sum of the cell values on which the rooks are placed.
//
// Example 1:
//
// Input: board = [[-3,1,1,1],[-3,1,-3,1],[-3,2,1,1]]
//
// Output: 4
//
// Explanation:
//
// We can place the rooks in the cells (0, 2), (1, 3), and (2, 1) for a sum of 1 + 1 + 2 = 4.
//
// Example 2:
//
// Input: board = [[1,2,3],[4,5,6],[7,8,9]]
//
// Output: 15
//
// Explanation:
//
// We can place the rooks in the cells (0, 0), (1, 1), and (2, 2) for a sum of 1 + 5 + 9 = 15.
//
// Example 3:
//
// Input: board = [[1,1,1],[1,1,1],[1,1,1]]
//
// Output: 3
//
// Explanation:
//
// We can place the rooks in the cells (0, 2), (1, 1), and (2, 0) for a sum of 1 + 1 + 1 = 3.
//
// Constraints:
//
//     3 <= m == board.length <= 500
//     3 <= n == board[i].length <= 500
//     -10^9 <= board[i][j] <= 10^9
//

struct Solution;

impl Solution {
    pub fn maximum_value_sum(board: Vec<Vec<i32>>) -> i64 {
        let n = board.len();
        let m = board[0].len();

        let mut idx = vec![vec![-1; 3]; n];
        for i in 0..n {
            for j in 0..m {
                for k in 0..3 {
                    if idx[i][k] == -1 || board[i][idx[i][k] as usize] <= board[i][j] {
                        let mut l = 2;
                        while l > k {
                            idx[i][l] = idx[i][l - 1];
                            l -= 1;
                        }
                        idx[i][k] = j as i32;
                        break;
                    }
                }
            }
        }

        let mut ans = i64::MIN;
        for i in 0..n - 2 {
            for &k1 in &idx[i] {
                for j in (i + 1)..(n - 1) {
                    for &k2 in &idx[j] {
                        if k2 != k1 {
                            for l in (j + 1)..n {
                                for &k3 in &idx[l] {
                                    if k3 != k1 && k3 != k2 {
                                        ans = ans.max(
                                            board[i][k1 as usize] as i64 + board[j][k2 as usize] as i64 + board[l][k3 as usize] as i64,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        ans
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::maximum_value_sum(vec![vec![-3, 1, 1, 1], vec![-3, 1, -3, 1], vec![-3, 2, 1, 1]]),
        4
    );
    assert_eq!(Solution::maximum_value_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), 15);
    assert_eq!(Solution::maximum_value_sum(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]]), 3);
}
