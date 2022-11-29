#![allow(dead_code)]

// 453. Minimum Moves to Equal Array Elements
// https://leetcode.com/problems/minimum-moves-to-equal-array-elements/
//
// Given an integer array nums of size n, return the minimum number of moves required to make all array elements equal.
//
// In one move, you can increment n - 1 elements of the array by 1.
//
// Example 1:
//
// Input: nums = [1,2,3]
// Output: 3
// Explanation: Only three moves are needed (remember each move increments two elements):
// [1,2,3]  =>  [2,3,3]  =>  [3,4,3]  =>  [4,4,4]
//
// Example 2:
//
// Input: nums = [1,1,1]
// Output: 0
//
// Constraints:
//
// - n == nums.length
// - 1 <= nums.length <= 105
// - -109 <= nums[i] <= 109
// - The answer is guaranteed to fit in a 32-bit integer.
//

struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut v = nums;
        v.sort();
        let mut sum = 0;
        for i in 1..v.len() {
            sum += v[i] - v[0];
        }
        sum
    }
}

#[test]
fn test_min_moves() {
    assert_eq!(Solution::min_moves(vec![1, 2, 3]), 3);
    assert_eq!(Solution::min_moves(vec![1, 1, 1]), 0);
}
