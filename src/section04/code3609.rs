#![allow(dead_code)]

// 3609. Minimum Moves to Reach Target in Grid
// https://leetcode.com/problems/minimum-moves-to-reach-target-in-grid/
// https://leetcode.cn/problems/minimum-moves-to-reach-target-in-grid/
//
// Hard
//
// You are given four integers sx, sy, tx, and ty, representing two points (sx, sy)
// and (tx, ty) on an infinitely large 2D grid.
//
// You start at (sx, sy).
//
// At any point (x, y), define m = max(x, y). You can either:
//
//     Move to (x + m, y), or
//     Move to (x, y + m).
//
// Return the minimum number of moves required to reach (tx, ty). If it is impossible to reach the target, return -1.
//
// Example 1:
//
// Input: sx = 1, sy = 2, tx = 5, ty = 4
//
// Output: 2
//
// Explanation:
//
// The optimal path is:
//
//     Move 1: max(1, 2) = 2. Increase the y-coordinate by 2, moving from (1, 2) to (1, 2 + 2) = (1, 4).
//     Move 2: max(1, 4) = 4. Increase the x-coordinate by 4, moving from (1, 4) to (1 + 4, 4) = (5, 4).
//
// Thus, the minimum number of moves to reach (5, 4) is 2.
//
// Example 2:
//
// Input: sx = 0, sy = 1, tx = 2, ty = 3
//
// Output: 3
//
// Explanation:
//
// The optimal path is:
//
//     Move 1: max(0, 1) = 1. Increase the x-coordinate by 1, moving from (0, 1) to (0 + 1, 1) = (1, 1).
//     Move 2: max(1, 1) = 1. Increase the x-coordinate by 1, moving from (1, 1) to (1 + 1, 1) = (2, 1).
//     Move 3: max(2, 1) = 2. Increase the y-coordinate by 2, moving from (2, 1) to (2, 1 + 2) = (2, 3).
//
// Thus, the minimum number of moves to reach (2, 3) is 3.
//
// Example 3:
//
// Input: sx = 1, sy = 1, tx = 2, ty = 2
//
// Output: -1
//
// Explanation:
//
//     It is impossible to reach (2, 2) from (1, 1) using the allowed moves. Thus, the answer is -1.
//
// Constraints:
//
//     0 <= sx <= tx <= 10^9
//     0 <= sy <= ty <= 10^9
//

struct Solution;

impl Solution {
    pub fn min_moves(sx: i32, sy: i32, tx: i32, ty: i32) -> i32 {
        let mut tx = tx;
        let mut ty = ty;
        if sx == 0 && sy == 0 {
            return if tx == 0 && ty == 0 { 0 } else { -1 };
        }

        let mut res = 0;
        let curr_x = sx;
        let curr_y = sy;
        while curr_x != tx || curr_y != ty {
            if curr_x > tx || curr_y > ty {
                return -1;
            }
            res += 1;
            if tx > ty {
                if tx > ty * 2 {
                    if tx % 2 != 0 {
                        return -1;
                    }
                    tx /= 2;
                } else {
                    tx -= ty;
                }
            } else if tx < ty {
                if ty > tx * 2 {
                    if ty % 2 != 0 {
                        return -1;
                    }
                    ty /= 2;
                } else {
                    ty -= tx;
                }
            } else if curr_x == 0 {
                tx = 0;
            } else if curr_y == 0 {
                ty = 0;
            } else {
                return -1;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_moves(1, 2, 5, 4), 2);
    assert_eq!(Solution::min_moves(0, 1, 2, 3), 3);
    assert_eq!(Solution::min_moves(1, 1, 2, 2), -1);
}
