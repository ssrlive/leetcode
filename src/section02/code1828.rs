#![allow(dead_code)]

/*

// 1828. Queries on Number of Points Inside a Circle
// https://leetcode.com/problems/queries-on-number-of-points-inside-a-circle/
// https://leetcode.cn/problems/queries-on-number-of-points-inside-a-circle/
//
// Medium
//
// You are given an array points where points[i] = [xi, yi] is the coordinates of the ith point on a 2D plane. Multiple points can have the same coordinates.

You are also given an array queries where queries[j] = [xj, yj, rj] describes a circle centered at (xj, yj) with a radius of rj.

For each query queries[j], compute the number of points inside the jth circle. Points on the border of the circle are considered inside.

Return an array answer, where answer[j] is the answer to the jth query.

Example 1:

Input: points = [[1,3],[3,3],[5,3],[2,2]], queries = [[2,3,1],[4,3,1],[1,1,2]]
Output: [3,2,2]
Explanation: The points and circles are shown above.
queries[0] is the green circle, queries[1] is the red circle, and queries[2] is the blue circle.

Example 2:

Input: points = [[1,1],[2,2],[3,3],[4,4],[5,5]], queries = [[1,2,2],[2,2,2],[4,3,2],[4,3,3]]
Output: [2,3,2,4]
Explanation: The points and circles are shown above.
queries[0] is green, queries[1] is red, queries[2] is blue, and queries[3] is purple.

Constraints:

    1 <= points.length <= 500
    points[i].length == 2
    0 <= x​​​​​​i, y​​​​​​i <= 500
    1 <= queries.length <= 500
    queries[j].length == 3
    0 <= xj, yj <= 500
    1 <= rj <= 500
    All coordinates are integers.

Follow up: Could you find the answer for each query in better complexity than O(n)?
*/

struct Solution;

impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        fn dist(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
            (x1 - x2).pow(2) + (y1 - y2).pow(2)
        }
        queries
            .iter()
            .map(|q| {
                let q0 = q[0];
                let q1 = q[1];
                let q2 = q[2].pow(2);
                points.iter().filter(|p| q2 >= dist(q0, q1, p[0], p[1])).count() as _
            })
            .collect()
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![1, 3], vec![3, 3], vec![5, 3], vec![2, 2]],
            vec![vec![2, 3, 1], vec![4, 3, 1], vec![1, 1, 2]],
            vec![3, 2, 2],
        ),
        (
            vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]],
            vec![vec![1, 2, 2], vec![2, 2, 2], vec![4, 3, 2], vec![4, 3, 3]],
            vec![2, 3, 2, 4],
        ),
    ];
    for (points, queries, expected) in cases {
        assert_eq!(Solution::count_points(points, queries), expected);
    }
}
