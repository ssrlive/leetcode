#![allow(dead_code)]

// 1033. Moving Stones Until Consecutive
// https://leetcode.com/problems/moving-stones-until-consecutive/
// https://leetcode.cn/problems/moving-stones-until-consecutive/
//
// There are three stones in different positions on the X-axis. You are given three integers a, b, and c, the positions of the stones.
//
// In one move, you pick up a stone at an endpoint (i.e., either the lowest or highest position stone), and move it to an unoccupied position between those endpoints. Formally, let's say the stones are currently at positions x, y, and z with x < y < z. You pick up the stone at either position x or position z, and move that stone to an integer position k, with x < k < z and k != y.
//
// The game ends when you cannot make any more moves (i.e., the stones are in three consecutive positions).
//
// Return an integer array answer of length 2 where:
//
// answer[0] is the minimum number of moves you can play, and
// answer[1] is the maximum number of moves you can play.
//
// Example 1:
//
// Input: a = 1, b = 2, c = 5
// Output: [1,2]
// Explanation: Move the stone from 5 to 3, or move the stone from 5 to 4 to 3.
//
// Example 2:
//
// Input: a = 4, b = 3, c = 2
// Output: [0,0]
// Explanation: We cannot make any moves.
//
// Example 3:
//
// Input: a = 3, b = 5, c = 1
// Output: [1,2]
// Explanation: Move the stone from 1 to 4; or move the stone from 1 to 2 to 4.
//
// Constraints:
//
// - 1 <= a, b, c <= 100
// - a, b, and c have different values.
//

struct Solution;

impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        use std::cmp::{max, min};
        let min_ = min(a, min(b, c));
        let max_ = max(a, max(b, c));
        let middle = a + b + c - min_ - max_;
        let d1 = middle - min_ - 1;
        let d2 = max_ - middle - 1;

        let min_ = if d1 == 0 || d2 == 0 {
            min(d1, 1) + min(d2, 1)
        } else if d1 == 1 || d2 == 1 {
            1
        } else {
            2
        };
        vec![min_, d1 + d2]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_moves_stones(1, 2, 5), vec![1, 2]);
    assert_eq!(Solution::num_moves_stones(4, 3, 2), vec![0, 0]);
    assert_eq!(Solution::num_moves_stones(3, 5, 1), vec![1, 2]);
    assert_eq!(Solution::num_moves_stones(1, 2, 3), vec![0, 0]);
}
