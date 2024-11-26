#![allow(dead_code)]

// 3366. Minimum Array Sum
// https://leetcode.com/problems/minimum-array-sum/
// https://leetcode.cn/problems/minimum-array-sum/
//
// Medium
//
// You are given an integer array nums and three integers k, op1, and op2.
//
// You can perform the following operations on nums:
//
// - Operation 1: Choose an index i and divide nums[i] by 2, rounding up to the nearest whole number.
//   You can perform this operation at most op1 times, and not more than once per index.
// - Operation 2: Choose an index i and subtract k from nums[i], but only if nums[i] is greater than or equal to k.
//   You can perform this operation at most op2 times, and not more than once per index.
// - Note: Both operations can be applied to the same index, but at most once each.
//
// Return the minimum possible sum of all elements in nums after performing any number of operations.
//
// Example 1:
//
// Input: nums = [2,8,3,19,3], k = 3, op1 = 1, op2 = 1
//
// Output: 23
//
// Explanation:
//
// Apply Operation 2 to nums[1] = 8, making nums[1] = 5.
// Apply Operation 1 to nums[3] = 19, making nums[3] = 10.
// The resulting array becomes [2, 5, 3, 10, 3], which has the minimum possible sum of 23 after applying the operations.
//
// Example 2:
//
// Input: nums = [2,4,3], k = 3, op1 = 2, op2 = 1
//
// Output: 3
//
// Explanation:
//
// Apply Operation 1 to nums[0] = 2, making nums[0] = 1.
// Apply Operation 1 to nums[1] = 4, making nums[1] = 2.
// Apply Operation 2 to nums[2] = 3, making nums[2] = 0.
// The resulting array becomes [1, 2, 0], which has the minimum possible sum of 3 after applying the operations.
//
// Constraints:
//
// 1 <= nums.length <= 100
// 0 <= nums[i] <= 10^5
// 0 <= k <= 10^5
// 0 <= op1, op2 <= nums.length
//

struct Solution;

impl Solution {
    pub fn min_array_sum(nums: Vec<i32>, k: i32, op1: i32, op2: i32) -> i32 {
        let n = nums.len();
        let op1 = op1 as usize;
        let op2 = op2 as usize;
        let mut dp = vec![vec![vec![i32::MAX; op2 + 1]; op1 + 1]; n + 1];

        for o1 in 0..=op1 {
            for o2 in 0..=op2 {
                dp[n][o1][o2] = 0;
            }
        }

        for pos in (0..n).rev() {
            for o1 in 0..=op1 {
                for o2 in 0..=op2 {
                    let x = nums[pos];
                    let mut min_v = x + dp[pos + 1][o1][o2];

                    if o1 > 0 && o2 > 0 {
                        let num_left = (x + 1) / 2 - k;
                        if num_left >= 0 {
                            min_v = min_v.min(num_left + dp[pos + 1][o1 - 1][o2 - 1]);
                        }
                        if x - k >= 0 {
                            min_v = min_v.min((x - k + 1) / 2 + dp[pos + 1][o1 - 1][o2 - 1]);
                        }
                    }

                    if o1 > 0 {
                        min_v = min_v.min((x + 1) / 2 + dp[pos + 1][o1 - 1][o2]);
                    }

                    if o2 > 0 && x >= k {
                        min_v = min_v.min(x - k + dp[pos + 1][o1][o2 - 1]);
                    }

                    dp[pos][o1][o2] = min_v;
                }
            }
        }

        dp[0][op1][op2]
    }
}

#[test]
fn test() {
    let nums = vec![2, 8, 3, 19, 3];
    let k = 3;
    let op1 = 1;
    let op2 = 1;
    let res = 23;
    assert_eq!(Solution::min_array_sum(nums, k, op1, op2), res);

    let nums = vec![2, 4, 3];
    let k = 3;
    let op1 = 2;
    let op2 = 1;
    let res = 3;
    assert_eq!(Solution::min_array_sum(nums, k, op1, op2), res);
}
