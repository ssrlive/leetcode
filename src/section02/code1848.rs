#![allow(dead_code)]

/*

// 1848. Minimum Distance to the Target Element
// https://leetcode.com/problems/minimum-distance-to-the-target-element/
// https://leetcode.cn/problems/minimum-distance-to-the-target-element/
//
// Easy
//
Given an integer array nums (0-indexed) and two integers target and start, find an index i such that nums[i] == target and abs(i - start) is minimized. Note that abs(x) is the absolute value of x.

Return abs(i - start).

It is guaranteed that target exists in nums.

Example 1:

Input: nums = [1,2,3,4,5], target = 5, start = 3
Output: 1
Explanation: nums[4] = 5 is the only value equal to target, so the answer is abs(4 - 3) = 1.

Example 2:

Input: nums = [1], target = 1, start = 0
Output: 0
Explanation: nums[0] = 1 is the only value equal to target, so the answer is abs(0 - 0) = 0.

Example 3:

Input: nums = [1,1,1,1,1,1,1,1,1,1], target = 1, start = 0
Output: 0
Explanation: Every value of nums is 1, but nums[0] minimizes abs(i - start), which is abs(0 - 0) = 0.

Constraints:

    1 <= nums.length <= 1000
    1 <= nums[i] <= 10^4
    0 <= start < nums.length
    target is in nums.
*/

struct Solution;

impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let mut min_dist = i32::MAX;
        for (i, val) in nums.iter().enumerate() {
            if *val == target {
                min_dist = min_dist.min((i as i32 - start).abs());
                if min_dist == 0 {
                    break;
                }
            }
        }
        min_dist
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3, 4, 5], 5, 3, 1),
        (vec![1], 1, 0, 0),
        (vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1, 0, 0),
    ];
    for (nums, target, start, expected) in cases {
        assert_eq!(Solution::get_min_distance(nums, target, start), expected);
    }
}
