#![allow(dead_code)]

// 1401. Circle and Rectangle Overlapping
// https://leetcode.com/problems/circle-and-rectangle-overlapping/
// https://leetcode.cn/problems/circle-and-rectangle-overlapping/
//
// Medium
//
// You are given a circle represented as (radius, xCenter, yCenter) and an axis-aligned rectangle
// represented as (x1, y1, x2, y2), where (x1, y1) are the coordinates of the bottom-left corner,
// and (x2, y2) are the coordinates of the top-right corner of the rectangle.
//
// Return true if the circle and rectangle are overlapped otherwise return false.
// In other words, check if there is any point (xi, yi) that belongs to the circle and the rectangle at the same time.
//
// Example 1:
//
// Input: radius = 1, xCenter = 0, yCenter = 0, x1 = 1, y1 = -1, x2 = 3, y2 = 1
// Output: true
// Explanation: Circle and rectangle share the point (1,0).
//
// Example 2:
//
// Input: radius = 1, xCenter = 1, yCenter = 1, x1 = 1, y1 = -3, x2 = 2, y2 = -1
// Output: false
//
// Example 3:
//
// Input: radius = 1, xCenter = 0, yCenter = 0, x1 = -1, y1 = 0, x2 = 0, y2 = 1
// Output: true
//
// Constraints:
//
// -    1 <= radius <= 2000
// -    -10^4 <= xCenter, yCenter <= 10^4
// -    -10^4 <= x1 < x2 <= 10^4
// -    -10^4 <= y1 < y2 <= 10^4
//

struct Solution;

impl Solution {
    pub fn check_overlap(radius: i32, x_center: i32, y_center: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        let x = if x_center < x1 {
            x1
        } else if x_center > x2 {
            x2
        } else {
            x_center
        };
        let y = if y_center < y1 {
            y1
        } else if y_center > y2 {
            y2
        } else {
            y_center
        };
        let dx = x_center - x;
        let dy = y_center - y;
        dx * dx + dy * dy <= radius * radius
    }
}

#[test]
fn test() {
    assert!(Solution::check_overlap(1, 0, 0, 1, -1, 3, 1));
    assert!(!Solution::check_overlap(1, 1, 1, 1, -3, 2, -1));
    assert!(Solution::check_overlap(1, 0, 0, -1, 0, 0, 1));
}
