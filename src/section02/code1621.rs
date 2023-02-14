#![allow(dead_code)]

/*

// 1621. Number of Sets of K Non-Overlapping Line Segments
// https://leetcode.com/problems/number-of-sets-of-k-non-overlapping-line-segments/
// https://leetcode.cn/problems/number-of-sets-of-k-non-overlapping-line-segments/
//
// Medium
//
// Given n points on a 1-D plane, where the ith point (from 0 to n-1) is at x = i, find the number of ways we can draw exactly k non-overlapping line segments such that each segment covers two or more points. The endpoints of each segment must have integral coordinates. The k line segments do not have to cover all n points, and they are allowed to share endpoints.

Return the number of ways we can draw k non-overlapping line segments. Since this number can be huge, return it modulo 109 + 7.

Example 1:

Input: n = 4, k = 2
Output: 5
Explanation: The two line segments are shown in red and blue.
The image above shows the 5 different ways {(0,2),(2,3)}, {(0,1),(1,3)}, {(0,1),(2,3)}, {(1,2),(2,3)}, {(0,1),(1,2)}.

Example 2:

Input: n = 3, k = 1
Output: 3
Explanation: The 3 ways are {(0,1)}, {(0,2)}, {(1,2)}.

Example 3:

Input: n = 30, k = 7
Output: 796297179
Explanation: The total number of possible ways to draw 7 line segments is 3796297200. Taking this number modulo 109 + 7 gives us 796297179.

Constraints:

    2 <= n <= 1000
    1 <= k <= n-1
*/

struct Solution;

impl Solution {
    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        fn ncr(n: usize, r: usize, fact: &[i64], invfact: &[i64]) -> i64 {
            if r > n {
                return 0;
            }
            mult(mult(fact[n], invfact[n - r]), invfact[r])
        }

        fn mult(a: i64, b: i64) -> i64 {
            a * b % MOD
        }

        let (n, k) = (n as usize, k as usize);
        let mut fact = vec![0; n + k + 1];
        let mut inv = vec![0; n + k + 1];
        let mut invfact = vec![0; n + k + 1];

        fact[0] = 1;
        invfact[0] = 1;
        fact[1] = 1;
        invfact[1] = 1;
        inv[1] = 1;
        for i in 2..n + k + 1 {
            fact[i] = mult(fact[i - 1], i as i64);
            inv[i] = mult(inv[MOD as usize % i], MOD - MOD / i as i64);
            invfact[i] = mult(invfact[i - 1], inv[i]);
        }

        ncr(n + k - 1, 2 * k, &fact, &invfact) as _
    }
}

#[test]
fn test() {
    let cases = vec![(4, 2, 5), (3, 1, 3), (30, 7, 796_297_179)];
    for (n, k, expected) in cases {
        assert_eq!(Solution::number_of_sets(n, k), expected);
    }
}
