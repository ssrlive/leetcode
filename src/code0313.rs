#![allow(dead_code)]

// 313. Super Ugly Number
// https://leetcode.com/problems/super-ugly-number/
//
// A super ugly number is a positive integer whose prime factors are in the array primes.
//
// Given an integer n and an array of integers primes, return the nth super ugly number.
//
// The nth super ugly number is guaranteed to fit in a 32-bit signed integer.
//
// Example 1:
//
// Input: n = 12, primes = [2,7,13,19]
// Output: 32
// Explanation: [1,2,4,7,8,13,14,16,19,26,28,32] is the sequence of the first 12 super ugly numbers given primes = [2,7,13,19].
//
// Example 2:
//
// Input: n = 1, primes = [2,3,5]
// Output: 1
// Explanation: 1 has no prime factors, therefore all of its prime factors are in the array primes = [2,3,5].
//
// Constraints:
//
// - 1 <= n <= 105
// - 1 <= primes.length <= 100
// - 2 <= primes[i] <= 1000
// - primes[i] is guaranteed to be a prime number.
// - All the values of primes are unique and sorted in ascending order.
//

struct Solution;

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        pub fn _nth_super_ugly_number(n: i64, primes: Vec<i64>) -> i64 {
            let mut arri = vec![0_i64; primes.len()];
            let mut arrn = vec![0_i64; primes.len()];
            let mut dp = vec![0_i64; n as usize + 1];
            dp[0] = 1;
            for j in 0..n as usize {
                let mut mini = i64::MAX;
                for i in 0..arri.len() {
                    arrn[i] = dp[arri[i] as usize] * primes[i] as i64;
                    mini = mini.min(arrn[i] as i64);
                }
                for i in 0..arri.len() {
                    if mini == arrn[i] as i64 {
                        dp[j + 1] = arrn[i];
                        arri[i] += 1;
                    }
                }
            }
            dp[n as usize - 1] as i64
        }

        let primes = primes.iter().map(|x| *x as i64).collect();
        _nth_super_ugly_number(n as i64, primes) as i32
    }
}

#[test]
fn test_nth_super_ugly_number() {
    assert_eq!(Solution::nth_super_ugly_number(12, vec![2, 7, 13, 19]), 32);
    assert_eq!(Solution::nth_super_ugly_number(1, vec![2, 3, 5]), 1);
    assert_eq!(
        Solution::nth_super_ugly_number(5911, vec![2, 3, 5, 7]),
        2144153025
    );
}
