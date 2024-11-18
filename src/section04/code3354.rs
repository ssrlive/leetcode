#![allow(dead_code)]

// 3354. Make Array Elements Equal to Zero
// https://leetcode.com/problems/make-array-elements-equal-to-zero/
// https://leetcode.cn/problems/make-array-elements-equal-to-zero/
//
// Easy
//
// You are given an integer array nums.
//
// Start by selecting a starting position curr such that nums[curr] == 0, and choose a movement direction of either left or right.
//
// After that, you repeat the following process:
//
// If curr is out of the range [0, n - 1], this process ends.
// If nums[curr] == 0, move in the current direction by incrementing curr if you are moving right, or decrementing curr if you are moving left.
// Else if nums[curr] > 0:
// Decrement nums[curr] by 1.
// Reverse your movement direction (left becomes right and vice versa).
// Take a step in your new direction.
// A selection of the initial position curr and movement direction is considered valid if every element in nums becomes 0 by the end of the process.
//
// Return the number of possible valid selections.
//
// Example 1:
//
// Input: nums = [1,0,2,0,3]
//
// Output: 2
//
// Explanation:
//
// The only possible valid selections are the following:
//
// Choose curr = 3, and a movement direction to the left.
// [1,0,2,0,3] -> [1,0,2,0,3] -> [1,0,1,0,3] -> [1,0,1,0,3] -> [1,0,1,0,2] -> [1,0,1,0,2] -> [1,0,0,0,2] -> [1,0,0,0,2] -> [1,0,0,0,1]
//             -> [1,0,0,0,1] -> [1,0,0,0,1] -> [1,0,0,0,1] -> [0,0,0,0,1] -> [0,0,0,0,1] -> [0,0,0,0,1] -> [0,0,0,0,1] -> [0,0,0,0,0].
//
// Choose curr = 3, and a movement direction to the right.
// [1,0,2,0,3] -> [1,0,2,0,3] -> [1,0,2,0,2] -> [1,0,2,0,2] -> [1,0,1,0,2] -> [1,0,1,0,2] -> [1,0,1,0,1] -> [1,0,1,0,1] -> [1,0,0,0,1]
//             -> [1,0,0,0,1] -> [1,0,0,0,0] -> [1,0,0,0,0] -> [1,0,0,0,0] -> [1,0,0,0,0] -> [0,0,0,0,0].
//
// Example 2:
//
// Input: nums = [2,3,4,0,4,1,0]
//
// Output: 0
//
// Explanation:
//
// There are no possible valid selections.
//
// Constraints:
//
// 1 <= nums.length <= 100
// 0 <= nums[i] <= 100
// There is at least one element i where nums[i] == 0.
//

struct Solution;

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut prefix_sum, mut suffix_sum) = (nums.clone(), nums.clone());
        for i in 1..n {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i];
        }
        for i in (0..n - 1).rev() {
            suffix_sum[i] = suffix_sum[i + 1] + nums[i];
        }
        let mut answer = 0;
        for i in 0..n {
            if nums[i] == 0 && (prefix_sum[i] - suffix_sum[i]).abs() <= 1 {
                answer += 2 - (prefix_sum[i] - suffix_sum[i]).abs();
            }
        }
        answer
    }
}

#[test]
fn test() {
    let nums = vec![1, 0, 2, 0, 3];
    let res = 2;
    assert_eq!(Solution::count_valid_selections(nums), res);

    let nums = vec![2, 3, 4, 0, 4, 1, 0];
    let res = 0;
    assert_eq!(Solution::count_valid_selections(nums), res);
}
