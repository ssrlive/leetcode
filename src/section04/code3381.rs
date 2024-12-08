#![allow(dead_code)]

// 3381. Maximum Subarray Sum With Length Divisible by K
// https://leetcode.com/problems/maximum-subarray-sum-with-length-divisible-by-k/
// https://leetcode.cn/problems/maximum-subarray-sum-with-length-divisible-by-k/
//
// Medium
//
// You are given an array of integers nums and an integer k.
//
// Return the maximum sum of a non-empty subarray of nums, such that the size of the subarray is divisible by k.
//
// A subarray is a contiguous non-empty sequence of elements within an array.
//
// Example 1:
//
// Input: nums = [1,2], k = 1
//
// Output: 3
//
// Explanation:
//
// The subarray [1, 2] with sum 3 has length equal to 2 which is divisible by 1.
//
// Example 2:
//
// Input: nums = [-1,-2,-3,-4,-5], k = 4
//
// Output: -10
//
// Explanation:
//
// The maximum sum subarray is [-1, -2, -3, -4] which has length equal to 4 which is divisible by 4.
//
// Example 3:
//
// Input: nums = [-5,1,2,-3,4], k = 2
//
// Output: 4
//
// Explanation:
//
// The maximum sum subarray is [1, 2, -3, 4] which has length equal to 4 which is divisible by 2.
//
// Constraints:
//
// 1 <= k <= nums.length <= 2 * 10^5
// -10^9 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        let mut prefix = vec![0; n + 1];

        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i] as i64;
        }

        let mut dp = vec![0; n];
        let mut res = i64::MIN;

        for i in (k - 1)..n {
            let s_add = prefix[i + 1] - prefix[i + 1 - k];
            let x = if i >= k { s_add + dp[i - k] } else { s_add }.max(s_add);
            dp[i] = x;
            res = res.max(x);
        }

        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 2];
    let k = 1;
    let res = 3;
    assert_eq!(Solution::max_subarray_sum(nums, k), res);

    let nums = vec![-1, -2, -3, -4, -5];
    let k = 4;
    let res = -10;
    assert_eq!(Solution::max_subarray_sum(nums, k), res);

    let nums = vec![-5, 1, 2, -3, 4];
    let k = 2;
    let res = 4;
    assert_eq!(Solution::max_subarray_sum(nums, k), res);
}
