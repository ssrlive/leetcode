#![allow(dead_code)]

// 3097. Shortest Subarray With OR at Least K II
// https://leetcode.com/problems/shortest-subarray-with-or-at-least-k-ii/
// https://leetcode.cn/problems/shortest-subarray-with-or-at-least-k-ii/
//
// Medium
//
// You are given an array nums of non-negative integers and an integer k.
//
// An array is called special if the bitwise OR of all of its elements is at least k.
//
// Return the length of the shortest special non-empty subarray of nums, or return -1 if no special subarray exists.
//
// Example 1:
//
// Input: nums = [1,2,3], k = 2
//
// Output: 1
//
// Explanation:
//
// The subarray [3] has OR value of 3. Hence, we return 1.
//
// Example 2:
//
// Input: nums = [2,1,8], k = 10
//
// Output: 3
//
// Explanation:
//
// The subarray [2,1,8] has OR value of 11. Hence, we return 3.
//
// Example 3:
//
// Input: nums = [1,2], k = 0
//
// Output: 1
//
// Explanation:
//
// The subarray [1] has OR value of 1. Hence, we return 1.
//
// Constraints:
//
//     1 <= nums.length <= 2 * 10^5
//     0 <= nums[i] <= 10^9
//     0 <= k <= 10^9
//

struct Solution;

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 1;
        }
        let (mut left, mut current_or, mut min_len) = (0, 0, i32::MAX);
        let mut bit_counts = [0; 32];

        for (right, &num) in (1..).zip(nums.iter()) {
            if num >= k {
                return 1;
            }
            current_or |= num;

            (0..32).for_each(|bit| bit_counts[bit as usize] += ((num >> bit) & 1 == 1) as i32);

            while current_or >= k {
                min_len = min_len.min(right - left);
                (0..32).for_each(|bit| {
                    let to_remove = nums[left as usize];
                    let decrement = (to_remove >> bit) & 1 == 1;
                    if decrement && bit_counts[bit as usize] == 1 {
                        current_or ^= 1 << bit
                    }
                    bit_counts[bit as usize] -= decrement as i32;
                });

                left += 1;
            }
        }

        match min_len {
            i32::MAX => -1,
            _ => min_len,
        }
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3];
    let k = 2;
    let output = 1;
    assert_eq!(Solution::minimum_subarray_length(nums, k), output);

    let nums = vec![2, 1, 8];
    let k = 10;
    let output = 3;
    assert_eq!(Solution::minimum_subarray_length(nums, k), output);

    let nums = vec![1, 2];
    let k = 0;
    let output = 1;
    assert_eq!(Solution::minimum_subarray_length(nums, k), output);
}
