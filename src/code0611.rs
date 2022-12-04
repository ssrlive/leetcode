#![allow(dead_code)]

// 611. Valid Triangle Number
// https://leetcode.com/problems/valid-triangle-number/
//
// Given an integer array nums, return the number of triplets chosen from the array
// that can make triangles if we take them as side lengths of a triangle.
//
// Example 1:
//
// Input: nums = [2,2,3,4]
// Output: 3
// Explanation: Valid combinations are:
// 2,3,4 (using the first 2)
// 2,3,4 (using the second 2)
// 2,2,3
//
// Example 2:
//
// Input: nums = [4,2,3,4]
// Output: 4
//
// Constraints:
//
// - 1 <= nums.length <= 1000
// - 0 <= nums[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut count = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let mut k = j + 1;
                while k < nums.len() && nums[i] + nums[j] > nums[k] {
                    k += 1;
                }
                count += k - j - 1;
            }
        }
        count as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::triangle_number(vec![2, 2, 3, 4]), 3);
    assert_eq!(Solution::triangle_number(vec![4, 2, 3, 4]), 4);
}
