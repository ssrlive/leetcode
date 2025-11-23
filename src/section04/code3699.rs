#![allow(dead_code)]

// 3699. Number of ZigZag Arrays I
// https://leetcode.com/problems/number-of-zigzag-arrays-i/
// https://leetcode.cn/problems/number-of-zigzag-arrays-i/
//
// Hard
//
// You are given three integers n, l, and r.
//
// A ZigZag array of length n is defined as follows:
//
// Each element lies in the range [l, r].
// No two adjacent elements are equal.
// No three consecutive elements form a strictly increasing or strictly decreasing sequence.
// Return the total number of valid ZigZag arrays.
//
// Since the answer may be large, return it modulo 109 + 7.
//
// A sequence is said to be strictly increasing if each element is strictly greater than its previous one (if exists).
//
// A sequence is said to be strictly decreasing if each element is strictly smaller than its previous one (if exists).
//
// Example 1:
//
// Input: n = 3, l = 4, r = 5
//
// Output: 2
//
// Explanation:
//
// There are only 2 valid ZigZag arrays of length n = 3 using values in the range [4, 5]:
//
// [4, 5, 4]
// [5, 4, 5]​​​​​​​
//
// Example 2:
//
// Input: n = 3, l = 1, r = 3
//
// Output: 10
//
// Explanation:
//
// There are 10 valid ZigZag arrays of length n = 3 using values in the range [1, 3]:
//
// [1, 2, 1], [1, 3, 1], [1, 3, 2]
// [2, 1, 2], [2, 1, 3], [2, 3, 1], [2, 3, 2]
// [3, 1, 2], [3, 1, 3], [3, 2, 3]
// All arrays meet the ZigZag conditions.
//
// Constraints:
//
// 3 <= n <= 2000
// 1 <= l < r <= 2000
//

struct Solution;

impl Solution {
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let r = (r - l) as usize;
        let mut dp = vec![1i64; r + 1];
        for i in 1..n as usize {
            let mut pre = 0i64;
            if i & 1 == 1 {
                for dp_v in dp.iter_mut().take(r + 1) {
                    let pre2 = pre + *dp_v;
                    *dp_v = pre;
                    pre = pre2 % MOD;
                }
            } else {
                for v in (0..=r).rev() {
                    let pre2 = pre + dp[v];
                    dp[v] = pre;
                    pre = pre2 % MOD;
                }
            }
        }
        let res = dp.iter().fold(0i64, |acc, &x| (acc + x) % MOD);
        ((res * 2) % MOD) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::zig_zag_arrays(3, 4, 5), 2);
    assert_eq!(Solution::zig_zag_arrays(3, 1, 3), 10);
}
