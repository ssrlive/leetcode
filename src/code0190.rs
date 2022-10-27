#![allow(dead_code)]

// 190. Reverse Bits
// https://leetcode.com/problems/reverse-bits/
//
// Reverse bits of a given 32 bits unsigned integer.
//
// Note:
// - Note that in some languages such as Java, there is no unsigned integer type.
//   In this case, both input and output will be given as a signed integer type.
//   They should not affect your implementation, as the integer's internal binary representation is the same, whether it is signed or unsigned.
// - In Java, the compiler represents the signed integers using 2's complement notation.
//   Therefore, in Example 2 above, the input represents the signed integer -3 and the output represents the signed integer -1073741825.
//

struct Solution {}

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut x = x;
        (0..32).into_iter().fold(0, |mut ans, _| {
            ans = (ans << 1) | (x & 1);
            x >>= 1;
            ans
        })
    }
}

#[test]
fn test_reverse_bits() {
    assert_eq!(
        Solution::reverse_bits(0b00000010100101000001111010011100),
        0b00111001011110000010100101000000
    );
    assert_eq!(
        Solution::reverse_bits(0b11111111111111111111111111111101),
        0b10111111111111111111111111111111
    );
}
