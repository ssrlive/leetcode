#![allow(dead_code)]

// 51. N-Queens
// https://leetcode.com/problems/n-queens/
// https://leetcode.cn/problems/n-queens/
//
// The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.
//
// Given an integer n, return all distinct solutions to the n-queens puzzle. You may return the answer in any order.
//
// Each solution contains a distinct board configuration of the n-queens' placement,
// where 'Q' and '.' both indicate a queen and an empty space, respectively.
//
// Example 1:
//
// Input: n = 4
// Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
// Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above
//
// Example 2:
//
// Input: n = 1
// Output: [["Q"]]
//
// Constraints:
//
// - 1 <= n <= 9
//

struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        // create all possible first rows
        let mut level = Vec::new();
        for i in 0..n {
            level.push(vec![i]);
        }
        // for each additional row append and each board append a queen in each possible position
        // note we need only check the new queen does not offend the existing queens
        for _ in 1..n {
            let mut new_level = Vec::new();
            for v in &level {
                for j in 0..n {
                    if !v.contains(&j) && v.iter().rev().enumerate().all(|(i, x)| (x - j).abs() != (1 + i as i32)) {
                        new_level.push(v.iter().copied().chain(vec![j]).collect());
                    }
                }
            }
            level = new_level.to_vec();
        }
        // translate formats
        level
            .iter()
            .map(|board| {
                board
                    .iter()
                    .map(|row| format!("{}Q{}", ".".repeat(*row as usize), ".".repeat((n - *row - 1) as usize)))
                    .collect()
            })
            .collect()
    }
}

#[test]
fn test() {
    let n = 4;
    let output = Solution::solve_n_queens(n);
    let expected = vec![vec![".Q..", "...Q", "Q...", "..Q."], vec!["..Q.", "Q...", "...Q", ".Q.."]];
    assert_eq!(output, expected);

    let n = 1;
    let output = Solution::solve_n_queens(n);
    let expected = vec![vec!["Q"]];
    assert_eq!(output, expected);
}
