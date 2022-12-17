#![allow(dead_code)]

// 818. Race Car
// https://leetcode.com/problems/race-car/
//
// Your car starts at position 0 and speed +1 on an infinite number line. Your car can go into negative positions.
// Your car drives automatically according to a sequence of instructions 'A' (accelerate) and 'R' (reverse):
//
// - When you get an instruction 'A', your car does the following:
//   - position += speed
//   - speed *= 2
// - When you get an instruction 'R', your car does the following:
//   - If your speed is positive then speed = -1
//   - otherwise speed = 1
//
// Your position stays the same.
//
// For example, after commands "AAR", your car goes to positions 0 --> 1 --> 3 --> 3, and your speed goes to 1 --> 2 --> 4 --> -1.
//
// Given a target position target, return the length of the shortest sequence of instructions to get there.
//
// Example 1:
//
// Input: target = 3
// Output: 2
// Explanation:
// The shortest instruction sequence is "AA".
// Your position goes from 0 --> 1 --> 3.
//
// Example 2:
//
// Input: target = 6
// Output: 5
// Explanation:
// The shortest instruction sequence is "AAARA".
// Your position goes from 0 --> 1 --> 3 --> 7 --> 7 --> 6.
//
// Constraints:
//
// - 1 <= target <= 10^4
//

struct Solution;

impl Solution {
    pub fn racecar(target: i32) -> i32 {
        use std::cmp::min;
        let n: usize = target as usize;
        let mut dp: Vec<i32> = vec![std::i32::MAX; n + 1];
        dp[0] = 0;

        for i in 1..n + 1 {
            let power = (i as f64 + 1.0).log2();
            if power.fract() == 0.0 {
                dp[i] = power as i32;
            } else {
                let mut step: i32 = power.floor() as i32;
                let mut j: usize = (1 << step) - 1;
                for bw in 0..step {
                    let distance: usize = (1 << bw) - 1;
                    dp[i] = min(dp[i], step + 1 + bw + 1 + dp[i - (j - distance)])
                }
                step += 1;
                j = (1 << step) - 1;
                dp[i] = min(dp[i], step + 1 + dp[j - i])
            }
        }
        dp[n]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::racecar(3), 2);
    assert_eq!(Solution::racecar(6), 5);
}
