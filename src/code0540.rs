#![allow(dead_code)]

// 540. Single Element in a Sorted Array
// https://leetcode.com/problems/single-element-in-a-sorted-array/
// https://leetcode.cn/problems/single-element-in-a-sorted-array/
//
// You are given a sorted array consisting of only integers where every element appears exactly twice,
// except for one element which appears exactly once.
//
// Return the single element that appears only once.
//
// Your solution must run in O(log n) time and O(1) space.
//
// Example 1:
//
// Input: nums = [1,1,2,3,3,4,4,8,8]
// Output: 2
//
// Example 2:
//
// Input: nums = [3,3,7,7,10,11,11]
// Output: 10
//
// Constraints:
//
// - 1 <= nums.length <= 10^5
// - 0 <= nums[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = (left + right) / 2;
            if mid % 2 == 1 {
                if nums[mid] == nums[mid - 1] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            } else if nums[mid] == nums[mid + 1] {
                left = mid + 2;
            } else {
                right = mid;
            }
        }
        nums[left]
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 2, 3, 3, 4, 4, 8, 8];
    assert_eq!(Solution::single_non_duplicate(nums), 2);
    let nums = vec![3, 3, 7, 7, 10, 11, 11];
    assert_eq!(Solution::single_non_duplicate(nums), 10);
}
