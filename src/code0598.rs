#![allow(dead_code)]

// 598. Range Addition II
// https://leetcode.com/problems/range-addition-ii/
// https://leetcode.cn/problems/range-addition-ii/
//
// You are given an m x n matrix M initialized with all 0's and an array of operations ops, where ops[i] = [ai, bi] means M[x][y] should be incremented by one for all 0 <= x < ai and 0 <= y < bi.
//
// Count and return the number of maximum integers in the matrix after performing all the operations.
//
// Example 1:
//
// Input: m = 3, n = 3, ops = [[2,2],[3,3]]
// Output: 4
// Explanation: The maximum integer in M is 2, and there are four of it in M. So return 4.
//
// Example 2:
//
// Input: m = 3, n = 3, ops = [[2,2],[3,3],[3,3],[3,3],[2,2],[3,3],[3,3],[3,3],[2,2],[3,3],[3,3],[3,3]]
// Output: 4
//
// Example 3:
//
// Input: m = 3, n = 3, ops = []
// Output: 9
//
// Constraints:
//
// - 1 <= m, n <= 4 * 10^4
// - 0 <= ops.length <= 10^4
// - ops[i].length == 2
// - 1 <= ai <= m
// - 1 <= bi <= n
//

struct Solution;

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let mut min_a = m;
        let mut min_b = n;
        for op in ops {
            min_a = min_a.min(op[0]);
            min_b = min_b.min(op[1]);
        }
        min_a * min_b
    }
}

#[test]
fn test_max_count() {
    assert_eq!(Solution::max_count(3, 3, vec![vec![2, 2], vec![3, 3]]), 4);
    assert_eq!(
        Solution::max_count(
            3,
            3,
            vec![
                vec![2, 2],
                vec![3, 3],
                vec![3, 3],
                vec![3, 3],
                vec![2, 2],
                vec![3, 3],
                vec![3, 3],
                vec![3, 3],
                vec![2, 2],
                vec![3, 3],
                vec![3, 3],
                vec![3, 3]
            ]
        ),
        4
    );
    assert_eq!(Solution::max_count(3, 3, vec![]), 9);
}
