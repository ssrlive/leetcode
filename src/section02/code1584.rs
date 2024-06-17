#![allow(dead_code)]

/*

// 1584. Min Cost to Connect All Points
Medium
3.4K
83
Companies

You are given an array points representing integer coordinates of some points on a 2D-plane, where points[i] = [xi, yi].

The cost of connecting two points [xi, yi] and [xj, yj] is the manhattan distance between them: |xi - xj| + |yi - yj|, where |val| denotes the absolute value of val.

Return the minimum cost to make all points connected. All points are connected if there is exactly one simple path between any two points.

Example 1:

Input: points = [[0,0],[2,2],[3,10],[5,2],[7,0]]
Output: 20
Explanation:

We can connect the points as shown above to get the minimum cost of 20.
Notice that there is a unique path between every pair of points.

Example 2:

Input: points = [[3,12],[-2,5],[-4,1]]
Output: 18

Constraints:

    1 <= points.length <= 1000
    -106 <= xi, yi <= 106
    All pairs (xi, yi) are distinct.
*/

struct Solution;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        use std::{cmp::Reverse, collections::BinaryHeap};
        let n = points.len();
        let mut dist = vec![vec![i32::MAX; n]; n];
        for i in 0..n {
            for j in i + 1..n {
                let d = (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs();
                dist[i][j] = d;
                dist[j][i] = d;
            }
        }
        let mut ans = 0;
        let mut visited = vec![false; n];
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, 0)));
        while let Some(Reverse((d, i))) = heap.pop() {
            if visited[i] {
                continue;
            }
            visited[i] = true;
            ans += d;
            for (j, &item) in visited.iter().enumerate() {
                if !item {
                    heap.push(Reverse((dist[i][j], j)));
                }
            }
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]], 20),
        (vec![vec![3, 12], vec![-2, 5], vec![-4, 1]], 18),
    ];
    for (points, expected) in cases {
        assert_eq!(Solution::min_cost_connect_points(points), expected);
    }
}
