#![allow(dead_code)]

// 523. Continuous Subarray Sum
// https://leetcode.com/problems/continuous-subarray-sum/
//
// Given an integer array nums and an integer k, return true if nums has a good subarray or false otherwise.
//
// A good subarray is a subarray where:
//
// - its length is at least two, and
// - the sum of the elements of the subarray is a multiple of k.
//
// Note that:
//
// - A subarray is a contiguous part of the array.
// - An integer x is a multiple of k if there exists an integer n such that x = n * k. 0 is always a multiple of k.
//
// Example 1:
//
// Input: nums = [23,2,4,6,7], k = 6
// Output: true
// Explanation: [2, 4] is a continuous subarray of size 2 whose elements sum up to 6.
//
// Example 2:
//
// Input: nums = [23,2,6,4,7], k = 6
// Output: true
// Explanation: [23, 2, 6, 4, 7] is an continuous subarray of size 5 whose elements sum up to 42.
// 42 is a multiple of 6 because 42 = 7 * 6 and 7 is an integer.
//
// Example 3:
//
// Input: nums = [23,2,6,4,7], k = 13
// Output: false
//
// Constraints:
//
// - 1 <= nums.length <= 105
// - 0 <= nums[i] <= 109
// - 0 <= sum(nums[i]) <= 231 - 1
// - 1 <= k <= 231 - 1
//

struct Solution;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut sum = 0;
        let mut map = std::collections::HashMap::<i32, i32>::new();
        map.insert(0, -1);
        for (i, &item) in nums.iter().enumerate() {
            sum += item;
            if k != 0 {
                sum %= k;
            }
            if let Some(&j) = map.get(&sum) {
                if i as i32 - j >= 2 {
                    return true;
                }
            } else {
                map.insert(sum, i as i32);
            }
        }
        false
    }
}

#[test]
fn test() {
    let nums = vec![23, 2, 4, 6, 7];
    let k = 6;
    assert_eq!(Solution::check_subarray_sum(nums, k), true);
    let nums = vec![23, 2, 6, 4, 7];
    let k = 6;
    assert_eq!(Solution::check_subarray_sum(nums, k), true);
    let nums = vec![23, 2, 6, 4, 7];
    let k = 13;
    assert_eq!(Solution::check_subarray_sum(nums, k), false);
}
