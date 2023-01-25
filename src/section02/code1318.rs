#![allow(dead_code)]

// 1318. Minimum Flips to Make a OR b Equal to c
// https://leetcode.com/problems/minimum-flips-to-make-a-or-b-equal-to-c/
// https://leetcode.cn/problems/minimum-flips-to-make-a-or-b-equal-to-c/
//
// Medium
//
// Given 3 positives numbers a, b and c. Return the minimum flips required in some bits of a and b
// to make ( a OR b == c ). (bitwise OR operation).
//
// Flip operation consists of change any single bit 1 to 0 or change the bit 0 to 1 in their binary representation.
//
// Example 1:
//
// Input: a = 2, b = 6, c = 5
// Output: 3
// Explanation: After flips a = 1 , b = 4 , c = 5 such that (a OR b == c)
//
// Example 2:
//
// Input: a = 4, b = 2, c = 7
// Output: 1
//
// Example 3:
//
// Input: a = 1, b = 2, c = 3
// Output: 0
//
// Constraints:
//
// -    1 <= a <= 10^9
// -    1 <= b <= 10^9
// -    1 <= c <= 10^9
//

struct Solution;

impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut a = a;
        let mut b = b;
        let mut c = c;
        let mut count = 0;
        while a > 0 || b > 0 || c > 0 {
            let a_bit = a & 1;
            let b_bit = b & 1;
            let c_bit = c & 1;
            if c_bit == 0 {
                count += a_bit + b_bit;
            } else if a_bit == 0 && b_bit == 0 {
                count += 1;
            }
            a >>= 1;
            b >>= 1;
            c >>= 1;
        }
        count
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_flips(2, 6, 5), 3);
    assert_eq!(Solution::min_flips(4, 2, 7), 1);
    assert_eq!(Solution::min_flips(1, 2, 3), 0);
}
