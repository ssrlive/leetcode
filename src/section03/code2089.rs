#![allow(dead_code)]

/*

// 2089. Find Target Indices After Sorting Array
// https://leetcode.com/problems/find-target-indices-after-sorting-array/
// https://leetcode.cn/problems/find-target-indices-after-sorting-array/
//
// Easy
//
// You are given a 0-indexed integer array nums and a target element target.

A target index is an index i such that nums[i] == target.

Return a list of the target indices of nums after sorting nums in non-decreasing order. If there are no target indices, return an empty list. The returned list must be sorted in increasing order.

Example 1:

Input: nums = [1,2,5,2,3], target = 2
Output: [1,2]
Explanation: After sorting, nums is [1,2,2,3,5].
The indices where nums[i] == 2 are 1 and 2.

Example 2:

Input: nums = [1,2,5,2,3], target = 3
Output: [3]
Explanation: After sorting, nums is [1,2,2,3,5].
The index where nums[i] == 3 is 3.

Example 3:

Input: nums = [1,2,5,2,3], target = 5
Output: [4]
Explanation: After sorting, nums is [1,2,2,3,5].
The index where nums[i] == 5 is 4.

Constraints:

    1 <= nums.length <= 100
    1 <= nums[i], target <= 100
*/

struct Solution;

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut num_target, mut num_before) = (0, 0);
        nums.iter().filter(|x| **x <= target).for_each(|&x| match x == target {
            true => num_target += 1,
            false => num_before += 1,
        });
        match num_target {
            0 => vec![],
            _ => (num_before..).take(num_target).collect(),
        }
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 5, 2, 3];
    let target = 2;
    assert_eq!(Solution::target_indices(nums, target), vec![1, 2]);
    let nums = vec![1, 2, 5, 2, 3];
    let target = 3;
    assert_eq!(Solution::target_indices(nums, target), vec![3]);
    let nums = vec![1, 2, 5, 2, 3];
    let target = 5;
    assert_eq!(Solution::target_indices(nums, target), vec![4]);
}
