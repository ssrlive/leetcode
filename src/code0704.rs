#![allow(dead_code)]

// 704. Binary Search
// https://leetcode.com/problems/binary-search/
//
// Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums.
// If target exists, then return its index. Otherwise, return -1.
//
// You must write an algorithm with O(log n) runtime complexity.
//
// Example 1:
//
// Input: nums = [-1,0,3,5,9,12], target = 9
// Output: 4
// Explanation: 9 exists in nums and its index is 4
//
// Example 2:
//
// Input: nums = [-1,0,3,5,9,12], target = 2
// Output: -1
// Explanation: 2 does not exist in nums so return -1
//
// Constraints:
//
// - 1 <= nums.length <= 10^4
// - -10^4 < nums[i], target < 10^4
// - All the integers in nums are unique.
// - nums is sorted in ascending order.
//
// Follow up: Could you implement the solution with runtime complexity O(log n)?

struct Solution;

impl Solution {
    fn search(nums: Vec<i32>, target: i32) -> i32 {
        fn _search(nums: Vec<i32>, target: i32) -> Option<i32> {
            let mut left = 0;
            let mut right = nums.len() as i32 - 1;
            while left <= right {
                let mid = left + (right - left) / 2;
                match target.cmp(nums.get(mid as usize)?) {
                    std::cmp::Ordering::Equal => return Some(mid),
                    std::cmp::Ordering::Less => right = mid - 1,
                    std::cmp::Ordering::Greater => left = mid + 1,
                }
            }
            None
        }
        _search(nums, target).unwrap_or(-1)
    }
}

#[test]
fn test() {
    let nums = vec![5];
    let target = -5;
    let res = -1;
    assert_eq!(Solution::search(nums, target), res);

    let nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 9;
    let res = 4;
    assert_eq!(Solution::search(nums, target), res);

    let nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 2;
    let res = -1;
    assert_eq!(Solution::search(nums, target), res);
}
