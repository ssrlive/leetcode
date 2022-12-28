#![allow(dead_code)]

// 2441. Largest Positive Integer That Exists With Its Negative
// https://leetcode.com/problems/largest-positive-integer-that-exists-with-its-negative/
// https://leetcode.cn/problems/largest-positive-integer-that-exists-with-its-negative/
//
// Given an integer array nums that does not contain any zeros, find the largest positive integer k such that -k also exists in the array.
//
// Return the positive integer k. If there is no such integer, return -1.
//
// Example 1:
//
// Input: nums = [-1,2,-3,3]
// Output: 3
// Explanation: 3 is the only valid k we can find in the array.
//
// Example 2:
//
// Input: nums = [-1,10,6,7,-7,1]
// Output: 7
// Explanation: Both 1 and 7 have their corresponding negative values in the array. 7 has a larger value.
//
// Example 3:
//
// Input: nums = [-10,8,6,7,-2,-3]
// Output: -1
// Explanation: There is no a single valid k, we return -1.
//
// Constraints:
//
// - 1 <= nums.length <= 1000
// - -1000 <= nums[i] <= 1000
// - nums[i] != 0
//

struct Solution;

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut neg: HashSet<i32> = HashSet::new();
        let mut pos: HashSet<i32> = HashSet::new();

        for n in nums {
            if n < 0 {
                neg.insert(n.abs());
            } else {
                pos.insert(n);
            }
        }

        *pos.intersection(&neg).max().unwrap_or(&-1)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_max_k(vec![-1, 2, -3, 3]), 3);
    assert_eq!(Solution::find_max_k(vec![-1, 10, 6, 7, -7, 1]), 7);
    assert_eq!(Solution::find_max_k(vec![-10, 8, 6, 7, -2, -3]), -1);
}
