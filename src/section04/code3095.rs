#![allow(dead_code)]

// 3095. Shortest Subarray With OR at Least K I
// https://leetcode.com/problems/shortest-subarray-with-or-at-least-k-i/
// https://leetcode.cn/problems/shortest-subarray-with-or-at-least-k-i/
//
// Easy
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
//     1 <= nums.length <= 50
//     0 <= nums[i] <= 50
//     0 <= k < 64
//

struct Solution;

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = usize::MAX;
        for i in 0..nums.len() {
            let mut curr = 0;
            for (j, &nums_j) in nums.iter().enumerate().skip(i) {
                curr |= nums_j;
                if curr >= k {
                    res = res.min(j - i + 1);
                    break;
                }
            }
        }

        if res == usize::MAX {
            return -1;
        }
        res as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_subarray_length(vec![1, 2, 3], 2), 1);
    assert_eq!(Solution::minimum_subarray_length(vec![2, 1, 8], 10), 3);
    assert_eq!(Solution::minimum_subarray_length(vec![1, 2], 0), 1);
}
