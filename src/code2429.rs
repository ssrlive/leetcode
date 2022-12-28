#![allow(dead_code)]

// 2429. Minimize XOR
// https://leetcode.com/problems/minimize-xor/
// https://leetcode.cn/problems/minimize-xor/
//
// Given two positive integers num1 and num2, find the positive integer x such that:
//
// x has the same number of set bits as num2, and
// The value x XOR num1 is minimal.
// Note that XOR is the bitwise XOR operation.
//
// Return the integer x. The test cases are generated such that x is uniquely determined.
//
// The number of set bits of an integer is the number of 1's in its binary representation.
//
// Example 1:
//
// Input: num1 = 3, num2 = 5
// Output: 3
// Explanation:
// The binary representations of num1 and num2 are 0011 and 0101, respectively.
// The integer 3 has the same number of set bits as num2, and the value 3 XOR 3 = 0 is minimal.
//
// Example 2:
//
// Input: num1 = 1, num2 = 12
// Output: 3
// Explanation:
// The binary representations of num1 and num2 are 0001 and 1100, respectively.
// The integer 3 has the same number of set bits as num2, and the value 3 XOR 1 = 2 is minimal.
//
// Constraints:
//
// - 1 <= num1, num2 <= 10^9
//

struct Solution;

impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let mut n = num2.count_ones();
        let mut i = 31;
        let mut x = 0;
        let mut num = num1;
        while n > 0 && num > 0 {
            let bm = 1 << (i - 1);
            if num & bm > 0 {
                num ^= bm;
                x |= bm;
                n -= 1;
            }
            i -= 1;
        }
        i = 0;
        while n > 0 {
            let bm = 1 << i;
            if num1 & bm == 0 {
                x |= bm;
                n -= 1;
            }
            i += 1;
        }
        x
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimize_xor(3, 5), 3);
    assert_eq!(Solution::minimize_xor(1, 12), 3);
    assert_eq!(Solution::minimize_xor(1, 1), 1);
    assert_eq!(Solution::minimize_xor(1, 2), 1);
    assert_eq!(Solution::minimize_xor(1, 3), 3);
    assert_eq!(Solution::minimize_xor(1, 4), 1);
}
