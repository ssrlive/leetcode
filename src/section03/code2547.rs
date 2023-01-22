#![allow(dead_code)]

// 2547. Minimum Cost to Split an Array
// https://leetcode.com/problems/minimum-cost-to-split-an-array/
// https://leetcode.cn/problems/minimum-cost-to-split-an-array/
//
// Hard
//
// You are given an integer array nums and an integer k.
//
// Split the array into some number of non-empty subarrays. The cost of a split is the sum
// of the importance value of each subarray in the split.
//
// Let trimmed(subarray) be the version of the subarray where all numbers which appear only once are removed.
//
// For example, trimmed([3,1,2,4,3,4]) = [3,4,3,4].
// The importance value of a subarray is k + trimmed(subarray).length.
//
// For example, if a subarray is [1,2,3,3,3,4,4], then trimmed([1,2,3,3,3,4,4]) = [3,3,3,4,4].
// The importance value of this subarray will be k + 5.
// Return the minimum possible cost of a split of nums.
//
// A subarray is a contiguous non-empty sequence of elements within an array.
//
// Example 1:
//
// Input: nums = [1,2,1,2,1,3,3], k = 2
// Output: 8
// Explanation: We split nums to have two subarrays: [1,2], [1,2,1,3,3].
// The importance value of [1,2] is 2 + (0) = 2.
// The importance value of [1,2,1,3,3] is 2 + (2 + 2) = 6.
// The cost of the split is 2 + 6 = 8. It can be shown that this is the minimum possible cost among all the possible splits.
//
// Example 2:
//
// Input: nums = [1,2,1,2,1], k = 2
// Output: 6
// Explanation: We split nums to have two subarrays: [1,2], [1,2,1].
// The importance value of [1,2] is 2 + (0) = 2.
// The importance value of [1,2,1] is 2 + (2) = 4.
// The cost of the split is 2 + 4 = 6. It can be shown that this is the minimum possible cost among all the possible splits.
//
// Example 3:
//
// Input: nums = [1,2,1,2,1], k = 5
// Output: 10
// Explanation: We split nums to have one subarray: [1,2,1,2,1].
// The importance value of [1,2,1,2,1] is 5 + (3 + 2) = 10.
// The cost of the split is 10. It can be shown that this is the minimum possible cost among all the possible splits.
//
// Constraints:
//
// -    1 <= nums.length <= 1000
// -    0 <= nums[i] < nums.length
// -    1 <= k <= 10^9
//

struct Solution;

impl Solution {
    pub fn min_cost(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        let n = nums.len();
        let mut dp = vec![i32::MAX; n];

        for i in 0..n {
            let mut mp = HashMap::<i32, i32>::new();
            let mut sum = 0;
            for j in (0..=i).rev() {
                if let Some(cnt) = mp.get(&nums[j]) {
                    sum += 1;
                    if *cnt == 1 {
                        sum += 1;
                    }
                }
                *mp.entry(nums[j]).or_insert(0) += 1;
                let mut temp = k + sum;
                if j > 0 {
                    temp += dp[j - 1];
                }
                dp[i] = dp[i].min(temp);
            }
        }
        dp[n - 1]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_cost(vec![1, 2, 1, 2, 1, 3, 3], 2), 8);
    assert_eq!(Solution::min_cost(vec![1, 2, 1, 2, 1], 2), 6);
    assert_eq!(Solution::min_cost(vec![1, 2, 1, 2, 1], 5), 10);
}
