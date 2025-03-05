#![allow(dead_code)]

// 33. Search in Rotated Sorted Array
// https://leetcode.com/problems/longest-valid-parentheses/description/
// https://leetcode.cn/problems/longest-valid-parentheses/description/
//
// There is an integer array nums sorted in ascending order (with distinct values).
//
// Prior to being passed to your function, nums is possibly rotated at an unknown pivot index k (1 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].
//
// Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.
//
// You must write an algorithm with O(log n) runtime complexity.
//
// Example 1:
//
// Input: nums = [4,5,6,7,0,1,2], target = 0
// Output: 4
//
// Example 2:
//
// Input: nums = [4,5,6,7,0,1,2], target = 3
// Output: -1
//
// Example 3:
//
// Input: nums = [1], target = 0
// Output: -1
//
// Constraints:
//
// - 1 <= nums.length <= 5000
// - -10^4 <= nums[i] <= 10^4
// - All values of nums are unique.
// - nums is an ascending array that is possibly rotated.
// - -10^4 <= target <= 10^4
//

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        fn bin_search_target_logn(nums: &[i32], target: i32) -> i32 {
            let length = nums.len();
            if length == 0 {
                return -1;
            }
            let mid = if length == 1 { 0 } else { length / 2 };
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[0] < nums[mid] {
                if target <= nums[mid] && target >= nums[0] {
                    bin_search_target_logn(&nums[0..mid], target)
                } else {
                    let m = bin_search_target_logn(&nums[mid + 1..length], target);
                    if m > -1 { m + mid as i32 + 1 } else { -1 }
                }
            } else if target <= nums[length - 1] && target >= nums[mid] {
                let m = bin_search_target_logn(&nums[mid + 1..length], target);
                if m > -1 { m + mid as i32 + 1 } else { -1 }
            } else {
                bin_search_target_logn(&nums[0..mid], target)
            }
        }
        bin_search_target_logn(&nums, target)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    assert_eq!(Solution::search(vec![1], 0), -1);
    assert_eq!(Solution::search(vec![1, 3], 0), -1);
    assert_eq!(Solution::search(vec![1, 3], 1), 0);
    assert_eq!(Solution::search(vec![1, 3], 3), 1);
    assert_eq!(Solution::search(vec![1, 3, 5], 0), -1);
    assert_eq!(Solution::search(vec![1, 3, 5], 1), 0);
    assert_eq!(Solution::search(vec![1, 3, 5], 3), 1);
    assert_eq!(Solution::search(vec![1, 3, 5], 5), 2);
    assert_eq!(Solution::search(vec![1, 3, 5], 2), -1);
    assert_eq!(Solution::search(vec![1, 3, 5], 4), -1);
    assert_eq!(Solution::search(vec![1, 3, 5], 6), -1);
    assert_eq!(Solution::search(vec![1, 3, 5, 7], 0), -1);
    assert_eq!(Solution::search(vec![1, 3, 5, 7], 1), 0);
    assert_eq!(Solution::search(vec![1, 3, 5, 7], 3), 1);
    assert_eq!(Solution::search(vec![1, 3, 5, 7], 5), 2);
    assert_eq!(Solution::search(vec![1, 3, 5, 7], 7), 3);
    assert_eq!(Solution::search(vec![1, 3, 5, 7], 2), -1);
    assert_eq!(Solution::search(vec![1, 3, 5, 7], 4), -1);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
}
