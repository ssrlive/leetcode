#![allow(dead_code)]

// 152. Maximum Product Subarray
// https://leetcode.com/problems/maximum-product-subarray/
// https://leetcode.cn/problems/maximum-product-subarray/
//
// Given an integer array nums, find the contiguous subarray within an array (containing at least one number) which has the largest product.
//
// The test cases are generated so that the answer will fit in a 32-bit integer.
//
// A subarray is a contiguous subsequence of the array.
//

struct Solution;

use std::cmp::{max, min};
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((1, 1, nums[0]), |(neg, pos, answ), &val| match (val, val > 0) {
                (0, _) => (0, 0, max(0, answ)),
                (_, true) => (neg * val, max(pos * val, val), max(max(pos * val, val), answ)),
                (_, false) => (min(pos * val, val), neg * val, max(neg * val, answ)),
            })
            .2
    }
}

#[test]
fn test_max_product() {
    assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
    assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    assert_eq!(Solution::max_product(vec![-2]), -2);
    assert_eq!(Solution::max_product(vec![-2, 3, -4]), 24);
    assert_eq!(Solution::max_product(vec![0, 2]), 2);
    assert_eq!(Solution::max_product(vec![2, -5, -2, -4, 3]), 24);
}
