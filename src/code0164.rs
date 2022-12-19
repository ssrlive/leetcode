#![allow(dead_code)]

// 164. Maximum Gap
// https://leetcode.com/problems/maximum-gap/
// https://leetcode.cn/problems/maximum-gap/
//
// Given an integer array nums, return the maximum difference between two successive elements in its sorted form.
// If the array contains less than two elements, return 0.
//
// You must write an algorithm that runs in linear time and uses linear extra space.
//

struct Solution;

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut max_gap = 0;
        for i in 1..nums.len() {
            max_gap = max_gap.max(nums[i] - nums[i - 1]);
        }
        max_gap
    }
}

#[test]
fn test_maximum_gap() {
    assert_eq!(Solution::maximum_gap(vec![3, 6, 9, 1]), 3);
    assert_eq!(Solution::maximum_gap(vec![10]), 0);
}
