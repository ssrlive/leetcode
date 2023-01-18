#![allow(dead_code)]

// 1201. Ugly Number III
// https://leetcode.com/problems/ugly-number-iii/
// https://leetcode.cn/problems/ugly-number-iii/
//
// Medium
//
// An ugly number is a positive integer that is divisible by a, b, or c.
//
// Given four integers n, a, b, and c, return the nth ugly number.
//
// Example 1:
//
// Input: n = 3, a = 2, b = 3, c = 5
// Output: 4
// Explanation: The ugly numbers are 2, 3, 4, 5, 6, 8, 9, 10... The 3rd is 4.
//
// Example 2:
//
// Input: n = 4, a = 2, b = 3, c = 4
// Output: 6
// Explanation: The ugly numbers are 2, 3, 4, 6, 8, 9, 10, 12... The 4th is 6.
//
// Example 3:
//
// Input: n = 5, a = 2, b = 11, c = 13
// Output: 10
// Explanation: The ugly numbers are 2, 4, 6, 8, 10, 11, 12, 13... The 5th is 10.
//
// Constraints:
//
// -    1 <= n, a, b, c <= 10^9
// -    1 <= a * b * c <= 10^18
// -    It is guaranteed that the result will be in range [1, 2 * 10^9].
//

struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
        fn count(n: i64, a: i64, b: i64, c: i64) -> i64 {
            let mut count = 0;
            count += n / a;
            count += n / b;
            count += n / c;
            count -= n / lcm(a, b);
            count -= n / lcm(a, c);
            count -= n / lcm(b, c);
            count += n / lcm(a, lcm(b, c));
            count
        }

        fn gcd(a: i64, b: i64) -> i64 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }

        fn lcm(a: i64, b: i64) -> i64 {
            a * b / gcd(a, b)
        }

        let (n, a, b, c) = (n as i64, a as i64, b as i64, c as i64);

        let mut left = 1;
        let mut right = 2 * 10_i64.pow(9);
        while left < right {
            let mid = left + (right - left) / 2;
            let count = count(mid, a, b, c);
            if count < n {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::nth_ugly_number(3, 2, 3, 5), 4);
    assert_eq!(Solution::nth_ugly_number(4, 2, 3, 4), 6);
    assert_eq!(Solution::nth_ugly_number(5, 2, 11, 13), 10);
}
