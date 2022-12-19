#![allow(dead_code)]

// 813. Largest Sum of Averages
// https://leetcode.com/problems/largest-sum-of-averages/
// https://leetcode.cn/problems/largest-sum-of-averages/
//
// You are given an integer array nums and an integer k. You can partition the array into at most k non-empty adjacent subarrays.
// The score of a partition is the sum of the averages of each subarray.
//
// Note that the partition must use every integer in nums, and that the score is not necessarily an integer.
//
// Return the maximum score you can achieve of all the possible partitions. Answers within 10-6 of the actual answer will be accepted.
//
// Example 1:
//
// Input: nums = [9,1,2,3,9], k = 3
// Output: 20.00000
// Explanation:
// The best choice is to partition nums into [9], [1, 2, 3], [9]. The answer is 9 + (1 + 2 + 3) / 3 + 9 = 20.
// We could have also partitioned nums into [9, 1], [2], [3, 9], for example.
// That partition would lead to a score of 5 + 2 + 6 = 13, which is worse.
//
// Example 2:
//
// Input: nums = [1,2,3,4,5,6,7], k = 4
// Output: 20.50000
//
// Constraints:
//
// - 1 <= nums.length <= 100
// - 1 <= nums[i] <= 10^4
// - 1 <= k <= nums.length
//

struct Solution;

impl Solution {
    pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        fn solve(nums: &Vec<i32>, idx: usize, k: i32, dp: &mut Vec<Vec<f64>>) -> f64 {
            if idx >= nums.len() {
                return 0.0;
            }
            if k == 0 {
                return std::f64::MIN;
            }
            if dp[idx][k as usize] != -1.0 {
                return dp[idx][k as usize];
            }
            let mut ans = std::f64::MIN;
            let mut sum = 0.0;
            for i in idx..nums.len() {
                sum += nums[i] as f64;
                ans = ans.max(sum / (i - idx + 1) as f64 + solve(nums, i + 1, k - 1, dp));
            }
            dp[idx][k as usize] = ans;
            ans
        }

        let n = nums.len();
        let mut dp = vec![vec![-1.0; k as usize + 1]; n + 1];
        solve(&nums, 0, k, &mut dp)
    }
}

#[test]
fn test() {
    let nums = vec![9, 1, 2, 3, 9];
    let k = 3;
    let expected = 20.0;
    assert_eq!(Solution::largest_sum_of_averages(nums, k), expected);

    let nums = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 4;
    let expected = 20.5;
    assert_eq!(Solution::largest_sum_of_averages(nums, k), expected);
}
