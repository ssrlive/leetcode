#![allow(dead_code)]

// 3599. Partition Array to Minimize XOR
// https://leetcode.com/problems/partition-array-to-minimize-xor/
// https://leetcode.cn/problems/partition-array-to-minimize-xor/
//
// Medium
//
// You are given an integer array nums and an integer k.
//
// Your task is to partition nums into k non-empty subarrays.
// For each subarray, compute the bitwise XOR of all its elements.
//
// Return the minimum possible value of the maximum XOR among these k subarrays.
//
// Example 1:
//
// Input: nums = [1,2,3], k = 2
//
// Output: 1
//
// Explanation:
//
// The optimal partition is [1] and [2, 3].
//
//     XOR of the first subarray is 1.
//     XOR of the second subarray is 2 XOR 3 = 1.
//
// The maximum XOR among the subarrays is 1, which is the minimum possible.
//
// Example 2:
//
// Input: nums = [2,3,3,2], k = 3
//
// Output: 2
//
// Explanation:
//
// The optimal partition is [2], [3, 3], and [2].
//
//     XOR of the first subarray is 2.
//     XOR of the second subarray is 3 XOR 3 = 0.
//     XOR of the third subarray is 2.
//
// The maximum XOR among the subarrays is 2, which is the minimum possible.
//
// Example 3:
//
// Input: nums = [1,1,2,3,1], k = 2
//
// Output: 0
//
// Explanation:
//
// The optimal partition is [1, 1] and [2, 3, 1].
//
//     XOR of the first subarray is 1 XOR 1 = 0.
//     XOR of the second subarray is 2 XOR 3 XOR 1 = 0.
//
// The maximum XOR among the subarrays is 0, which is the minimum possible.
//
// Constraints:
//
//     1 <= nums.length <= 250
//     1 <= nums[i] <= 109
//     1 <= k <= n
//

struct Solution;

impl Solution {
    pub fn min_xor(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;

        // Step 1: Prefix XOR
        let mut pfix = vec![0; n + 1];
        for i in 1..=n {
            pfix[i] = pfix[i - 1] ^ nums[i - 1];
        }

        // Step 2: Initialize DP
        let mut dp = vec![vec![i32::MAX; k + 1]; n + 1];
        for i in 0..=n {
            dp[i][1] = pfix[i]; // Base case
        }

        // Step 3: Fill DP table
        for parts in 2..=k {
            for end in parts..=n {
                for split in (parts - 1)..end {
                    let segment_xor = pfix[end] ^ pfix[split];
                    let max_xor = dp[split][parts - 1].max(segment_xor);
                    dp[end][parts] = dp[end][parts].min(max_xor);
                }
            }
        }

        dp[n][k]
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3];
    let k = 2;
    assert_eq!(Solution::min_xor(nums, k), 1);

    let nums = vec![2, 3, 3, 2];
    let k = 3;
    assert_eq!(Solution::min_xor(nums, k), 2);

    let nums = vec![1, 1, 2, 3, 1];
    let k = 2;
    assert_eq!(Solution::min_xor(nums, k), 0);
}
