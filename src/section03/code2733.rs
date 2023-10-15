#![allow(dead_code)]

// 2733. Neither Minimum nor Maximum
// https://leetcode.com/problems/neither-minimum-nor-maximum/
// https://leetcode.cn/problems/neither-minimum-nor-maximum/
//
// Easy
//
// Given an integer array nums containing distinct positive integers, find and return any number from the array
// that is neither the minimum nor the maximum value in the array, or -1 if there is no such number.
//
// Return the selected integer.
//
// Example 1:
//
// Input: nums = [3,2,1,4]
// Output: 2
// Explanation: In this example, the minimum value is 1 and the maximum value is 4. Therefore, either 2 or 3 can be valid answers.
//
// Example 2:
//
// Input: nums = [1,2]
// Output: -1
// Explanation: Since there is no number in nums that is neither the maximum nor the minimum,
// we cannot select a number that satisfies the given condition. Therefore, there is no answer.
//
// Example 3:
//
// Input: nums = [2,1,3]
// Output: 2
// Explanation: Since 2 is neither the maximum nor the minimum value in nums, it is the only valid answer.
//
// Constraints:
//
//     1 <= nums.length <= 100
//     1 <= nums[i] <= 100
//     All values in nums are distinct
//

struct Solution;

impl Solution {
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        let mut lo = nums[0];
        let mut hi = nums[0];
        for &x in nums.iter().skip(1) {
            if lo == hi {
                lo = lo.min(x);
                hi = hi.max(x);
            } else if x != lo && x != hi {
                return x.max(lo).min(hi);
            }
        }
        -1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_non_min_or_max(vec![3, 2, 1, 4]), 2);
    assert_eq!(Solution::find_non_min_or_max(vec![1, 2]), -1);
    assert_eq!(Solution::find_non_min_or_max(vec![2, 1, 3]), 2);
}
