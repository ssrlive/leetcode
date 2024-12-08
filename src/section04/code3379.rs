#![allow(dead_code)]

// 3379. Transformed Array
// https://leetcode.com/problems/transformed-array/
// https://leetcode.cn/problems/transformed-array/
//
// Easy
//
// You are given an integer array nums that represents a circular array. Your task is to create a new array
// result of the same size, following these rules:
//
// For each index i (where 0 <= i < nums.length), perform the following independent actions:
// If nums[i] > 0: Start at index i and move nums[i] steps to the right in the circular array.
// Set result[i] to the value of the index where you land.
// If nums[i] < 0: Start at index i and move abs(nums[i]) steps to the left in the circular array.
// Set result[i] to the value of the index where you land.
// If nums[i] == 0: Set result[i] to nums[i].
// Return the new array result.
//
// Note: Since nums is circular, moving past the last element wraps around to the beginning,
// and moving before the first element wraps back to the end.
//
// Example 1:
//
// Input: nums = [3,-2,1,1]
//
// Output: [1,1,1,3]
//
// Explanation:
//
// For nums[0] that is equal to 3, If we move 3 steps to right, we reach nums[3]. So result[0] should be 1.
// For nums[1] that is equal to -2, If we move 2 steps to left, we reach nums[3]. So result[1] should be 1.
// For nums[2] that is equal to 1, If we move 1 step to right, we reach nums[3]. So result[2] should be 1.
// For nums[3] that is equal to 1, If we move 1 step to right, we reach nums[0]. So result[3] should be 3.
//
// Example 2:
//
// Input: nums = [-1,4,-1]
//
// Output: [-1,-1,4]
//
// Explanation:
//
// For nums[0] that is equal to -1, If we move 1 step to left, we reach nums[2]. So result[0] should be -1.
// For nums[1] that is equal to 4, If we move 4 steps to right, we reach nums[2]. So result[1] should be -1.
// For nums[2] that is equal to -1, If we move 1 step to left, we reach nums[1]. So result[2] should be 4.
//
// Constraints:
//
// 1 <= nums.length <= 100
// -100 <= nums[i] <= 100
//

struct Solution;

impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![0; n];

        for i in 0..n {
            res[i] = nums[((i as i32 + nums[i]) % (n as i32) + (n as i32)) as usize % n];
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![3, -2, 1, 1];
    let res = vec![1, 1, 1, 3];
    assert_eq!(Solution::construct_transformed_array(nums), res);

    let nums = vec![-1, 4, -1];
    let res = vec![-1, -1, 4];
    assert_eq!(Solution::construct_transformed_array(nums), res);
}
