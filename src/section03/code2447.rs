#![allow(dead_code)]

// 2447. Number of Subarrays With GCD Equal to K
// https://leetcode.com/problems/number-of-subarrays-with-gcd-equal-to-k/
// https://leetcode.cn/problems/number-of-subarrays-with-gcd-equal-to-k/
//
// Given an integer array nums and an integer k, return the number of subarrays of nums
// where the greatest common divisor of the subarray's elements is k.
//
// A subarray is a contiguous non-empty sequence of elements within an array.
//
// The greatest common divisor of an array is the largest integer that evenly divides all the array elements.
//
// Example 1:
//
// Input: nums = [9,3,1,2,6,3], k = 3
// Output: 4
// Explanation: The subarrays of nums where 3 is the greatest common divisor of all the subarray's elements are:
// - [9,3,1,2,6,3]
// - [9,3,1,2,6,3]
// - [9,3,1,2,6,3]
// - [9,3,1,2,6,3]
//
// Example 2:
//
// Input: nums = [4], k = 7
// Output: 0
// Explanation: There are no subarrays of nums where 7 is the greatest common divisor of all the subarray's elements.
//
// Constraints:
//
// - 1 <= nums.length <= 1000
// - 1 <= nums[i], k <= 10^9
//

struct Solution;

impl Solution {
    pub fn subarray_gcd(nums: Vec<i32>, k: i32) -> i32 {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 { a } else { gcd(b, a % b) }
        }

        let n = nums.len();
        let mut rez = 0;
        for i in 0..n {
            let mut g = nums[i];
            let mut j = i;
            while g >= k && j < n {
                g = gcd(g, nums[j]);
                if g == k {
                    rez += 1;
                }
                j += 1;
            }
        }
        rez
    }
}

#[test]
fn test() {
    assert_eq!(Solution::subarray_gcd(vec![9, 3, 1, 2, 6, 3], 3), 4);
    assert_eq!(Solution::subarray_gcd(vec![4], 7), 0);
}
