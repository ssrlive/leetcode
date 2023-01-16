#![allow(dead_code)]

// 35. Search Insert Position
// https://leetcode.com/problems/search-insert-position/description/
// https://leetcode.cn/problems/search-insert-position/description/
//
// Given a sorted array of distinct integers and a target value, return the index if the target is found.
// If not, return the index where it would be if it were inserted in order.
//
// You must write an algorithm with O(log n) runtime complexity.
//
// Example 1:
//
// Input: nums = [1,3,5,6], target = 5
// Output: 2
//
// Example 2:
//
// Input: nums = [1,3,5,6], target = 2
// Output: 1
//
// Example 3:
//
// Input: nums = [1,3,5,6], target = 7
// Output: 4
//
// Constraints:
//
// - 1 <= nums.length <= 10^4
// - -10^4 <= nums[i] <= 10^4
// - nums contains distinct values sorted in ascending order.
// - -10^4 <= target <= 10^4
//

struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
}
