#![allow(dead_code)]

// 3001. Minimum Moves to Capture The Queen
// https://leetcode.com/problems/minimum-moves-to-capture-the-queen/
// https://leetcode.cn/problems/minimum-moves-to-capture-the-queen/
//
// Medium
//
// There is a 1-indexed 8 x 8 chessboard containing 3 pieces.
//
// You are given 6 integers a, b, c, d, e, and f where:
//
//     (a, b) denotes the position of the white rook.
//     (c, d) denotes the position of the white bishop.
//     (e, f) denotes the position of the black queen.
//
// Given that you can only move the white pieces, return the minimum number of moves required to capture the black queen.
//
// Note that:
//
//     Rooks can move any number of squares either vertically or horizontally, but cannot jump over other pieces.
//     Bishops can move any number of squares diagonally, but cannot jump over other pieces.
//     A rook or a bishop can capture the queen if it is located in a square that they can move to.
//     The queen does not move.
//
// Example 1:
//
// Input: a = 1, b = 1, c = 8, d = 8, e = 2, f = 3
// Output: 2
// Explanation: We can capture the black queen in two moves by moving the white rook to (1, 3) then to (2, 3).
// It is impossible to capture the black queen in less than two moves since it is not being attacked by any of the pieces at the beginning.
//
// Example 2:
//
// Input: a = 5, b = 3, c = 3, d = 4, e = 5, f = 2
// Output: 1
// Explanation: We can capture the black queen in a single move by doing one of the following:
// - Move the white rook to (5, 2).
// - Move the white bishop to (5, 2).
//
// Constraints:
//
//     1 <= a, b, c, d, e, f <= 8
//     No two pieces are on the same square.
//

struct Solution;

impl Solution {
    pub fn min_moves_to_capture_the_queen(r_y: i32, r_x: i32, b_y: i32, b_x: i32, q_y: i32, q_x: i32) -> i32 {
        let is_d1 = (b_y - q_y == b_x - q_x) && ((r_y - q_y != r_x - q_x) || r_y < b_y.min(q_y) || r_y > b_y.max(q_y));
        let is_d2 = (b_y - q_y == q_x - b_x) && ((r_y - q_y != q_x - r_x) || r_y < b_y.min(q_y) || r_y > b_y.max(q_y));
        let is_c = (r_y == q_y) && ((b_y != r_y) || b_x < r_x.min(q_x) || b_x > r_x.max(q_x));
        let is_r = (r_x == q_x) && ((b_x != r_x) || b_y < r_y.min(q_y) || b_y > r_y.max(q_y));

        if is_d1 || is_d2 || is_c || is_r {
            return 1;
        }
        2
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_moves_to_capture_the_queen(1, 1, 8, 8, 2, 3), 2);
    assert_eq!(Solution::min_moves_to_capture_the_queen(5, 3, 3, 4, 5, 2), 1);
    assert_eq!(Solution::min_moves_to_capture_the_queen(5, 3, 3, 4, 5, 3), 1);
}
