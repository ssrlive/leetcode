#![allow(dead_code)]

// 896. Monotonic Array
// https://leetcode.com/problems/monotonic-array/
// https://leetcode.cn/problems/monotonic-array/
//
// An array is monotonic if it is either monotone increasing or monotone decreasing.
//
// An array nums is monotone increasing if for all i <= j, nums[i] <= nums[j]. An array nums is monotone decreasing if for all i <= j, nums[i] >= nums[j].
//
// Given an integer array nums, return true if the given array is monotonic, or false otherwise.
//
// Example 1:
//
// Input: nums = [1,2,2,3]
// Output: true
//
// Example 2:
//
// Input: nums = [6,5,4,4]
// Output: true
//
// Example 3:
//
// Input: nums = [1,3,2]
// Output: false
//
// Constraints:
//
// - 1 <= nums.length <= 10^5
// - -10^5 <= nums[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }

        let res = nums
            .windows(2)
            .map(|v| (v[0] <= v[1], v[0] >= v[1]))
            .reduce(|acc, x| (acc.0 & x.0, acc.1 & x.1))
            .unwrap();
        res.0 || res.1
    }
}

#[test]
fn test() {
    assert!(Solution::is_monotonic(vec![1, 2, 2, 3]));
    assert!(Solution::is_monotonic(vec![6, 5, 4, 4]));
    assert!(!Solution::is_monotonic(vec![1, 3, 2]));
    assert!(Solution::is_monotonic(vec![1, 2, 4, 5]));
    assert!(Solution::is_monotonic(vec![1, 1, 1]));
    assert!(!Solution::is_monotonic(vec![1, 2, 2, 1]));
}
