#![allow(dead_code)]

/*

// 2016. Maximum Difference Between Increasing Elements
// https://leetcode.com/problems/maximum-difference-between-increasing-elements/
// https://leetcode.cn/problems/maximum-difference-between-increasing-elements/
//
// Easy
//
// Given a 0-indexed integer array nums of size n, find the maximum difference between nums[i] and nums[j] (i.e., nums[j] - nums[i]), such that 0 <= i < j < n and nums[i] < nums[j].

Return the maximum difference. If no such i and j exists, return -1.

Example 1:

Input: nums = [7,1,5,4]
Output: 4
Explanation:
The maximum difference occurs with i = 1 and j = 2, nums[j] - nums[i] = 5 - 1 = 4.
Note that with i = 1 and j = 0, the difference nums[j] - nums[i] = 7 - 1 = 6, but i > j, so it is not valid.

Example 2:

Input: nums = [9,4,3,2]
Output: -1
Explanation:
There is no i and j such that i < j and nums[i] < nums[j].

Example 3:

Input: nums = [1,5,2,10]
Output: 9
Explanation:
The maximum difference occurs with i = 0 and j = 3, nums[j] - nums[i] = 10 - 1 = 9.

Constraints:

    n == nums.length
    2 <= n <= 1000
    1 <= nums[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut diff = -1;
        let (mut max, mut min) = (nums[0], nums[0]);
        for &val in nums[1..].iter() {
            if val < min {
                min = val;
                max = val;
            } else if val > max {
                max = val;
                diff = diff.max(max - min);
            }
        }
        diff
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximum_difference(vec![7, 1, 5, 4]), 4);
    assert_eq!(Solution::maximum_difference(vec![9, 4, 3, 2]), -1);
    assert_eq!(Solution::maximum_difference(vec![1, 5, 2, 10]), 9);
}
