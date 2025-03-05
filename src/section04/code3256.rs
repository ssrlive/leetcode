#![allow(dead_code)]

// 3256. Maximum Value Sum by Placing Three Rooks I
// https://leetcode.com/problems/maximum-value-sum-by-placing-three-rooks-i/
// https://leetcode.cn/problems/maximum-value-sum-by-placing-three-rooks-i/
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
//     3 <= m == board.length <= 100
//     3 <= n == board[i].length <= 100
//     -10^9 <= board[i][j] <= 10^9
//

struct Solution;

impl Solution {
    pub fn maximum_value_sum(board: Vec<Vec<i32>>) -> i64 {
        let (m, _n) = (board.len(), board[0].len());
        let mut max_three = Vec::new();
        for row in &board {
            let mut rec = ((i32::MIN, 0), (i32::MIN, 0), (i32::MIN, 0));
            for (i, &n) in row.iter().enumerate() {
                if n > rec.0.0 {
                    rec.2 = rec.1;
                    rec.1 = rec.0;
                    rec.0 = (n, i);
                } else if n > rec.1.0 {
                    rec.2 = rec.1;
                    rec.1 = (n, i);
                } else if n > rec.2.0 {
                    rec.2 = (n, i);
                }
            }
            max_three.push(rec);
        }

        let mut res = i64::MIN;
        for i in 0..m {
            let aa = &max_three[i].0;
            for j in 0..m {
                if i == j {
                    continue;
                }
                let tmp = &max_three[j];
                let bb = if tmp.0.1 != aa.1 { &tmp.0 } else { &tmp.1 };
                for (k, max_three_k) in max_three.iter().enumerate().take(m) {
                    if k == i || k == j {
                        continue;
                    }
                    let tmp = &max_three_k;
                    let cc = if tmp.0.1 != aa.1 && tmp.0.1 != bb.1 {
                        &tmp.0
                    } else if tmp.1.1 != aa.1 && tmp.1.1 != bb.1 {
                        &tmp.1
                    } else {
                        &tmp.2
                    };
                    res = res.max(aa.0 as i64 + bb.0 as i64 + cc.0 as i64);
                }
            }
        }
        res
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
