#![allow(dead_code)]

// 1037. Valid Boomerang
// https://leetcode.com/problems/valid-boomerang/
// https://leetcode.cn/problems/valid-boomerang/
//
// Given an array points where points[i] = [xi, yi] represents a point on the X-Y plane, return true if these points are a boomerang.
//
// A boomerang is a set of three points that are all distinct and not in a straight line.
//
// Example 1:
//
// Input: points = [[1,1],[2,3],[3,2]]
// Output: true
//
// Example 2:
//
// Input: points = [[1,1],[2,2],[3,3]]
// Output: false
//
// Constraints:
//
// - points.length == 3
// - points[i].length == 2
// - 0 <= xi, yi <= 100
//

struct Solution;

impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let (x0, y0) = (points[0][0], points[0][1]);
        let (x1, y1) = (points[1][0], points[1][1]);
        let (x2, y2) = (points[2][0], points[2][1]);

        ((x0 - x1) * (y2 - y1) - (x2 - x1) * (y0 - y1)).abs() > 0
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 1], vec![2, 3], vec![3, 2]], true),
        (vec![vec![1, 1], vec![2, 2], vec![3, 3]], false),
        (vec![vec![0, 0], vec![1, 1], vec![1, 1]], false),
    ];
    for (points, expected) in cases {
        assert_eq!(Solution::is_boomerang(points), expected);
    }
}
