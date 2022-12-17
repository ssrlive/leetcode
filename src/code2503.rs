#![allow(dead_code)]

// 2503. Maximum Number of Points From Grid Queries
// https://leetcode.com/problems/maximum-number-of-points-from-grid-queries/
//
// You are given an m x n integer matrix grid and an array queries of size k.
//
// Find an array answer of size k such that for each integer queres[i] you start in the top left cell of the matrix and repeat the following process:
//
// - If queries[i] is strictly greater than the value of the current cell that you are in, then you get one point if it is your first time visiting this cell,
//   and you can move to any adjacent cell in all 4 directions: up, down, left, and right.
// - Otherwise, you do not get any points, and you end this process.
//
// After the process, answer[i] is the maximum number of points you can get. Note that for each query you are allowed to visit the same cell multiple times.
//
// Return the resulting array answer.
//
// Example 1:
//
// Input: grid = [[1,2,3],[2,5,7],[3,5,1]], queries = [5,6,2]
// Output: [5,8,1]
// Explanation: The diagrams above show which cells we visit to get points for each query.
//
// Example 2:
//
// Input: grid = [[5,2,1],[1,1,2]], queries = [3]
// Output: [0]
// Explanation: We can not get any points because the value of the top left cell is already greater than or equal to 3.
//
// Constraints:
//
// - m == grid.length
// - n == grid[i].length
// - 2 <= m, n <= 1000
// - 4 <= m * n <= 10^5
// - k == queries.length
// - 1 <= k <= 10^4
// - 1 <= grid[i][j], queries[i] <= 10^6
//

struct Solution;

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut data = vec![];
        for (i, &item) in queries.iter().enumerate() {
            data.push((item, i));
        }
        data.sort();

        let mut ret = vec![0; data.len()];
        let (m, n) = (grid.len(), grid[0].len());
        let mut flag = vec![vec![0; n]; m];
        let mut pq = BinaryHeap::from([Reverse((grid[0][0], 0, 0))]);
        let mut count = 0;
        let dirs = [[1, 0], [0, 1], [-1, 0], [0, -1]];
        flag[0][0] = 1;

        for q in data {
            while let Some(Reverse((val, u, v))) = pq.peek() {
                if *val >= q.0 {
                    break;
                }
                let (u, v) = (*u as i32, *v as i32);
                count += 1;
                pq.pop();

                for d in dirs {
                    let (i, j) = (u + d[0], v + d[1]);
                    if i < 0 || i == m as i32 || j < 0 || j == n as i32 {
                        continue;
                    }

                    let (i, j) = (i as usize, j as usize);
                    if flag[i][j] == 1 {
                        continue;
                    }

                    flag[i][j] = 1;
                    pq.push(Reverse((grid[i][j], i, j)));
                }
            }

            ret[q.1] = count;
        }

        ret
    }
}

#[test]
fn test() {
    let grid = vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]];
    let queries = vec![5, 6, 2];
    let res = vec![5, 8, 1];
    assert_eq!(Solution::max_points(grid, queries), res);

    let grid = vec![vec![5, 2, 1], vec![1, 1, 2]];
    let queries = vec![3];
    let res = vec![0];
    assert_eq!(Solution::max_points(grid, queries), res);
}
