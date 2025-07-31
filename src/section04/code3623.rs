#![allow(dead_code)]

// 3623. Count Number of Trapezoids I
// https://leetcode.com/problems/count-number-of-trapezoids-i/
// https://leetcode.cn/problems/count-number-of-trapezoids-i/
//
// Medium
//
// You are given a 2D integer array points, where points[i] = [xi, yi] represents the coordinates of the ith point on the Cartesian plane.
//
// A horizontal trapezoid is a convex quadrilateral with at least one pair of horizontal sides (i.e. parallel to the x-axis).
// Two lines are parallel if and only if they have the same slope.
//
// Return the number of unique horizontal trapezoids that can be formed by choosing any four distinct points from points.
//
// Since the answer may be very large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: points = [[1,0],[2,0],[3,0],[2,2],[3,2]]
//
// Output: 3
//
// Explanation:
//
// There are three distinct ways to pick four points that form a horizontal trapezoid:
//
// Using points [1,0], [2,0], [3,2], and [2,2].
// Using points [2,0], [3,0], [3,2], and [2,2].
// Using points [1,0], [3,0], [3,2], and [2,2].
//
// Example 2:
//
// Input: points = [[0,0],[1,0],[0,1],[2,1]]
//
// Output: 1
//
// Explanation:
//
// There is only one horizontal trapezoid that can be formed.
//
// Constraints:
//
// 4 <= points.length <= 10^5
// –10^8 <= xi, yi <= 10^8
// All points are pairwise distinct.
//

struct Solution;

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut groups = std::collections::HashMap::new();
        for point in points {
            *groups.entry(point[1]).or_insert(0) += 1;
        }
        let mut res = 0_i64;
        let mut total = 0_i64;
        for group in groups {
            let lines = group.1 * (group.1 - 1) / 2;
            res = (res + total * lines) % MOD;
            total = (total + lines) % MOD;
        }
        res as i32
    }
}

#[test]
fn test() {
    let points = vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![2, 2], vec![3, 2]];
    assert_eq!(Solution::count_trapezoids(points), 3);

    let points = vec![vec![0, 0], vec![1, 0], vec![0, 1], vec![2, 1]];
    assert_eq!(Solution::count_trapezoids(points), 1);
}
