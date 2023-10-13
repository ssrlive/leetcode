#![allow(dead_code)]

// 217. Contains Duplicate
// https://leetcode.com/problems/contains-duplicate/
// https://leetcode.cn/problems/contains-duplicate/
//
// Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.
//
// Example 1:
// Input: nums = [1,2,3,1]
// Output: true
//
// Example 2:
// Input: nums = [1,2,3,4]
// Output: false
//
// Example 3:
// Input: nums = [1,1,1,3,3,4,3,2,4,2]
// Output: true
//
// Constraints:
// 1 <= nums.length <= 10^5
// -10^9 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::new();
        for num in nums {
            if set.contains(&num) {
                return true;
            }
            set.insert(num);
        }
        false
    }
}

#[test]
fn test_contains_duplicate() {
    assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
    assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
    assert!(Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
}
