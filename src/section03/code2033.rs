#![allow(dead_code)]

/*

// 2033. Minimum Operations to Make a Uni-Value Grid
// https://leetcode.com/problems/minimum-operations-to-make-a-uni-value-grid/
// https://leetcode.cn/problems/minimum-operations-to-make-a-uni-value-grid/
//
// Medium
//
// You are given a 2D integer grid of size m x n and an integer x. In one operation, you can add x to or subtract x from any element in the grid.

A uni-value grid is a grid where all the elements of it are equal.

Return the minimum number of operations to make the grid uni-value. If it is not possible, return -1.

Example 1:

Input: grid = [[2,4],[6,8]], x = 2
Output: 4
Explanation: We can make every element equal to 4 by doing the following:
- Add x to 2 once.
- Subtract x from 6 once.
- Subtract x from 8 twice.
A total of 4 operations were used.

Example 2:

Input: grid = [[1,5],[2,3]], x = 1
Output: 5
Explanation: We can make every element equal to 3.

Example 3:

Input: grid = [[1,2],[3,4]], x = 2
Output: -1
Explanation: It is impossible to make every element equal.

Constraints:

    m == grid.length
    n == grid[i].length
    1 <= m, n <= 10^5
    1 <= m * n <= 10^5
    1 <= x, grid[i][j] <= 10^4
*/

struct Solution;

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut vec1d: Vec<i32> = grid.into_iter().flatten().collect();
        vec1d.sort_unstable();
        if vec1d.windows(2).all(|w| (w[1] - w[0]) % x == 0) {
            let d = vec1d[vec1d.len() / 2];
            vec1d.iter().fold(0, |sum, v| sum + (d - v).abs() / x)
        } else {
            -1
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![2, 4], vec![6, 8]], 2, 4),
        (vec![vec![1, 5], vec![2, 3]], 1, 5),
        (vec![vec![1, 2], vec![3, 4]], 2, -1),
    ];
    for (grid, x, expected) in cases {
        assert_eq!(Solution::min_operations(grid, x), expected);
    }
}
