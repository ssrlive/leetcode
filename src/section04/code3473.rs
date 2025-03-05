#![allow(dead_code)]

// 3473. Sum of K Subarrays With Length at Least M
// https://leetcode.com/problems/sum-of-k-subarrays-with-length-at-least-m/
// https://leetcode.cn/problems/sum-of-k-subarrays-with-length-at-least-m/
//
// Medium
//
// You are given an integer array nums and two integers, k and m.
//
// Return the maximum sum of k non-overlapping subarrays of nums, where each subarray has a length of at least m.
//
// Example 1:
//
// Input: nums = [1,2,-1,3,3,4], k = 2, m = 2
//
// Output: 13
//
// Explanation:
//
// The optimal choice is:
//
// Subarray nums[3..5] with sum 3 + 3 + 4 = 10 (length is 3 >= m).
// Subarray nums[0..1] with sum 1 + 2 = 3 (length is 2 >= m).
// The total sum is 10 + 3 = 13.
//
// Example 2:
//
// Input: nums = [-10,3,-1,-2], k = 4, m = 1
//
// Output: -10
//
// Explanation:
//
// The optimal choice is choosing each element as a subarray. The output is (-10) + 3 + (-1) + (-2) = -10.
//
// Constraints:
//
// 1 <= nums.length <= 2000
// -10^4 <= nums[i] <= 10^4
// 1 <= k <= floor(nums.length / m)
// 1 <= m <= 3
//

struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32, m: i32) -> i32 {
        const MOD: i32 = -1000000000;
        let n = nums.len();
        let mut prefix = vec![0; n + 1];

        nums.iter().fold(0, |acc, x| {
            let i = acc + 1;
            prefix[i] = prefix[i - 1] + x;
            i
        });

        let mut dp = vec![vec![MOD; n + 1]; k as usize + 1];

        for j in 0..=n {
            dp[0][j] = 0;
        }

        for i in 0..k {
            let mut best = MOD;
            for j in 0..=n {
                if j > 0 {
                    dp[(i + 1) as usize][j] = dp[(i + 1) as usize][j].max(dp[(i + 1) as usize][j - 1]);
                }

                if j as i32 - m >= 0 {
                    best = best.max(dp[i as usize][j - m as usize] - prefix[j - m as usize]);
                }

                if best != MOD {
                    dp[(i + 1) as usize][j] = dp[(i + 1) as usize][j].max(prefix[j] + best);
                }
            }
        }

        dp[k as usize][n]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_sum(vec![1, 2, -1, 3, 3, 4], 2, 2), 13);
    assert_eq!(Solution::max_sum(vec![-10, 3, -1, -2], 4, 1), -10);
}
