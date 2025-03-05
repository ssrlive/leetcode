#![allow(dead_code)]

// 878. Nth Magical Number
// https://leetcode.com/problems/nth-magical-number/
// https://leetcode.cn/problems/nth-magical-number/
//
// A positive integer is magical if it is divisible by either a or b.
//
// Given the three integers n, a, and b, return the nth magical number. Since the answer may be very large, return it modulo 10^9 + 7.
//
// Example 1:
//
// Input: n = 1, a = 2, b = 3
// Output: 2
//
// Example 2:
//
// Input: n = 4, a = 2, b = 3
// Output: 6
//
// Constraints:
//
// - 1 <= n <= 10^9
// - 2 <= a, b <= 4 * 10^4
//

struct Solution;

impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        fn gcd(a: i32, b: i32) -> i32 {
            let r = a % b;
            if r == 0 { b } else { gcd(b, r) }
        }

        const CLIP: i64 = 1_000_000_007;
        let (a, b) = (a.min(b), a.max(b));
        if b % a == 0 {
            return ((n as i64 * a as i64) % CLIP) as i32;
        }
        let g = gcd(a, b);
        let (ka, kb) = (b / g, a / g);
        let nk = ka + kb - 1;
        let base = ((n / nk) as i64 * ka as i64 * a as i64 % CLIP) as i32;
        let r = n % nk;
        let (mut pa, mut pb) = (0, 0);
        while pa + pb < r {
            if (pa + 1) * a < (pb + 1) * b {
                pa += 1;
            } else {
                pb += 1;
            }
        }
        ((base as i64 + (pa as i64 * a as i64).max(pb as i64 * b as i64)) % CLIP) as i32
    }
}

#[test]
fn test() {}
