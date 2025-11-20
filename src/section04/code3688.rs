#![allow(dead_code)]

// 3688. Bitwise OR of Even Numbers in an Array
// https://leetcode.com/problems/bitwise-or-of-even-numbers-in-an-array/
// https://leetcode.cn/problems/bitwise-or-of-even-numbers-in-an-array/
//
// Easy
//
// You are given an integer array nums.
//
// Return the bitwise OR of all even numbers in the array.
//
// If there are no even numbers in nums, return 0.
//
// Example 1:
//
// Input: nums = [1,2,3,4,5,6]
//
// Output: 6
//
// Explanation:
//
// The even numbers are 2, 4, and 6. Their bitwise OR equals 6.
//
// Example 2:
//
// Input: nums = [7,9,11]
//
// Output: 0
//
// Explanation:
//
// There are no even numbers, so the result is 0.
//
// Example 3:
//
// Input: nums = [1,8,16]
//
// Output: 24
//
// Explanation:
//
// The even numbers are 8 and 16. Their bitwise OR equals 24.
//
// Constraints:
//
// 1 <= nums.length <= 100
// 1 <= nums[i] <= 100
//

struct Solution;

impl Solution {
    pub fn even_number_bitwise_o_rs(nums: Vec<i32>) -> i32 {
        nums.iter().filter(|&x| x % 2 == 0).fold(0, |acc, x| acc | x)
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(Solution::even_number_bitwise_o_rs(nums), 6);

    let nums = vec![7, 9, 11];
    assert_eq!(Solution::even_number_bitwise_o_rs(nums), 0);

    let nums = vec![1, 8, 16];
    assert_eq!(Solution::even_number_bitwise_o_rs(nums), 24);
}
