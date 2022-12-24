#![allow(dead_code)]

// 75. Sort Colors
// https://leetcode.com/problems/sort-colors/
// https://leetcode.cn/problems/sort-colors/
//
// Given an array nums with n objects colored red, white, or blue, sort them in-place so that
// objects of the same color are adjacent, with the colors in the order red, white, and blue.
//
// We will use the integers 0, 1, and 2 to represent the color red, white, and blue, respectively.
//
// You must solve this problem without using the library's sort function.
//
// Example 1:
//
// Input: nums = [2,0,2,1,1,0]
// Output: [0,0,1,1,2,2]
//
// Example 2:
//
// Input: nums = [2,0,1]
// Output: [0,1,2]
//
// Constraints:
//
// - n == nums.length
// - 1 <= n <= 300
// - nums[i] is either 0, 1, or 2.
//
// Follow up: Could you come up with a one-pass algorithm using only constant extra space?
//

struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut i, mut j, mut k) = (0, 0, nums.len() as i32 - 1);
        while j <= k {
            match nums[j as usize] {
                0 => {
                    nums.swap(i as usize, j as usize);
                    i += 1;
                    j += 1;
                }
                2 => {
                    nums.swap(j as usize, k as usize);
                    k -= 1;
                }
                _ => {
                    j += 1;
                }
            }
        }
    }
}

#[test]
fn test() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    Solution::sort_colors(&mut nums);
    assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);

    let mut nums = vec![2, 0, 1];
    Solution::sort_colors(&mut nums);
    assert_eq!(nums, vec![0, 1, 2]);
}
