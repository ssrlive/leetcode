#![allow(dead_code)]

// 3467. Transform Array by Parity
// https://leetcode.com/problems/transform-array-by-parity/
// https://leetcode.cn/problems/transform-array-by-parity/
//
// Easy
//
// You are given an integer array nums. Transform nums by performing the following operations in the exact order specified:
//
// Replace each even number with 0.
// Replace each odd numbers with 1.
// Sort the modified array in non-decreasing order.
// Return the resulting array after performing these operations.
//
// Example 1:
//
// Input: nums = [4,3,2,1]
//
// Output: [0,0,1,1]
//
// Explanation:
//
// Replace the even numbers (4 and 2) with 0 and the odd numbers (3 and 1) with 1. Now, nums = [0, 1, 0, 1].
// After sorting nums in non-descending order, nums = [0, 0, 1, 1].
//
// Example 2:
//
// Input: nums = [1,5,1,4,2]
//
// Output: [0,0,1,1,1]
//
// Explanation:
//
// Replace the even numbers (4 and 2) with 0 and the odd numbers (1, 5 and 1) with 1. Now, nums = [1, 1, 1, 0, 0].
// After sorting nums in non-descending order, nums = [0, 0, 1, 1, 1].
//
// Constraints:
//
// 1 <= nums.length <= 100
// 1 <= nums[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn transform_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums.iter().map(|&x| x % 2).collect::<Vec<i32>>();
        nums.sort();
        nums
    }
}

#[test]
fn test() {
    let nums = vec![4, 3, 2, 1];
    let result = vec![0, 0, 1, 1];
    assert_eq!(Solution::transform_array(nums), result);

    let nums = vec![1, 5, 1, 4, 2];
    let result = vec![0, 0, 1, 1, 1];
    assert_eq!(Solution::transform_array(nums), result);
}
