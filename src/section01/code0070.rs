#![allow(dead_code)]

// 70. Climbing Stairs
// https://leetcode.com/problems/climbing-stairs/
// https://leetcode.cn/problems/climbing-stairs/
//
// You are climbing a staircase. It takes n steps to reach the top.
//
// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
//
// Example 1:
//
// Input: n = 2
// Output: 2
// Explanation: There are two ways to climb to the top.
// 1. 1 step + 1 step
// 2. 2 steps
//
// Example 2:
//
// Input: n = 3
// Output: 3
// Explanation: There are three ways to climb to the top.
// 1. 1 step + 1 step + 1 step
// 2. 1 step + 2 steps
// 3. 2 steps + 1 step
//
// Constraints:
//
// - 1 <= n <= 45
//

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut arr = [2, 1];
        let mut i = 3;
        while i <= n as usize {
            arr[i & 0b01] = arr[0] + arr[1];
            i += 1;
        }
        arr[n as usize & 0b01]
    }
}

#[test]
fn test() {
    let cases = vec![(2, 2), (3, 3), (4, 5), (5, 8), (6, 13), (7, 21)];
    for (n, expected) in cases {
        assert_eq!(Solution::climb_stairs(n), expected);
    }
}
