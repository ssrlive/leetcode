#![allow(dead_code)]

// 1390. Four Divisors
// https://leetcode.com/problems/four-divisors/
// https://leetcode.cn/problems/four-divisors/
//
// Medium
//
// Given an integer array nums, return the sum of divisors of the integers in that array
// that have exactly four divisors. If there is no such integer in the array, return 0.
//
// Example 1:
//
// Input: nums = [21,4,7]
// Output: 32
// Explanation:
// 21 has 4 divisors: 1, 3, 7, 21
// 4 has 3 divisors: 1, 2, 4
// 7 has 2 divisors: 1, 7
// The answer is the sum of divisors of 21 only.
//
// Example 2:
//
// Input: nums = [21,21]
// Output: 64
//
// Example 3:
//
// Input: nums = [1,2,3,4,5]
// Output: 0
//
// Constraints:
//
// -    1 <= nums.length <= 10^4
// -    1 <= nums[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for n in nums {
            let mut last_d = 0;
            for d in 2..((n as f64).sqrt() as i32 + 1) {
                if n % d == 0 {
                    if last_d == 0 {
                        last_d = d;
                    } else {
                        last_d = 0;
                        break;
                    }
                }
            }
            if last_d > 0 && last_d != n / last_d {
                sum += 1 + n + last_d + n / last_d;
            }
        }
        sum
    }
}

#[test]
fn test() {
    assert_eq!(Solution::sum_four_divisors(vec![21, 4, 7]), 32);
    assert_eq!(Solution::sum_four_divisors(vec![21, 21]), 64);
    assert_eq!(Solution::sum_four_divisors(vec![1, 2, 3, 4, 5]), 0);
}
