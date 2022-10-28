#![allow(dead_code)]

// 223. Rectangle Area
// https://leetcode.com/problems/rectangle-area/
//
// Given the coordinates of two rectilinear rectangles in a 2D plane, return the total area covered by the two rectangles.
//
// The first rectangle is defined by its bottom-left corner (ax1, ay1) and its top-right corner (ax2, ay2).
//
// The second rectangle is defined by its bottom-left corner (bx1, by1) and its top-right corner (bx2, by2).
//
// Example 1:
//
// Input: ax1 = -3, ay1 = 0, ax2 = 3, ay2 = 4, bx1 = 0, by1 = -1, bx2 = 9, by2 = 2
// Output: 45
//
// Example 2:
//
// Input: ax1 = -2, ay1 = -2, ax2 = 2, ay2 = 2, bx1 = -2, by1 = -2, bx2 = 2, by2 = 2
// Output: 16
//
// Constraints:
//
// -10^4 <= ax1, ay1, ax2, ay2, bx1, by1, bx2, by2 <= 10^4

struct Solution;

impl Solution {
    #[allow(clippy::too_many_arguments)]
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        ((ax1 - ax2) * (ay1 - ay2)).abs() + ((bx1 - bx2) * (by1 - by2)).abs()
            - if ax1 >= bx2 || ax2 <= bx1 || ay1 >= by2 || ay2 <= by1 {
                0
            } else {
                (std::cmp::min(ax2, bx2) - std::cmp::max(ax1, bx1))
                    * (std::cmp::min(ay2, by2) - std::cmp::max(ay1, by1))
            }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
    assert_eq!(Solution::compute_area(-2, -2, 2, 2, -2, -2, 2, 2), 16);
}
