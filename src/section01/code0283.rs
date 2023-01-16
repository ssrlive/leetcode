#![allow(dead_code)]

// 283. Move Zeroes
// https://leetcode.com/problems/move-zeroes/
// https://leetcode.cn/problems/move-zeroes/
//
// Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
//
// Note that you must do this in-place without making a copy of the array.
//
// Example 1:
//
// Input: nums = [0,1,0,3,12]
// Output: [1,3,12,0,0]
//
// Example 2:
//
// Input: nums = [0]
// Output: [0]
//
// Constraints:
//
// - 1 <= nums.length <= 10^4
// - -2^31 <= nums[i] <= 2^31 - 1
//
// Follow up: Could you minimize the total number of operations done?
//

struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut j = 0;
        while j < nums.len() {
            if nums[j] != 0 {
                nums[i] = nums[j];
                i += 1;
            }
            j += 1;
        }
        while i < nums.len() {
            nums[i] = 0;
            i += 1;
        }
    }
}

#[test]
fn test_move_zeroes() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, vec![1, 3, 12, 0, 0]);

    let mut nums = vec![0];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, vec![0]);
}
