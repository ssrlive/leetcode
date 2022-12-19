#![allow(dead_code)]

// 485. Max Consecutive Ones
// https://leetcode.com/problems/max-consecutive-ones/
// https://leetcode.cn/problems/max-consecutive-ones/
//
// Given a binary array nums, return the maximum number of consecutive 1's in the array.
//
// Example 1:
//
// Input: nums = [1,1,0,1,1,1]
// Output: 3
// Explanation: The first two digits or the last three digits are consecutive 1s. The maximum number of consecutive 1s is 3.
//
// Example 2:
//
// Input: nums = [1,0,1,1,0,1]
// Output: 2
//
// Constraints:
//
// - 1 <= nums.length <= 10^5
// - nums[i] is either 0 or 1.
//
// Follow up: Could you find the maximum number of consecutive 1's without using any built-in function?

struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut count = 0;
        for n in nums {
            if n == 1 {
                count += 1;
            } else {
                if count > max {
                    max = count;
                }
                count = 0;
            }
        }
        if count > max {
            max = count;
        }
        max
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
    assert_eq!(Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]), 2);
}
