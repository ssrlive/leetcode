#![allow(dead_code)]

// 2550. Count Collisions of Monkeys on a Polygon
// https://leetcode.com/problems/count-collisions-of-monkeys-on-a-polygon/
// https://leetcode.cn/problems/count-collisions-of-monkeys-on-a-polygon/
//
// Medium
//
// There is a regular convex polygon with n vertices. The vertices are labeled from 0 to n - 1 in a clockwise direction,
// and each vertex has exactly one monkey. The following figure shows a convex polygon of 6 vertices.
//
// Each monkey moves simultaneously to a neighboring vertex. A neighboring vertex for a vertex i can be:
//
//     the vertex (i + 1) % n in the clockwise direction, or
//     the vertex (i - 1 + n) % n in the counter-clockwise direction.
//
// A collision happens if at least two monkeys reside on the same vertex after the movement.
//
// Return the number of ways the monkeys can move so that at least one collision happens.
// Since the answer may be very large, return it modulo 109 + 7.
//
// Note that each monkey can only move once.
//
// Example 1:
//
// Input: n = 3
// Output: 6
// Explanation: There are 8 total possible movements.
// Two ways such that they collide at some point are:
// - Monkey 1 moves in a clockwise direction; monkey 2 moves in an anticlockwise direction;
//   monkey 3 moves in a clockwise direction. Monkeys 1 and 2 collide.
// - Monkey 1 moves in an anticlockwise direction; monkey 2 moves in an anticlockwise direction;
//   monkey 3 moves in a clockwise direction. Monkeys 1 and 3 collide.
// It can be shown 6 total movements result in a collision.
//
// Example 2:
//
// Input: n = 4
// Output: 14
// Explanation: It can be shown that there are 14 ways for the monkeys to collide.
//
// Constraints:
//
// -    3 <= n <= 10^9
//

struct Solution;

impl Solution {
    pub fn monkey_move(n: i32) -> i32 {
        let mut res = 1;
        let mut base = 2;
        let modu = 1_000_000_007_i64;
        let mut n = n;
        while n > 0 {
            if n % 2 == 1 {
                res = res * base % modu;
            }
            base = base * base % modu;
            n >>= 1;
        }
        ((res - 2 + modu) % modu) as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::monkey_move(3), 6);
    assert_eq!(Solution::monkey_move(4), 14);
    assert_eq!(Solution::monkey_move(5), 30);
    assert_eq!(Solution::monkey_move(6), 62);
}
