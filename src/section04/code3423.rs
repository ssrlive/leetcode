#![allow(dead_code)]

// 3423. Maximum Difference Between Adjacent Elements in a Circular Array
// https://leetcode.com/problems/maximum-difference-between-adjacent-elements-in-a-circular-array/
// https://leetcode.cn/problems/maximum-difference-between-adjacent-elements-in-a-circular-array/
//
// Easy
//
// Given a circular array nums, find the maximum absolute difference between adjacent elements.
//
// Note: In a circular array, the first and last elements are adjacent.
//
// Example 1:
//
// Input: nums = [1,2,4]
//
// Output: 3
//
// Explanation:
//
// Because nums is circular, nums[0] and nums[2] are adjacent. They have the maximum absolute difference of |4 - 1| = 3.
//
// Example 2:
//
// Input: nums = [-5,-10,-5]
//
// Output: 5
//
// Explanation:
//
// The adjacent elements nums[0] and nums[1] have the maximum absolute difference of |-5 - (-10)| = 5.
//
// Constraints:
//
//     2 <= nums.length <= 100
//     -100 <= nums[i] <= 100
//

struct Solution;

impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max_diff = 0;

        for i in 0..n {
            let diff = (nums[i] - nums[(i + 1) % n]).abs();
            max_diff = max_diff.max(diff);
        }

        max_diff
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 4];
    let output = 3;
    assert_eq!(Solution::max_adjacent_distance(nums), output);

    let nums = vec![-5, -10, -5];
    let output = 5;
    assert_eq!(Solution::max_adjacent_distance(nums), output);
}
