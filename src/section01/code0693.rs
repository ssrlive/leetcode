#![allow(dead_code)]

// 693. Binary Number with Alternating Bits
// https://leetcode.com/problems/binary-number-with-alternating-bits/
// https://leetcode.cn/problems/binary-number-with-alternating-bits/
//
// Given a positive integer, check whether it has alternating bits: namely, if two adjacent bits will always have different values.
//
// Example 1:
//
// Input: n = 5
// Output: true
// Explanation: The binary representation of 5 is: 101
//
// Example 2:
//
// Input: n = 7
// Output: false
// Explanation: The binary representation of 7 is: 111.
//
// Example 3:
//
// Input: n = 11
// Output: false
// Explanation: The binary representation of 11 is: 1011.
//
// Constraints:
//
// - 1 <= n <= 2^31 - 1
//

struct Solution;

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut n = n;
        let mut prev = n & 1;
        n >>= 1;
        while n > 0 {
            let curr = n & 1;
            if curr == prev {
                return false;
            }
            prev = curr;
            n >>= 1;
        }
        true
    }
}

#[test]
fn test() {
    assert!(Solution::has_alternating_bits(5));
    assert!(!Solution::has_alternating_bits(7));
    assert!(!Solution::has_alternating_bits(11));
}
