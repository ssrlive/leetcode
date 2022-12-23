#![allow(dead_code)]

// 52. N-Queens II
// https://leetcode.com/problems/n-queens-ii/description/
// https://leetcode.cn/problems/n-queens-ii/description/
//
// The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.
//
// Given an integer n, return the number of distinct solutions to the n-queens puzzle.
//
// Example 1:
//
// Input: n = 4
// Output: 2
// Explanation: There are two distinct solutions to the 4-queens puzzle as shown.
//
// Example 2:
//
// Input: n = 1
// Output: 1
//
// Constraints:
//
// - 1 <= n <= 9
//

struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut v = Vec::new();
        Self::backtrack(n, &mut v)
    }
    fn backtrack(n: i32, v: &mut Vec<i32>) -> i32 {
        if v.len() == n as usize {
            return 1;
        }
        let mut ret = 0;
        for i in 0..n {
            if v.iter()
                .enumerate()
                .any(|(j, &q)| q == i || (v.len() - j) as i32 == (q - i).abs())
            {
                continue;
            }
            v.push(i);
            ret += Self::backtrack(n, v);
            v.pop();
        }
        ret
    }
}

#[test]
fn test() {
    assert_eq!(Solution::total_n_queens(4), 2);
    assert_eq!(Solution::total_n_queens(1), 1);
}
