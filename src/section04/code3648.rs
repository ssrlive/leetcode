#![allow(dead_code)]

// 3648. Minimum Sensors to Cover Grid
// https://leetcode.com/problems/minimum-sensors-to-cover-grid/
// https://leetcode.cn/problems/minimum-sensors-to-cover-grid/
//
// Medium
//
// You are given n × m grid and an integer k.
//
// A sensor placed on cell (r, c) covers all cells whose Chebyshev distance from (r, c) is at most k.
//
// The Chebyshev distance between two cells (r1, c1) and (r2, c2) is max(|r1 − r2|,|c1 − c2|).
//
// Your task is to return the minimum number of sensors required to cover every cell of the grid.
//
// Example 1:
//
// Input: n = 5, m = 5, k = 1
//
// Output: 4
//
// Explanation:
//
// Placing sensors at positions (0, 3), (1, 0), (3, 3), and (4, 1) ensures every cell in the grid is covered. Thus, the answer is 4.
//
// Example 2:
//
// Input: n = 2, m = 2, k = 2
//
// Output: 1
//
// Explanation:
//
// With k = 2, a single sensor can cover the entire 2 * 2 grid regardless of its position. Thus, the answer is 1.
//
// Constraints:
//
// 1 <= n <= 10^3
// 1 <= m <= 10^3
// 0 <= k <= 10^3
//

struct Solution;

impl Solution {
    pub fn min_sensors(n: i32, m: i32, k: i32) -> i32 {
        let s = 2 * k + 1;
        (n as f64 / s as f64).ceil() as i32 * (m as f64 / s as f64).ceil() as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_sensors(5, 5, 1), 4);
    assert_eq!(Solution::min_sensors(2, 2, 2), 1);
}
