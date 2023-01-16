#![allow(dead_code)]
#![allow(clippy::mut_range_bound)]

// 2507. Smallest Value After Replacing With Sum of Prime Factors
// https://leetcode.com/problems/smallest-value-after-replacing-with-sum-of-prime-factors/
// https://leetcode.cn/problems/smallest-value-after-replacing-with-sum-of-prime-factors/
//
// You are given a positive integer n.
//
// Continuously replace n with the sum of its prime factors.
//
// - Note that if a prime factor divides n multiple times, it should be included in the sum as many times as it divides n.
//
// Return the smallest value n will take on.
//
// Example 1:
//
// Input: n = 15
// Output: 5
// Explanation: Initially, n = 15.
// 15 = 3 * 5, so replace n with 3 + 5 = 8.
// 8 = 2 * 2 * 2, so replace n with 2 + 2 + 2 = 6.
// 6 = 2 * 3, so replace n with 2 + 3 = 5.
// 5 is the smallest value n will take on.
//
// Example 2:
//
// Input: n = 3
// Output: 3
// Explanation: Initially, n = 3.
// 3 is the smallest value n will take on.
//
// Constraints:
//
// - 2 <= n <= 10^5
//

struct Solution;

impl Solution {
    pub fn smallest_value(n: i32) -> i32 {
        let mut last = -1;
        let mut n = n;
        while last != n {
            last = n;
            let mut sum = vec![];
            for i in 2..=n {
                if i * i > n {
                    break;
                }
                while n % i == 0 && n > 0 {
                    sum.push(i);
                    n /= i;
                }
            }
            if n > 1 {
                sum.push(n);
            }
            n = sum.iter().sum();
        }
        n
    }
}

#[test]
fn test() {
    assert_eq!(Solution::smallest_value(15), 5);
    assert_eq!(Solution::smallest_value(3), 3);
}
