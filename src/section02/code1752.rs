#![allow(dead_code)]

/*

// 1752. Check if Array Is Sorted and Rotated
// https://leetcode.com/problems/check-if-array-is-sorted-and-rotated/
// https://leetcode.cN/problems/check-if-array-is-sorted-and-rotated/
//
// Easy
//
// Given an array nums, return true if the array was originally sorted in non-decreasing order, then rotated some number of positions (including zero). Otherwise, return false.

There may be duplicates in the original array.

Note: An array A rotated by x positions results in an array B of the same length such that A[i] == B[(i+x) % A.length], where % is the modulo operation.

Example 1:

Input: nums = [3,4,5,1,2]
Output: true
Explanation: [1,2,3,4,5] is the original sorted array.
You can rotate the array by x = 3 positions to begin on the the element of value 3: [3,4,5,1,2].

Example 2:

Input: nums = [2,1,3,4]
Output: false
Explanation: There is no sorted array once rotated that can make nums.

Example 3:

Input: nums = [1,2,3]
Output: true
Explanation: [1,2,3] is the original sorted array.
You can rotate the array by x = 0 positions (i.e. no rotation) to make nums.

Constraints:

    1 <= nums.length <= 100
    1 <= nums[i] <= 100
*/

struct Solution;

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut count = 0;
        for i in 0..nums.len() {
            if nums[i] > nums[(i + 1) % nums.len()] {
                count += 1;
            }
        }
        count <= 1
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (vec![3, 4, 5, 1, 2], true),
        (vec![2, 1, 3, 4], false),
        (vec![1, 2, 3], true),
    ];
    for (nums, expected) in cases {
        assert_eq!(Solution::check(nums), expected);
    }
}
