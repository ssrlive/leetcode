#![allow(dead_code)]

// 491. Increasing Subsequences
// https://leetcode.com/problems/increasing-subsequences/
// https://leetcode.cn/problems/increasing-subsequences/
//
// Given an integer array nums, return all the different possible increasing subsequences of the given array with at least two elements. You may return the answer in any order.
//
// The given array may contain duplicates, and two equal integers should also be considered a special case of increasing sequence.
//
// Example 1:
//
// Input: nums = [4,6,7,7]
// Output: [[4,6],[4,6,7],[4,6,7,7],[4,7],[4,7,7],[6,7],[6,7,7],[7,7]]
//
// Example 2:
//
// Input: nums = [4,4,3,2,1]
// Output: [[4,4]]
//
// Constraints:
//
// - 1 <= nums.length <= 15
// - -100 <= nums[i] <= 100
//
// Follow up: Could you find an algorithm that runs in O(n2) complexity?

struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = HashSet::new();
        let mut path = Vec::new();
        Self::backtrack(&nums, 0, &mut path, &mut result);
        result.into_iter().collect()
    }

    fn backtrack(nums: &[i32], start: usize, path: &mut Vec<i32>, result: &mut HashSet<Vec<i32>>) -> Option<()> {
        if path.len() >= 2 {
            result.insert(path.clone());
        }

        for i in start..nums.len() {
            if path.is_empty() || nums[i] >= *path.last()? {
                path.push(nums[i]);
                Self::backtrack(nums, i + 1, path, result)?;
                path.pop();
            }
        }
        Some(())
    }
}

#[test]
fn test_find_subsequences() {
    let nums = vec![4, 6, 7, 7];
    let result = Solution::find_subsequences(nums);
    assert_eq!(result.len(), 8);
}
