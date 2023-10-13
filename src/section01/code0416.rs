#![allow(dead_code)]

// 416. Partition Equal Subset Sum
// https://leetcode.com/problems/partition-equal-subset-sum/
// https://leetcode.cn/problems/partition-equal-subset-sum/
//
// Given a non-empty array nums containing only positive integers, find if the array can be partitioned
// into two subsets such that the sum of elements in both subsets is equal.
//
// Example 1:
//
// Input: nums = [1,5,11,5]
// Output: true
// Explanation: The array can be partitioned as [1, 5, 5] and [11].
//
// Example 2:
//
// Input: nums = [1,2,3,5]
// Output: false
// Explanation: The array cannot be partitioned into equal sum subsets.
//
// Constraints:
//
// - 1 <= nums.length <= 200
// - 1 <= nums[i] <= 100
//

struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            return false;
        }
        let target = sum / 2;
        let mut dp = vec![false; (target + 1) as usize];
        dp[0] = true;
        for num in nums {
            for i in (num..=target).rev() {
                dp[i as usize] = dp[i as usize] || dp[(i - num) as usize];
            }
        }
        dp[target as usize]
    }
}

#[test]
fn test() {
    assert!(Solution::can_partition(vec![1, 5, 11, 5]));
    assert!(!Solution::can_partition(vec![1, 2, 3, 5]));
}
