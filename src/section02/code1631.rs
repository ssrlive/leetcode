#![allow(dead_code)]

/*

// 1631. Path With Minimum Effort
// https://leetcode.com/problems/path-with-minimum-effort/
// https://leetcode.cn/problems/path-with-minimum-effort/
//
// Medium
//
// You are a hiker preparing for an upcoming hike. You are given heights, a 2D array of size rows x columns, where heights[row][col] represents the height of cell (row, col). You are situated in the top-left cell, (0, 0), and you hope to travel to the bottom-right cell, (rows-1, columns-1) (i.e., 0-indexed). You can move up, down, left, or right, and you wish to find a route that requires the minimum effort.

A route's effort is the maximum absolute difference in heights between two consecutive cells of the route.

Return the minimum effort required to travel from the top-left cell to the bottom-right cell.

Example 1:

Input: heights = [[1,2,2],[3,8,2],[5,3,5]]
Output: 2
Explanation: The route of [1,3,5,3,5] has a maximum absolute difference of 2 in consecutive cells.
This is better than the route of [1,2,2,2,5], where the maximum absolute difference is 3.

Example 2:

Input: heights = [[1,2,3],[3,8,4],[5,3,5]]
Output: 1
Explanation: The route of [1,2,3,4,5] has a maximum absolute difference of 1 in consecutive cells, which is better than route [1,3,5,3,5].

Example 3:

Input: heights = [[1,2,1,1,1],[1,2,1,2,1],[1,2,1,2,1],[1,2,1,2,1],[1,1,1,2,1]]
Output: 0
Explanation: This route does not require any effort.

Constraints:

    rows == heights.length
    columns == heights[i].length
    1 <= rows, columns <= 100
    1 <= heights[i][j] <= 10^6
*/

struct Solution;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let (r, c) = (heights.len(), heights[0].len());
        let mut min_effort = vec![vec![None; c]; r];
        let mut bh = BinaryHeap::new();
        bh.push((Reverse(0), (0, 0)));
        while let Some((Reverse(min), (i, j))) = bh.pop() {
            if i == r - 1 && j == c - 1 {
                return min;
            }
            if min_effort[i][j].is_some() {
                continue;
            } else {
                min_effort[i][j] = Some(min);
            }
            let height = heights[i][j];
            for (i, j) in [(-1, 0), (0, -1), (1, 0), (0, 1)].iter().filter_map(|(di, dj)| {
                let (ii, jj) = (i as i32 + di, j as i32 + dj);
                if (0..r as i32).contains(&ii) && (0..c as i32).contains(&jj) {
                    Some((ii as usize, jj as usize))
                } else {
                    None
                }
            }) {
                bh.push((Reverse(min.max((heights[i][j] - height).abs())), (i, j)));
            }
        }
        unreachable!();
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]], 2),
        (vec![vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5]], 1),
        (
            vec![
                vec![1, 2, 1, 1, 1],
                vec![1, 2, 1, 2, 1],
                vec![1, 2, 1, 2, 1],
                vec![1, 2, 1, 2, 1],
                vec![1, 1, 1, 2, 1],
            ],
            0,
        ),
    ];
    for (heights, expected) in cases {
        assert_eq!(Solution::minimum_effort_path(heights), expected);
    }
}
