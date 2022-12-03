#![allow(dead_code)]

// 594. Longest Harmonious Subsequence
// https://leetcode.com/problems/longest-harmonious-subsequence/
//
// We define a harmonious array as an array where the difference between its maximum value and its minimum value is exactly 1.
//
// Given an integer array nums, return the length of its longest harmonious subsequence among all its possible subsequences.
//
// A subsequence of array is a sequence that can be derived from the array by deleting some or no elements
// without changing the order of the remaining elements.
//
// Example 1:
//
// Input: nums = [1,3,2,2,5,2,3,7]
// Output: 5
// Explanation: The longest harmonious subsequence is [3,2,2,2,3].
//
// Example 2:
//
// Input: nums = [1,2,3,4]
// Output: 2
//
// Example 3:
//
// Input: nums = [1,1,1,1]
// Output: 0
//
// Constraints:
//
// 1 <= nums.length <= 2 * 10^4
// -10^9 <= nums[i] <= 10^9
//
// Follow up: Could you solve it in O(n) time where n is the length of the given array?

struct Solution;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        let mut max = 0;
        for (num, count) in map.iter() {
            if let Some(next_count) = map.get(&(num + 1)) {
                max = max.max(count + next_count);
            }
        }

        max
    }
}

#[test]
fn test_find_lhs() {
    assert_eq!(Solution::find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]), 5);
    assert_eq!(Solution::find_lhs(vec![1, 2, 3, 4]), 2);
    assert_eq!(Solution::find_lhs(vec![1, 1, 1, 1]), 0);
}
