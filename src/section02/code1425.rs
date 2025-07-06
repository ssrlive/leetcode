#![allow(dead_code)]

// 1425. Constrained Subsequence Sum
// https://leetcode.com/problems/constrained-subsequence-sum/
// https://leetcode.cn/problems/constrained-subsequence-sum/
//
// Hard
//
// Given an integer array nums and an integer k, return the maximum sum of a non-empty subsequence
// of that array such that for every two consecutive integers in the subsequence, nums[i] and nums[j],
// where i < j, the condition j - i <= k is satisfied.
//
// A subsequence of an array is obtained by deleting some number of elements (can be zero) from the array,
// leaving the remaining elements in their original order.
//
// Example 1:
//
// Input: nums = [10,2,-10,5,20], k = 2
// Output: 37
// Explanation: The subsequence is [10, 2, 5, 20].
//
// Example 2:
//
// Input: nums = [-1,-2,-3], k = 1
// Output: -1
// Explanation: The subsequence must be non-empty, so we choose the largest number.
//
// Example 3:
//
// Input: nums = [10,-2,-10,-5,20], k = 2
// Output: 23
// Explanation: The subsequence is [10, -2, -5, 20].
//
// Constraints:
//
// -    1 <= k <= nums.length <= 10^5
// -    -10^4 <= nums[i] <= 10^4
//

struct Solution;

impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![0; nums.len()];
        let mut max = nums[0];
        let mut q = std::collections::VecDeque::new();
        for i in 0..nums.len() {
            dp[i] = nums[i];
            if let Some(&j) = q.front() {
                dp[i] = std::cmp::max(dp[i], dp[j] + nums[i]);
            }
            max = std::cmp::max(max, dp[i]);
            while let Some(&j) = q.back() {
                if dp[j] < dp[i] {
                    q.pop_back();
                } else {
                    break;
                }
            }
            q.push_back(i);
            if let Some(&j) = q.front()
                && i as i32 - j as i32 >= k
            {
                q.pop_front();
            }
        }
        max
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![10, 2, -10, 5, 20], 2, 37),
        (vec![-1, -2, -3], 1, -1),
        (vec![10, -2, -10, -5, 20], 2, 23),
    ];
    for (nums, k, expected) in cases {
        assert_eq!(Solution::constrained_subset_sum(nums, k), expected);
    }
}
