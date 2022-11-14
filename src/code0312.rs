#![allow(dead_code)]

// 312. Burst Balloons
// https://leetcode.com/problems/burst-balloons/
//
// You are given n balloons, indexed from 0 to n - 1. Each balloon is painted with a number
// on it represented by an array nums. You are asked to burst all the balloons.
//
// If you burst the ith balloon, you will get nums[i - 1] * nums[i] * nums[i + 1] coins.
// If i - 1 or i + 1 goes out of bounds of the array, then treat it as if there is a balloon with a 1 painted on it.
//
// Return the maximum coins you can collect by bursting the balloons wisely.
//
// Example 1:
//
// Input: nums = [3,1,5,8]
// Output: 167
// Explanation:
// nums = [3,1,5,8] --> [3,5,8] --> [3,8] --> [8] --> []
// coins =  3*1*5    +   3*5*8   +  1*3*8  + 1*8*1 = 167
//
// Example 2:
//
// Input: nums = [1,5]
// Output: 10
//
// Constraints:
//
// - n == nums.length
// - 1 <= n <= 300
// - 0 <= nums[i] <= 100

struct Solution;

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.push(1);
        nums.insert(0, 1);
        let n = nums.len();
        
        let mut dp = vec![vec![0; n]; n];
        for i in (1..n - 1).rev() {
            for j in i..n - 1 {
                for k in i..=j {
                    let coins = nums[i - 1]*nums[k]*nums[j + 1] + dp[i][k - 1] + dp[k + 1][j];
                    dp[i][j] = dp[i][j].max(coins);
                }
            }
        }
        dp[1][n - 2]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
    assert_eq!(Solution::max_coins(vec![1, 5]), 10);
}
