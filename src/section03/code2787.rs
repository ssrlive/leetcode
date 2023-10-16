#![allow(dead_code)]

// 2787. Ways to Express an Integer as Sum of Powers
// https://leetcode.com/problems/ways-to-express-an-integer-as-sum-of-powers/
// https://leetcode.cn/problems/ways-to-express-an-integer-as-sum-of-powers/
//
// Medium
//
// Given two positive integers n and x.
//
// Return the number of ways n can be expressed as the sum of the xth power of unique positive integers,
// in other words, the number of sets of unique integers [n1, n2, ..., nk] where n = n1x + n2x + ... + nkx.
//
// Since the result can be very large, return it modulo 109 + 7.
//
// For example, if n = 160 and x = 3, one way to express n is n = 23 + 33 + 53.
//
// Example 1:
//
// Input: n = 10, x = 2
// Output: 1
// Explanation: We can express n as the following: n = 32 + 12 = 10.
// It can be shown that it is the only way to express 10 as the sum of the 2nd power of unique integers.
//
// Example 2:
//
// Input: n = 4, x = 1
// Output: 2
// Explanation: We can express n in the following ways:
// - n = 41 = 4.
// - n = 31 + 11 = 4.
//
// Constraints:
//
//     1 <= n <= 300
//     1 <= x <= 5
//

struct Solution;

impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        let mut data = vec![1];
        let n = n as usize;

        for a in 2..=n {
            let mut t = 1;
            let mut k = 0;
            while k < x && t < n {
                t *= a;
                k += 1;
            }
            if t <= n && k == x {
                data.push(t);
            }
        }

        let mut dp = vec![0; n + 1];
        dp[0] = 1;

        for d in data {
            for i in (d..=n).rev() {
                dp[i] = (dp[i] + dp[i - d]) % 1_000_000_007;
            }
        }

        dp[n]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_ways(10, 2), 1);
    assert_eq!(Solution::number_of_ways(4, 1), 2);
}
