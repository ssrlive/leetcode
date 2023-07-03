#![allow(dead_code)]

// 1027. Longest Arithmetic Subsequence
// https://leetcode.com/problems/longest-arithmetic-subsequence/
// https://leetcode.cn/problems/longest-arithmetic-subsequence/
//
// Given an array nums of integers, return the length of the longest arithmetic subsequence in nums.
//
// Note that:
//
// A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
// A sequence seq is arithmetic if seq[i + 1] - seq[i] are all the same value (for 0 <= i < seq.length - 1).
//
// Example 1:
//
// Input: nums = [3,6,9,12]
// Output: 4
// Explanation:  The whole array is an arithmetic sequence with steps of length = 3.
//
// Example 2:
//
// Input: nums = [9,4,7,2,10]
// Output: 3
// Explanation:  The longest arithmetic subsequence is [4,7,10].
//
// Example 3:
//
// Input: nums = [20,1,15,3,10,5,8]
// Output: 4
// Explanation:  The longest arithmetic subsequence is [20,15,10,5].
//
// Constraints:
//
// - 2 <= nums.length <= 1000
// - 0 <= nums[i] <= 500
//

struct Solution;

impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        fn find_deltas(nums: &[i32]) -> (i32, i32) {
            let mut min_val = nums[0];
            let mut max_val = nums[0];
            let mut min_delta = i32::MAX;
            let mut max_delta = i32::MIN;
            for &n in nums.iter().skip(1) {
                min_delta = min_delta.min(n - max_val);
                max_delta = max_delta.max(n - min_val);
                min_val = min_val.min(n);
                max_val = max_val.max(n);
            }
            (min_delta, max_delta)
        }

        let (min_delta, max_delta) = find_deltas(&nums);
        let size = max_delta - min_delta + 1;
        let mut dp = vec![vec![0; nums.len()]; size as usize];
        let mut max = 2;
        for i in 1..nums.len() {
            for j in 0..i {
                let delta = (nums[i] - nums[j] - min_delta) as usize;
                dp[delta][i] = 2_i32.max(dp[delta][j] + 1);
                max = max.max(dp[delta][i]);
            }
        }
        max
    }
}

#[test]
fn test() {
    let cases = vec![(vec![3, 6, 9, 12], 4), (vec![9, 4, 7, 2, 10], 3), (vec![20, 1, 15, 3, 10, 5, 8], 4)];
    for (nums, expected) in cases {
        assert_eq!(Solution::longest_arith_seq_length(nums), expected);
    }
}
