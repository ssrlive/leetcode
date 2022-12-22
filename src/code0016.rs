#![allow(dead_code)]

// 16. 3Sum Closest
// https://leetcode.com/problems/3sum-closest/
// https://leetcode.cn/problems/3sum-closest/
//
// Given an integer array nums of length n and an integer target, find three integers in nums such that the sum is closest to target.
//
// Return the sum of the three integers.
//
// You may assume that each input would have exactly one solution.
//
// Example 1:
//
// Input: nums = [-1,2,1,-4], target = 1
// Output: 2
// Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
//
// Example 2:
//
// Input: nums = [0,0,0], target = 1
// Output: 0
// Explanation: The sum that is closest to the target is 0. (0 + 0 + 0 = 0).
//
// Constraints:
//
// 3 <= nums.length <= 500
// -1000 <= nums[i] <= 1000
// -10^4 <= target <= 10^4
//

struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut closest = nums[0] + nums[1] + nums[2];
        for i in 0..nums.len() {
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if (sum - target).abs() < (closest - target).abs() {
                    closest = sum;
                }
                if sum < target {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
        closest
    }
}

#[test]
fn test() {
    let nums = vec![-1, 2, 1, -4];
    let target = 1;
    let output = 2;
    assert_eq!(Solution::three_sum_closest(nums, target), output);

    let nums = vec![0, 0, 0];
    let target = 1;
    let output = 0;
    assert_eq!(Solution::three_sum_closest(nums, target), output);
}
