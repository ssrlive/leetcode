#![allow(dead_code)]

// 462. Minimum Moves to Equal Array Elements II
// https://leetcode.com/problems/minimum-moves-to-equal-array-elements-ii/
// https://leetcode.cn/problems/minimum-moves-to-equal-array-elements-ii/
//
// Given an integer array nums of size n, return the minimum number of moves required to make all array elements equal.
//
// In one move, you can increment or decrement an element of the array by 1.
//
// Test cases are designed so that the answer will fit in a 32-bit integer.
//
// Example 1:
//
// Input: nums = [1,2,3]
// Output: 2
// Explanation:
// Only two moves are needed (remember each move increments or decrements one element):
// [1,2,3]  =>  [2,2,3]  =>  [2,2,2]
//
// Example 2:
//
// Input: nums = [1,10,2,9]
// Output: 16
//
// Constraints:
//
// - n == nums.length
// - 1 <= nums.length <= 10^5
// - -10^9 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut count = 0;
        let mut i = 0;
        let mut j = nums.len() - 1;
        while i < j {
            count += nums[j] - nums[i];
            i += 1;
            j -= 1;
        }
        count
    }
}

#[test]
fn test_min_moves2() {
    assert_eq!(Solution::min_moves2(vec![1, 2, 3]), 2);
    assert_eq!(Solution::min_moves2(vec![1, 10, 2, 9]), 16);
}
