#![allow(dead_code)]

// 910. Smallest Range II
// https://leetcode.com/problems/smallest-range-ii/
// https://leetcode.cn/problems/smallest-range-ii/
//
// You are given an integer array nums and an integer k.
//
// For each index i where 0 <= i < nums.length, change nums[i] to be either nums[i] + k or nums[i] - k.
//
// The score of nums is the difference between the maximum and minimum elements in nums.
//
// Return the minimum score of nums after changing the values at each index.
//
// Example 1:
//
// Input: nums = [1], k = 0
// Output: 0
// Explanation: The score is max(nums) - min(nums) = 1 - 1 = 0.
//
// Example 2:
//
// Input: nums = [0,10], k = 2
// Output: 6
// Explanation: Change nums to be [2, 8]. The score is max(nums) - min(nums) = 8 - 2 = 6.
//
// Example 3:
//
// Input: nums = [1,3,6], k = 3
// Output: 3
// Explanation: Change nums to be [4, 6, 3]. The score is max(nums) - min(nums) = 6 - 3 = 3.
//
// Constraints:
//
// - 1 <= nums.length <= 10^4
// - 0 <= nums[i] <= 10^4
// - 0 <= k <= 10^4
//

struct Solution;

impl Solution {
    pub fn smallest_range_ii(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut res = nums[nums.len() - 1] - nums[0];
        let left = nums[0] + k;
        let right = nums[nums.len() - 1] - k;
        for i in 0..nums.len() - 1 {
            let maxi = nums[i] + k;
            let mini = nums[i + 1] - k;
            let maxi = if maxi > right { maxi } else { right };
            let mini = if mini < left { mini } else { left };
            res = if res < maxi - mini { res } else { maxi - mini };
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::smallest_range_ii(vec![1], 0), 0);
    assert_eq!(Solution::smallest_range_ii(vec![0, 10], 2), 6);
    assert_eq!(Solution::smallest_range_ii(vec![1, 3, 6], 3), 3);
}
