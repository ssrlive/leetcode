#![allow(dead_code)]

// 1413. Minimum Value to Get Positive Step by Step Sum
// https://leetcode.com/problems/minimum-value-to-get-positive-step-by-step-sum/
// https://leetcode.cn/problems/minimum-value-to-get-positive-step-by-step-sum/
//
// Easy
//
// Given an array of integers nums, you start with an initial positive value startValue.
//
// In each iteration, you calculate the step by step sum of startValue plus elements in nums (from left to right).
//
// Return the minimum positive value of startValue such that the step by step sum is never less than 1.
//
// Example 1:
//
// Input: nums = [-3,2,-3,4,2]
// Output: 5
// Explanation: If you choose startValue = 4, in the third iteration your step by step sum is less than 1.
// step by step sum
// startValue = 4 | startValue = 5 | nums
//   (4 -3 ) = 1  | (5 -3 ) = 2    |  -3
//   (1 +2 ) = 3  | (2 +2 ) = 4    |   2
//   (3 -3 ) = 0  | (4 -3 ) = 1    |  -3
//   (0 +4 ) = 4  | (1 +4 ) = 5    |   4
//   (4 +2 ) = 6  | (5 +2 ) = 7    |   2
//
// Example 2:
//
// Input: nums = [1,2]
// Output: 1
// Explanation: Minimum start value should be positive.
//
// Example 3:
//
// Input: nums = [1,-2,-3]
// Output: 5
//
// Constraints:
//
// -    1 <= nums.length <= 100
// -    -100 <= nums[i] <= 100
//

struct Solution;

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut min = 0;
        let mut sum = 0;
        for num in nums {
            sum += num;
            if sum < min {
                min = sum;
            }
        }
        1 - min
    }
}

#[test]
fn test() {
    let cases = vec![(vec![-3, 2, -3, 4, 2], 5), (vec![1, 2], 1), (vec![1, -2, -3], 5)];
    for (nums, expected) in cases {
        assert_eq!(Solution::min_start_value(nums), expected);
    }
}
