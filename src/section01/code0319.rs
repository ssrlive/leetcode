#![allow(dead_code)]

// 319. Bulb Switcher
// https://leetcode.com/problems/bulb-switcher/
// https://leetcode.cn/problems/bulb-switcher/
//
// There are n bulbs that are initially off. You first turn on all the bulbs.
// Then, you turn off every second bulb. On the third round, you toggle every
// third bulb (turning on if it's off or turning off if it's on). For the ith
// round, you toggle every i bulb. For the nth round, you only toggle the last
// bulb. Return the number of bulbs that are on after n rounds.
//
// Example 1:
//
// Input: n = 3
// Output: 1
// Explanation:
// At the first round, the three bulbs are [on, on, on].
// At the second round, the three bulbs are [on, off, on].
// At the third round, the three bulbs are [on, off, off].
// So you should return 1 because there is only one bulb is on.
//
// Example 2:
//
// Input: n = 0
// Output: 0
//
// Example 3:
//
// Input: n = 1
// Output: 1
//
// Constraints:
//
// - 0 <= n <= 10^9
//
// Follow up: Could you find an O(1) solution?
//

struct Solution;

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64).sqrt() as i32
    }
}

#[test]
fn test_bulb_switch() {
    assert_eq!(Solution::bulb_switch(3), 1);
    assert_eq!(Solution::bulb_switch(0), 0);
    assert_eq!(Solution::bulb_switch(1), 1);
}
