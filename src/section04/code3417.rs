#![allow(dead_code)]

// 3417. Zigzag Grid Traversal With Skip
// https://leetcode.com/problems/zigzag-grid-traversal-with-skip/
// https://leetcode.cn/problems/zigzag-grid-traversal-with-skip/
//
// Easy
//
// You are given an m x n 2D array grid of positive integers.
//
// Your task is to traverse grid in a zigzag pattern while skipping every alternate cell.
//
// Zigzag pattern traversal is defined as following the below actions:
//
//     Start at the top-left cell (0, 0).
//     Move right within a row until the end of the row is reached.
//     Drop down to the next row, then traverse left until the beginning of the row is reached.
//     Continue alternating between right and left traversal until every row has been traversed.
//
// Note that you must skip every alternate cell during the traversal.
//
// Return an array of integers result containing, in order, the value of the cells visited during the zigzag traversal with skips.
//
// Example 1:
//
// Input: grid = [[1,2],[3,4]]
//
// Output: [1,4]
//
// Explanation:
//
// Example 2:
//
// Input: grid = [[2,1],[2,1],[2,1]]
//
// Output: [2,1,2]
//
// Explanation:
//
// Example 3:
//
// Input: grid = [[1,2,3],[4,5,6],[7,8,9]]
//
// Output: [1,3,5,7,9]
//
// Explanation:
//
// Constraints:
//
//     2 <= n == grid.length <= 50
//     2 <= m == grid[i].length <= 50
//     1 <= grid[i][j] <= 2500
//

struct Solution;

impl Solution {
    pub fn zigzag_traversal(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::new();
        for (i, r) in grid.iter().enumerate() {
            if i % 2 == 0 {
                res.extend_from_slice(r);
            } else {
                res.extend_from_slice(&r.iter().rev().cloned().collect::<Vec<i32>>());
            }
        }
        res.iter().step_by(2).cloned().collect()
    }
}

#[test]
fn test() {
    let grid = vec![vec![1, 2], vec![3, 4]];
    let res = vec![1, 4];
    assert_eq!(Solution::zigzag_traversal(grid), res);

    let grid = vec![vec![2, 1], vec![2, 1], vec![2, 1]];
    let res = vec![2, 1, 2];
    assert_eq!(Solution::zigzag_traversal(grid), res);

    let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let res = vec![1, 3, 5, 7, 9];
    assert_eq!(Solution::zigzag_traversal(grid), res);
}
