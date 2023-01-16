#![allow(dead_code)]

// 795. Number of Subarrays with Bounded Maximum
// https://leetcode.com/problems/number-of-subarrays-with-bounded-maximum/
// https://leetcode.cn/problems/number-of-subarrays-with-bounded-maximum/
//
// Given an integer array nums and two integers left and right, return the number of contiguous non-empty subarrays
// such that the value of the maximum array element in that subarray is in the range [left, right].
//
// The test cases are generated so that the answer will fit in a 32-bit integer.
//
// Example 1:
//
// Input: nums = [2,1,4,3], left = 2, right = 3
// Output: 3
// Explanation: There are three subarrays that meet the requirements: [2], [2, 1], [3].
//
// Example 2:
//
// Input: nums = [2,9,2,5,6], left = 2, right = 8
// Output: 7
//
// Constraints:
//
// - 1 <= nums.length <= 10^5
// - 0 <= nums[i] <= 10^9
// - 0 <= left <= right <= 10^9
//

struct Solution;

impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let mut result = 0;
        let mut start = 0;
        let mut end = 0;
        let mut count = 0;
        while end < nums.len() {
            if nums[end] >= left && nums[end] <= right {
                count = end - start + 1;
                result += count;
            } else if nums[end] < left {
                result += count;
            } else {
                start = end + 1;
                count = 0;
            }
            end += 1;
        }
        result as i32
    }
}

#[test]
fn test() {
    let nums = vec![2, 1, 4, 3];
    assert_eq!(Solution::num_subarray_bounded_max(nums, 2, 3), 3);

    let nums = vec![2, 9, 2, 5, 6];
    assert_eq!(Solution::num_subarray_bounded_max(nums, 2, 8), 7);
}
