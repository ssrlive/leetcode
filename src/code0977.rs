#![allow(dead_code)]

// 977. Squares of a Sorted Array
// https://leetcode.com/problems/squares-of-a-sorted-array/
// https://leetcode.cn/problems/squares-of-a-sorted-array/
//
// Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.
//
// Example 1:
//
// Input: nums = [-4,-1,0,3,10]
// Output: [0,1,9,16,100]
// Explanation: After squaring, the array becomes [16,1,0,9,100].
// After sorting, it becomes [0,1,9,16,100].
//
// Example 2:
//
// Input: nums = [-7,-3,2,3,11]
// Output: [4,9,9,49,121]
//
// Constraints:
//
// - 1 <= nums.length <= 10^4
// - -10^4 <= nums[i] <= 10^4
// - nums is sorted in non-decreasing order.
//
// Follow up: Squaring each element and sorting the new array is very trivial, could you find an O(n) solution using a different approach?
//

struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut result = nums.iter().map(|&x| x * x).collect::<Vec<i32>>();
        result.sort_unstable();
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::sorted_squares(vec![-4, -1, 0, 3, 10]), vec![0, 1, 9, 16, 100]);
    assert_eq!(Solution::sorted_squares(vec![-7, -3, 2, 3, 11]), vec![4, 9, 9, 49, 121]);
}
