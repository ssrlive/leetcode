#![allow(dead_code)]

// 812. Largest Triangle Area
// https://leetcode.com/problems/largest-triangle-area/
// https://leetcode.cn/problems/largest-triangle-area/
//
// Given an array of points on the X-Y plane points where points[i] = [xi, yi], return the area of the largest triangle that
// can be formed by any three different points. Answers within 10-5 of the actual answer will be accepted.
//
// Example 1:
//
// Input: points = [[0,0],[0,1],[1,0],[0,2],[2,0]]
// Output: 2.00000
// Explanation: The five points are shown in the above figure. The red triangle is the largest.
//
// Example 2:
//
// Input: points = [[1,0],[0,0],[0,1]]
// Output: 0.50000
//
// Constraints:
//
// - 3 <= points.length <= 50
// - -50 <= xi, yi <= 50
// - All the given points are unique.
//

struct Solution;

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        fn _largest_triangle_area(points: Vec<Vec<i32>>) -> Option<f64> {
            let mut max_area = 0.0;
            for i in 0..points.len() {
                for j in i + 1..points.len() {
                    for k in j + 1..points.len() {
                        let area = 0.5
                            * ((points[i][0] * points[j][1] + points[j][0] * points[k][1] + points[k][0] * points[i][1])
                                - (points[i][1] * points[j][0] + points[j][1] * points[k][0] + points[k][1] * points[i][0]))
                                .abs() as f64;
                        if area > max_area {
                            max_area = area;
                        }
                    }
                }
            }
            Some(max_area)
        }

        _largest_triangle_area(points).unwrap_or_default()
    }
}

#[test]
fn test() {
    let points = vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![0, 2], vec![2, 0]];
    let expected = 2.0;
    assert_eq!(Solution::largest_triangle_area(points), expected);

    let points = vec![vec![1, 0], vec![0, 0], vec![0, 1]];
    let expected = 0.5;
    assert_eq!(Solution::largest_triangle_area(points), expected);
}
