#![allow(dead_code)]

// 1015. Smallest Integer Divisible by K
// https://leetcode.com/problems/smallest-integer-divisible-by-k/
// https://leetcode.cn/problems/smallest-integer-divisible-by-k/
//
// Given a positive integer k, you need to find the length of the smallest positive integer n such that n is divisible by k, and n only contains the digit 1.
//
// Return the length of n. If there is no such n, return -1.
//
// Note: n may not fit in a 64-bit signed integer.
//
// Example 1:
//
// Input: k = 1
// Output: 1
// Explanation: The smallest answer is n = 1, which has length 1.
//
// Example 2:
//
// Input: k = 2
// Output: -1
// Explanation: There is no such positive integer n divisible by 2.
//
// Example 3:
//
// Input: k = 3
// Output: 3
// Explanation: The smallest answer is n = 111, which has length 3.
//
// Constraints:
//
// - 1 <= k <= 10^5
//

struct Solution;

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut m = 0;
        for i in 1..=k {
            m = (m * 10 + 1) % k;
            if m == 0 {
                return i;
            }
        }
        -1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::smallest_repunit_div_by_k(1), 1);
    assert_eq!(Solution::smallest_repunit_div_by_k(2), -1);
    assert_eq!(Solution::smallest_repunit_div_by_k(3), 3);
    assert_eq!(Solution::smallest_repunit_div_by_k(4), -1);
}
