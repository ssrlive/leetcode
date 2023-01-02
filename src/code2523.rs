#![allow(dead_code)]

// 2523. Closest Prime Numbers in Range
// https://leetcode.com/problems/closest-prime-numbers-in-range/
// https://leetcode.cn/problems/closest-prime-numbers-in-range/
//
// Given two positive integers left and right, find the two integers num1 and num2 such that:
//
// left <= nums1 < nums2 <= right .
// nums1 and nums2 are both prime numbers.
// nums2 - nums1 is the minimum amongst all other pairs satisfying the above conditions.
//
// Return the positive integer array ans = [nums1, nums2]. If there are multiple pairs satisfying these conditions,
// return the one with the minimum nums1 value or [-1, -1] if such numbers do not exist.
//
// A number greater than 1 is called prime if it is only divisible by 1 and itself.
//
// Example 1:
//
// Input: left = 10, right = 19
// Output: [11,13]
// Explanation: The prime numbers between 10 and 19 are 11, 13, 17, and 19.
// The closest gap between any pair is 2, which can be achieved by [11,13] or [17,19].
// Since 11 is smaller than 17, we return the first pair.
//
// Example 2:
//
// Input: left = 4, right = 6
// Output: [-1,-1]
// Explanation: There exists only one prime number in the given range, so the conditions cannot be satisfied.
//
// Constraints:
//
// - 1 <= left <= right <= 10^6
//

struct Solution;

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let left = 2.max(left);
        let mut numbers = vec![false; (right + 1) as usize];
        for p in 2..((right as f64).sqrt() as i32 + 1) {
            if !numbers[p as usize] {
                for j in (p * p..=right).step_by(p as usize) {
                    numbers[j as usize] = true;
                }
            }
        }

        let mut primes = vec![];
        for i in left..=right {
            if !numbers[i as usize] {
                primes.push(i);
            }
        }
        if primes.len() < 2 {
            return vec![-1, -1];
        }
        let mut l = primes[0];
        let mut r = primes[1];
        let mut diff = r - l;
        for i in 0..primes.len() - 1 {
            if diff > primes[i + 1] - primes[i] {
                diff = primes[i + 1] - primes[i];
                r = primes[i + 1];
                l = primes[i];
            }
        }
        vec![l, r]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::closest_primes(10, 19), vec![11, 13]);
    assert_eq!(Solution::closest_primes(4, 6), vec![-1, -1]);
}
