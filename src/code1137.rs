#![allow(dead_code)]

// 1137. N-th Tribonacci Number
// https://leetcode.com/problems/n-th-tribonacci-number/
// https://leetcode.cn/problems/n-th-tribonacci-number/
//
// The Tribonacci sequence Tn is defined as follows:
//
// T0 = 0, T1 = 1, T2 = 1, and Tn+3 = Tn + Tn+1 + Tn+2 for n >= 0.
//
// Given n, return the value of Tn.
//
// Example 1:
//
// Input: n = 4
// Output: 4
// Explanation:
// T_3 = 0 + 1 + 1 = 2
// T_4 = 1 + 1 + 2 = 4
//
// Example 2:
//
// Input: n = 25
// Output: 1389537
//
// Constraints:
//
// - 0 <= n <= 37
// - The answer is guaranteed to fit within a 32-bit integer, ie. answer <= 2^31 - 1.
//

struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        fn inner(n: i32, mut tribo: [i32; 3]) -> i32 {
            let tribo_sum = tribo.iter().sum();
            if n == 3 {
                return tribo_sum;
            }
            tribo = [tribo[1], tribo[2], tribo_sum];
            inner(n - 1, tribo)
        }
        match n {
            0 => 0,
            1 | 2 => 1,
            _ => inner(n, [0, 1, 1]),
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::tribonacci(4), 4);
    assert_eq!(Solution::tribonacci(25), 1389537);
}
