#![allow(dead_code)]

// 2543. Check if Point Is Reachable
// https://leetcode.com/problems/check-if-point-is-reachable/
// https://leetcode.cn/problems/check-if-point-is-reachable/
//
// Hard
//
// There exists an infinitely large grid. You are currently at point (1, 1),
// and you need to reach the point (targetX, targetY) using a finite number of steps.
//
// In one step, you can move from point (x, y) to any one of the following points:
//
// (x, y - x)
// (x - y, y)
// (2 * x, y)
// (x, 2 * y)
//
// Given two integers targetX and targetY representing the X-coordinate and Y-coordinate of your final position,
// return true if you can reach the point from (1, 1) using some number of steps, and false otherwise.
//
// Example 1:
//
// Input: targetX = 6, targetY = 9
// Output: false
// Explanation: It is impossible to reach (6,9) from (1,1) using any sequence of moves, so false is returned.
//
// Example 2:
//
// Input: targetX = 4, targetY = 7
// Output: true
// Explanation: You can follow the path (1,1) -> (1,2) -> (1,4) -> (1,8) -> (1,7) -> (2,7) -> (4,7).
//
// Constraints:
//
// 1 <= targetX, targetY <= 10^9
//

struct Solution;

impl Solution {
    pub fn is_reachable(target_x: i32, target_y: i32) -> bool {
        fn gcd(a: i32, b: i32) -> i32 {
            if a < b {
                return gcd(b, a);
            }
            if a % b == 0 {
                return b;
            }
            gcd(b, a % b)
        }

        let (mut target_x, mut target_y) = (target_x, target_y);
        while target_x % 2 == 0 && target_y % 2 == 0 {
            target_x >>= 1;
            target_y >>= 1;
        }

        gcd(target_x, target_y) == 1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_reachable(6, 9), false);
    assert_eq!(Solution::is_reachable(4, 7), true);
    assert_eq!(Solution::is_reachable(1, 1), true);
    assert_eq!(Solution::is_reachable(1, 2), true);
    assert_eq!(Solution::is_reachable(1, 3), true);
}
