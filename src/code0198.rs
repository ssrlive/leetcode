#![allow(dead_code)]

// 198. House Robber
// https://leetcode.com/problems/house-robber/

// You are a professional robber planning to rob houses along a street.
// Each house has a certain amount of money stashed, the only constraint stopping you
// from robbing each of them is that adjacent houses have security system connected
// and it will automatically contact the police if two adjacent houses were broken into on the same night.
//
// Given a list of non-negative integers representing the amount of money of each house,
// determine the maximum amount of money you can rob tonight without alerting the police.

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 2];
        for i in 0..nums.len() {
            dp[i + 2] = std::cmp::max(dp[i] + nums[i], dp[i + 1]);
        }
        dp[nums.len() + 1]
    }
}

#[test]
fn test_rob() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
}
