#![allow(dead_code)]

/*

// 1680. Concatenation of Consecutive Binary Numbers
// https://leetcode.com/problems/concatenation-of-consecutive-binary-numbers/
// https://leetcode.cn/problems/concatenation-of-consecutive-binary-numbers/
//
// Medium
//
// Given an integer n, return the decimal value of the binary string formed by concatenating the binary representations of 1 to n in order, modulo 109 + 7.

Example 1:

Input: n = 1
Output: 1
Explanation: "1" in binary corresponds to the decimal value 1.

Example 2:

Input: n = 3
Output: 27
Explanation: In binary, 1, 2, and 3 corresponds to "1", "10", and "11".
After concatenating them, we have "11011", which corresponds to the decimal value 27.

Example 3:

Input: n = 12
Output: 505379714
Explanation: The concatenation results in "1101110010111011110001001101010111100".
The decimal value of that is 118505380540.
After modulo 109 + 7, the result is 505379714.

Constraints:

    1 <= n <= 10^5
*/

struct Solution;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let n = n as i64;
        let mut res = 0;
        let mod_num = 1_000_000_007;
        for i in 1..=n {
            let mut x = i;
            let mut bits = 0;
            while x > 0 {
                bits += 1;
                x >>= 1;
            }
            res = ((res << bits) + i) % mod_num;
        }
        res as _
    }
}

#[test]
fn test() {
    let cases = vec![(1, 1), (3, 27), (12, 505379714)];
    for (n, expected) in cases {
        assert_eq!(Solution::concatenated_binary(n), expected);
    }
}
