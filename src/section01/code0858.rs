#![allow(dead_code)]

// 858. Mirror Reflection
// https://leetcode.com/problems/mirror-reflection/
// https://leetcode.cn/problems/mirror-reflection/
//
// There is a special square room with mirrors on each of the four walls. Except for the southwest corner,
// there are receptors on each of the remaining corners, numbered 0, 1, and 2.
//
// The square room has walls of length p and a laser ray from the southwest corner first meets the east wall at a distance q from the 0th receptor.
//
// Given the two integers p and q, return the number of the receptor that the ray meets first.
//
// The test cases are guaranteed so that the ray will meet a receptor eventually.
//
// Example 1:
//
// Input: p = 2, q = 1
// Output: 2
// Explanation: The ray meets receptor 2 the first time it gets reflected back to the left wall.
//
// Example 2:
//
// Input: p = 3, q = 1
// Output: 1
//
// Constraints:
//
// - 1 <= q <= p <= 1000
//

struct Solution;

impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let (mut p, mut q) = (p, q);
        while p % 2 == 0 && q % 2 == 0 {
            p >>= 1;
            q >>= 1;
        }
        if p % 2 == 0 {
            2
        } else {
            i32::from(q % 2 != 0)
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::mirror_reflection(2, 1), 2);
    assert_eq!(Solution::mirror_reflection(3, 1), 1);
    assert_eq!(Solution::mirror_reflection(4, 1), 2);
    assert_eq!(Solution::mirror_reflection(5, 1), 1);
}
