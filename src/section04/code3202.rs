#![allow(dead_code)]

// 3202. Find the Maximum Length of Valid Subsequence II
// https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-ii/
// https://leetcode.cn/problems/find-the-maximum-length-of-valid-subsequence-ii/
//
// Medium
//
// You are given an integer array nums and a positive integer k.
//
// A subsequence sub of nums with length x is called valid if it satisfies:
//
//     (sub[0] + sub[1]) % k == (sub[1] + sub[2]) % k == ... == (sub[x - 2] + sub[x - 1]) % k.
//
// Return the length of the longest valid subsequence of nums.
//
// Example 1:
//
// Input: nums = [1,2,3,4,5], k = 2
//
// Output: 5
//
// Explanation:
//
// The longest valid subsequence is [1, 2, 3, 4, 5].
//
// Example 2:
//
// Input: nums = [1,4,2,3,1,4], k = 3
//
// Output: 4
//
// Explanation:
//
// The longest valid subsequence is [1, 4, 1, 4].
//
// Constraints:
//
//     2 <= nums.length <= 10^3
//     1 <= nums[i] <= 10^7
//     1 <= k <= 10^3
//

struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut res = 0;
        for r in 0..k {
            let mut dp = vec![0; k];
            for &nums_i in &nums {
                let v = nums_i as usize;
                let r1 = v % k;
                let r2 = (k + r - r1) % k;
                dp[r1] = dp[r1].max(dp[r2] + 1);
            }

            for &dp_i in dp.iter().take(k) {
                res = res.max(dp_i);
            }
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximum_length(vec![1, 2, 3, 4, 5], 2), 5);
    assert_eq!(Solution::maximum_length(vec![1, 4, 2, 3, 1, 4], 3), 4);
    assert_eq!(Solution::maximum_length(vec![1, 2, 3, 4, 5], 3), 4);
}
