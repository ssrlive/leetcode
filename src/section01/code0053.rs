#![allow(dead_code)]

// 53. Maximum Subarray
// https://leetcode.com/problems/maximum-subarray/
// https://leetcode.cn/problems/maximum-subarray/
//
// Given an integer array nums, find the subarray which has the largest sum and return its sum.
//
// Example 1:
//
// Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
// Output: 6
// Explanation: [4,-1,2,1] has the largest sum = 6.
//
// Example 2:
//
// Input: nums = [1]
// Output: 1
//
// Example 3:
//
// Input: nums = [5,4,-1,7,8]
// Output: 23
//
// Constraints:
//
// - 1 <= nums.length <= 10^5
// - -10^4 <= nums[i] <= 10^4
//
// Follow up: If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.
//

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut cur, mut max) = (nums[0], nums[0]);
        for item in nums.iter().skip(1) {
            cur = match cur < 0 {
                true => *item,
                _ => cur + *item,
            };
            max = max.max(cur);
        }
        max
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    assert_eq!(Solution::max_sub_array(vec![1]), 1);
    assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
}
