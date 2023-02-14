#![allow(dead_code)]

/*

// 1637. Widest Vertical Area Between Two Points Containing No Points
// https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/
// https://leetcode.cn/problems/widest-vertical-area-between-two-points-containing-no-points/
//
// Medium
//
// Given n points on a 2D plane where points[i] = [xi, yi], Return the widest vertical area between two points such that no points are inside the area.

A vertical area is an area of fixed-width extending infinitely along the y-axis (i.e., infinite height). The widest vertical area is the one with the maximum width.

Note that points on the edge of a vertical area are not considered included in the area.

Example 1:
​

Input: points = [[8,7],[9,9],[7,4],[9,7]]
Output: 1
Explanation: Both the red and the blue area are optimal.

Example 2:

Input: points = [[3,1],[9,0],[1,0],[1,4],[5,3],[8,8]]
Output: 3

Constraints:

    n == points.length
    2 <= n <= 10^5
    points[i].length == 2
    0 <= xi, yi <= 10^9
*/

struct Solution;

impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_unstable_by_key(|v| v[0]);
        let mut max = 0;
        for i in 1..points.len() {
            max = max.max(points[i][0] - points[i - 1][0]);
        }
        max
    }
}

#[test]
fn test() {
    let points = vec![vec![8, 7], vec![9, 9], vec![7, 4], vec![9, 7]];
    assert_eq!(Solution::max_width_of_vertical_area(points), 1);
    let points = vec![vec![3, 1], vec![9, 0], vec![1, 0], vec![1, 4], vec![5, 3], vec![8, 8]];
    assert_eq!(Solution::max_width_of_vertical_area(points), 3);
}
