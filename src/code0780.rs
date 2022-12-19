#![allow(dead_code)]

// 780. Reaching Points
// https://leetcode.com/problems/reaching-points/
// https://leetcode.cn/problems/reaching-points/
//
// Given four integers sx, sy, tx, and ty, return true if it is possible to convert the point (sx, sy)
// to the point (tx, ty) through some operations, or false otherwise.
//
// The allowed operation on some point (x, y) is to convert it to either (x, x + y) or (x + y, y).
//
// Example 1:
//
// Input: sx = 1, sy = 1, tx = 3, ty = 5
// Output: true
// Explanation:
// One series of moves that transforms the starting point to the target is:
// (1, 1) -> (1, 2)
// (1, 2) -> (3, 2)
// (3, 2) -> (3, 5)
//
// Example 2:
//
// Input: sx = 1, sy = 1, tx = 2, ty = 2
// Output: false
//
// Example 3:
//
// Input: sx = 1, sy = 1, tx = 1, ty = 1
// Output: true
//
// Constraints:
//
// - 1 <= sx, sy, tx, ty <= 10^9
//

struct Solution;

impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        let (mut x, mut y) = (tx, ty);

        if y == sy {
            let diff = x - sx;
            return diff >= 0 && diff % sy == 0;
        } else if x == sx {
            let diff = y - sy;
            return diff >= 0 && diff % sx == 0;
        }

        loop {
            if x == sx && y == sy {
                return true;
            } else if x < sx || y < sy {
                return false;
            } else if x > y {
                x -= y;
            } else if x < y {
                y -= x;
            } else if x == y {
                return false;
            } else {
                break;
            }
        }

        false
    }
}

#[test]
fn test() {
    assert_eq!(Solution::reaching_points(1, 1, 3, 5), true);
    assert_eq!(Solution::reaching_points(1, 1, 2, 2), false);
    assert_eq!(Solution::reaching_points(1, 1, 1, 1), true);
    assert_eq!(Solution::reaching_points(1, 1, 1000000000, 1), true);
    assert_eq!(Solution::reaching_points(1, 1, 1, 1000000000), true);
    assert_eq!(Solution::reaching_points(1, 1, 1000000000, 1000000000), false);
    assert_eq!(Solution::reaching_points(9, 5, 12, 8), false);
    assert_eq!(Solution::reaching_points(9, 10, 9, 19), true);
    assert_eq!(Solution::reaching_points(35, 13, 455955547, 420098884), false);
    assert_eq!(Solution::reaching_points(1, 1, 1000000000, 1000000000), false);
    assert_eq!(Solution::reaching_points(1, 1, 1000000000, 1), true);
    assert_eq!(Solution::reaching_points(1, 1, 1, 1000000000), true);
    assert_eq!(Solution::reaching_points(1, 1, 2, 1), true);
    assert_eq!(Solution::reaching_points(1, 1, 1, 2), true);
    assert_eq!(Solution::reaching_points(1, 1, 2, 2), false);
    assert_eq!(Solution::reaching_points(1, 1, 3, 5), true);
    assert_eq!(Solution::reaching_points(1, 1, 5, 3), true);
    assert_eq!(Solution::reaching_points(1, 1, 3, 2), true);
    assert_eq!(Solution::reaching_points(1, 1, 2, 3), true);
    assert_eq!(Solution::reaching_points(1, 1, 4, 5), true);
}
