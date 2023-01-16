#![allow(dead_code)]

// 1043. Partition Array for Maximum Sum
// https://leetcode.com/problems/partition-array-for-maximum-sum/
// https://leetcode.cn/problems/partition-array-for-maximum-sum/
//
// Given an integer array arr, partition the array into (contiguous) subarrays of length at most k. After partitioning, each subarray has their values changed to become the maximum value of that subarray.
//
// Return the largest sum of the given array after partitioning. Test cases are generated so that the answer fits in a 32-bit integer.
//
// Example 1:
//
// Input: arr = [1,15,7,9,2,5,10], k = 3
// Output: 84
// Explanation: arr becomes [15,15,15,9,10,10,10]
//
// Example 2:
//
// Input: arr = [1,4,1,5,7,3,6,1,9,9,3], k = 4
// Output: 83
//
// Example 3:
//
// Input: arr = [1], k = 1
// Output: 1
//
// Constraints:
//
// - 1 <= arr.length <= 500
// - 0 <= arr[i] <= 10^9
// - 1 <= k <= arr.length
//

struct Solution;

impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![0; arr.len() + 1];
        for i in 0..arr.len() {
            let mut max = 0;
            for j in 1..=k {
                if i + 1 >= j {
                    max = max.max(arr[i + 1 - j]);
                    dp[i + 1] = dp[i + 1].max(dp[i + 1 - j] + max * j as i32);
                }
            }
        }
        dp[arr.len()]
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 15, 7, 9, 2, 5, 10], 3, 84),
        (vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3], 4, 83),
        (vec![1], 1, 1),
    ];
    for (arr, k, expected) in cases {
        assert_eq!(Solution::max_sum_after_partitioning(arr, k), expected);
    }
}
