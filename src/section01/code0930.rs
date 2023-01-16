#![allow(dead_code)]

// 930. Binary Subarrays With Sum
// https://leetcode.com/problems/binary-subarrays-with-sum/
// https://leetcode.cn/problems/binary-subarrays-with-sum/
//
// Given a binary array nums and an integer goal, return the number of non-empty subarrays with a sum goal.
//
// A subarray is a contiguous part of the array.
//
// Example 1:
//
// Input: nums = [1,0,1,0,1], goal = 2
// Output: 4
// Explanation: The 4 subarrays are bolded and underlined below:
// [1,0,1,0,1]
// [1,0,1,0,1]
// [1,0,1,0,1]
// [1,0,1,0,1]
//
// Example 2:
//
// Input: nums = [0,0,0,0,0], goal = 0
// Output: 15
//
// Constraints:
//
// - 1 <= nums.length <= 3 * 10^4
// - nums[i] is either 0 or 1.
// - 0 <= goal <= nums.length
//

struct Solution;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let z_counts = nums.split(|x| *x == 1).map(|s| s.len() as i32).collect::<Vec<_>>();

        if goal == 0 {
            z_counts.into_iter().map(|x| (x * (x + 1)) / 2).sum()
        } else {
            z_counts
                .windows(goal as usize + 1)
                .map(|slice| (slice.first().unwrap() + 1) * (slice.last().unwrap() + 1))
                .sum()
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2), 4);
    assert_eq!(Solution::num_subarrays_with_sum(vec![0, 0, 0, 0, 0], 0), 15);
}
