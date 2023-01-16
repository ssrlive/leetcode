#![allow(dead_code)]

// 643. Maximum Average Subarray I
// https://leetcode.com/problems/maximum-average-subarray-i/
// https://leetcode.cn/problems/maximum-average-subarray-i/
//
// You are given an integer array nums consisting of n elements, and an integer k.
//
// Find a contiguous subarray whose length is equal to k that has the maximum average value and return this value.
// Any answer with a calculation error less than 10^-5 will be accepted.
//
// Example 1:
//
// Input: nums = [1,12,-5,-6,50,3], k = 4
// Output: 12.75000
// Explanation: Maximum average is (12 - 5 - 6 + 50) / 4 = 51 / 4 = 12.75
//
// Example 2:
//
// Input: nums = [5], k = 1
// Output: 5.00000
//
// Constraints:
//
// - n == nums.length
// - 1 <= k <= n <= 10^5
// - -10^4 <= nums[i] <= 10^4
//

struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum = 0;
        for &item in nums.iter().take(k as usize) {
            sum += item;
        }
        let mut max_sum = sum;
        for i in k as usize..nums.len() {
            sum += nums[i] - nums[i - k as usize];
            max_sum = max_sum.max(sum);
        }
        max_sum as f64 / k as f64
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4), 12.75);
    assert_eq!(Solution::find_max_average(vec![5], 1), 5.0);
}
