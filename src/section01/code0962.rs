#![allow(dead_code)]

// 962. Maximum Width Ramp
// https://leetcode.com/problems/maximum-width-ramp/
// https://leetcode.cn/problems/maximum-width-ramp/
//
// A ramp in an integer array nums is a pair (i, j) for which i < j and nums[i] <= nums[j]. The width of such a ramp is j - i.
//
// Given an integer array nums, return the maximum width of a ramp in nums. If there is no ramp in nums, return 0.
//
// Example 1:
//
// Input: nums = [6,0,8,2,1,5]
// Output: 4
// Explanation: The maximum width ramp is achieved at (i, j) = (1, 5): nums[1] = 0 and nums[5] = 5.
//
// Example 2:
//
// Input: nums = [9,8,1,0,1,9,4,0,4,1]
// Output: 7
// Explanation: The maximum width ramp is achieved at (i, j) = (2, 9): nums[2] = 1 and nums[9] = 1.
//
// Constraints:
//
// - 2 <= nums.length <= 5 * 10^4
// - 0 <= nums[i] <= 5 * 10^4
//

struct Solution;

impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let len = nums.len() as i32;
        let mut idx: Vec<i32> = (0..len).collect();
        idx.sort_by(|&i, &j| nums[i as usize].cmp(&nums[j as usize]));

        let mut answer = 0;
        let mut min_i = len;

        for i in idx {
            answer = answer.max(i - min_i);
            min_i = min_i.min(i);
        }

        answer
    }
}

#[test]
fn test() {
    let cases = vec![(vec![6, 0, 8, 2, 1, 5], 4), (vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1], 7)];
    for (nums, expected) in cases {
        assert_eq!(Solution::max_width_ramp(nums), expected);
    }
}
