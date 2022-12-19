#![allow(dead_code)]

// 628. Maximum Product of Three Numbers
// https://leetcode.com/problems/maximum-product-of-three-numbers/
// https://leetcode.cn/problems/maximum-product-of-three-numbers/
//
// Given an integer array nums, find three numbers whose product is maximum and return the maximum product.
//
// Example 1:
//
// Input: nums = [1,2,3]
// Output: 6
//
// Example 2:
//
// Input: nums = [1,2,3,4]
// Output: 24
//
// Example 3:
//
// Input: nums = [-1,-2,-3]
// Output: -6
//
// Constraints:
//
// 3 <= nums.length <= 10^4
// -1000 <= nums[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let a = nums[n - 1] * nums[n - 2] * nums[n - 3];
        let b = nums[0] * nums[1] * nums[n - 1];
        a.max(b)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximum_product(vec![1, 2, 3]), 6);
    assert_eq!(Solution::maximum_product(vec![1, 2, 3, 4]), 24);
    assert_eq!(Solution::maximum_product(vec![-1, -2, -3]), -6);
}
