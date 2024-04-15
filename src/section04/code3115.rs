#![allow(dead_code)]

// 3115. Maximum Prime Difference
// https://leetcode.com/problems/maximum-prime-difference/
// https://leetcode.cn/problems/maximum-prime-difference/
//
// Medium
//
// You are given an integer array nums.
//
// Return an integer that is the maximum distance between the indices of two (not necessarily different) prime numbers in nums.
//
// Example 1:
//
// Input: nums = [4,2,9,5,3]
//
// Output: 3
//
// Explanation: nums[1], nums[3], and nums[4] are prime. So the answer is |4 - 1| = 3.
//
// Example 2:
//
// Input: nums = [4,8,2,8]
//
// Output: 0
//
// Explanation: nums[2] is prime. Because there is just one prime number, the answer is |2 - 2| = 0.
//
// Constraints:
//
//     1 <= nums.length <= 3 * 10^5
//     1 <= nums[i] <= 100
//     The input is generated such that the number of prime numbers in the nums is at least one.
//

struct Solution;

impl Solution {
    pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
        let left = nums.iter().position(|&x| Self::is_prime(x)).unwrap();
        let right = nums.iter().rposition(|&x| Self::is_prime(x)).unwrap();
        (right - left) as _
    }

    fn is_prime(x: i32) -> bool {
        if x < 2 {
            return false;
        }
        (2..=(x as f64).sqrt() as i32).all(|n| x % n != 0)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximum_prime_difference(vec![4, 2, 9, 5, 3]), 3);
    assert_eq!(Solution::maximum_prime_difference(vec![4, 8, 2, 8]), 0);
}
