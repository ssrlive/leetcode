#![allow(dead_code)]

// 191. Number of 1 Bits
// https://leetcode.com/problems/number-of-1-bits/
//
// Write a function that takes an unsigned integer and returns the number of '1' bits it has (also known as the Hamming weight).
//

struct Solution;

impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        std::iter::successors(Some(n), |acc| if *acc == 0 { None } else { Some(*acc & (*acc - 1)) }).count() as i32 - 1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::hamming_weight(0b00000000000000000000000000001011), 3);
    assert_eq!(Solution::hamming_weight(0b00000000000000000000000010000000), 1);
    assert_eq!(Solution::hamming_weight(0b11111111111111111111111111111101), 31);
}
