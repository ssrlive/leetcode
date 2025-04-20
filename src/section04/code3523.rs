#![allow(dead_code)]

// 3523. Make Array Non-decreasing
// https://leetcode.com/problems/make-array-non-decreasing/
// https://leetcode.cn/problems/make-array-non-decreasing/
//
// Medium
//
// You are given an integer array nums. In one operation, you can select a
//
// and replace it with a single element equal to its maximum value.
//
// Return the maximum possible size of the array after performing zero or more operations such that the resulting array is non-decreasing.
//
// Example 1:
//
// Input: nums = [4,2,5,3,5]
//
// Output: 3
//
// Explanation:
//
// One way to achieve the maximum size is:
//
//     Replace subarray nums[1..2] = [2, 5] with 5 → [4, 5, 3, 5].
//     Replace subarray nums[2..3] = [3, 5] with 5 → [4, 5, 5].
//
// The final array [4, 5, 5] is non-decreasing with size 3.
//
// Example 2:
//
// Input: nums = [1,2,3]
//
// Output: 3
//
// Explanation:
//
// No operation is needed as the array [1,2,3] is already non-decreasing.
//
// Constraints:
//
//     1 <= nums.length <= 2 * 10^5
//     1 <= nums[i] <= 2 * 10^5
//

struct Solution;

impl Solution {
    pub fn maximum_possible_size(nums: Vec<i32>) -> i32 {
        let (mut size, mut prev) = (0, -1);
        nums.iter().for_each(|&num| {
            if num >= prev {
                prev = num;
                size += 1;
            }
        });
        size
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximum_possible_size(vec![4, 2, 5, 3, 5]), 3);
    assert_eq!(Solution::maximum_possible_size(vec![1, 2, 3]), 3);
}
