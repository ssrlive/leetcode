#![allow(dead_code)]

// 213. House Robber II
// https://leetcode.com/problems/house-robber-ii/
// https://leetcode.cn/problems/house-robber-ii/
//
// You are a professional robber planning to rob houses along a street.
// Each house has a certain amount of money stashed, the only constraint stopping you from
// robbing each of them is that adjacent houses have security system connected and it will
// automatically contact the police if two adjacent houses were broken into on the same night.
//
// Given an integer array nums representing the amount of money of each house,
// return the maximum amount of money you can rob tonight without alerting the police.
//
// Example 1:
//
// Input: nums = [2,3,2]
// Output: 3
// Explanation: You cannot rob house 1 (money = 2) and then rob house 3 (money = 2),
// because they are adjacent houses.
//
// Example 2:
//
// Input: nums = [1,2,3,1]
// Output: 4
// Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
// Total amount you can rob = 1 + 3 = 4.
//
// Example 3:
//
// Input: nums = [0]
// Output: 0
//
// Constraints:
//
// 1 <= nums.length <= 100
// 0 <= nums[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        let mut dp = vec![0; n];
        dp[0] = nums[0];
        dp[1] = nums[0].max(nums[1]);
        for i in 2..n - 1 {
            dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
        }
        let ans = dp[n - 2];
        dp[0] = 0;
        dp[1] = nums[1];
        for i in 2..n {
            dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
        }
        ans.max(dp[n - 1])
    }
}

#[test]
fn test_rob() {
    assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob(vec![0]), 0);
}
