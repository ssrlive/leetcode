#![allow(dead_code)]

// 1175. Prime Arrangements
// https://leetcode.com/problems/prime-arrangements/
// https://leetcode.cn/problems/prime-arrangements/
//
// Return the number of permutations of 1 to n so that prime numbers are at prime indices (1-indexed.)
//
// (Recall that an integer is prime if and only if it is greater than 1, and cannot be written
//   as a product of two positive integers both smaller than it.)
//
// Since the answer may be large, return the answer modulo 10^9 + 7.
//
// Example 1:
//
// Input: n = 5
// Output: 12
// Explanation: For example [1,2,5,4,3] is a valid permutation, but [5,2,3,4,1] is not because the prime number 5 is at index 1.
//
// Example 2:
//
// Input: n = 100
// Output: 682289015
//
// Constraints:
//
// 1 <= n <= 100
//

struct Solution;

impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        const PRIME: [i32; 25] = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
        ];
        let factorial_mod = |n: i32| -> i64 { (2..=n as i64).fold(1, |acc, x| (acc * x) % MOD) };
        let num_primes = PRIME.iter().take_while(|&&x| x <= n).count() as i32;
        ((factorial_mod(num_primes) * factorial_mod(n - num_primes)) % MOD) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_prime_arrangements(5), 12);
    assert_eq!(Solution::num_prime_arrangements(100), 682289015);
}
