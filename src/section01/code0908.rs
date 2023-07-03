#![allow(dead_code)]

// 908. Smallest Range I
// https://leetcode.com/problems/smallest-range-i/
// https://leetcode.cn/problems/smallest-range-i/
//
// You are given an integer array nums and an integer k.
//
// In one operation, you can choose any index i where 0 <= i < nums.length and change nums[i] to nums[i] + x
// where x is an integer from the range [-k, k]. You can apply this operation at most once for each index i.
//
// The score of nums is the difference between the maximum and minimum elements in nums.
//
// Return the minimum score of nums after applying the mentioned operation at most once for each index in it.
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
// Output: 0
// Explanation: Change nums to be [4, 4, 4]. The score is max(nums) - min(nums) = 4 - 4 = 0.
//
// Constraints:
//
// - 1 <= nums.length <= 10^4
// - 0 <= nums[i] <= 10^4
// - 0 <= k <= 10^4
//

struct Solution;

impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let (min_val, max_val) = nums
            .iter()
            .fold((i32::MAX, i32::MIN), |(min_val, max_val), &x| (min_val.min(x), max_val.max(x)));

        (max_val - min_val - 2 * k).max(0)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::smallest_range_i(vec![1], 0), 0);
    assert_eq!(Solution::smallest_range_i(vec![0, 10], 2), 6);
    assert_eq!(Solution::smallest_range_i(vec![1, 3, 6], 3), 0);
}
