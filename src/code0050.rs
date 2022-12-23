#![allow(dead_code)]

// 50. Pow(x, n)
// https://leetcode.com/problems/powx-n/
// https://leetcode.cn/problems/powx-n/
//
// Implement pow(x, n), which calculates x raised to the power n (i.e., xn).
//
// Example 1:
//
// Input: x = 2.00000, n = 10
// Output: 1024.00000
//
// Example 2:
//
// Input: x = 2.10000, n = 3
// Output: 9.26100
//
// Example 3:
//
// Input: x = 2.00000, n = -2
// Output: 0.25000
// Explanation: 2-2 = 1/22 = 1/4 = 0.25
//
// Constraints:
//
// - -100.0 < x < 100.0
// - -231 <= n <= 231-1
// - n is an integer.
// - -104 <= xn <= 104
//

struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        fn recurse_my_pow(rez: f64, factor: f64, n: i64) -> f64 {
            if n == 0 {
                rez
            } else {
                let rez = if n & 1 == 1 { rez * factor } else { rez };
                recurse_my_pow(rez, factor * factor, n >> 1)
            }
        }

        let factor = if x == 0.0 {
            0.0
        } else if n >= 0 {
            x
        } else {
            1.0 / x
        };
        let n = if n >= 0 { n as i64 } else { -(n as i64) };
        recurse_my_pow(1.0, factor, n)
    }
}
