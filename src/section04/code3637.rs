#![allow(dead_code)]

// 3637. Trionic Array I
// https://leetcode.com/problems/trionic-array-i/
// https://leetcode.cn/problems/trionic-array-i/
//
// Easy
//
// You are given an integer array nums of length n.
//
// An array is trionic if there exist indices 0 < p < q < n − 1 such that:
//
//     nums[0...p] is strictly increasing,
//     nums[p...q] is strictly decreasing,
//     nums[q...n − 1] is strictly increasing.
//
// Return true if nums is trionic, otherwise return false.
//
// Example 1:
//
// Input: nums = [1,3,5,4,2,6]
//
// Output: true
//
// Explanation:
//
// Pick p = 2, q = 4:
//
//     nums[0...2] = [1, 3, 5] is strictly increasing (1 < 3 < 5).
//     nums[2...4] = [5, 4, 2] is strictly decreasing (5 > 4 > 2).
//     nums[4...5] = [2, 6] is strictly increasing (2 < 6).
//
// Example 2:
//
// Input: nums = [2,1,3]
//
// Output: false
//
// Explanation:
//
// There is no way to pick p and q to form the required three segments.
//
// Constraints:
//
//     3 <= n <= 100
//     -1000 <= nums[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let len = nums.len();
        if len < 4 {
            return false;
        }

        let mut prev = nums[0];
        let mut i = 1;

        let mut inc0 = false;
        while i < len && prev < nums[i] {
            prev = nums[i];
            i += 1;
            inc0 = true;
        }

        let mut dec0 = false;
        while i < len && prev > nums[i] {
            prev = nums[i];
            i += 1;
            dec0 = true;
        }

        let mut inc1 = false;
        while i < len && prev < nums[i] {
            prev = nums[i];
            inc1 = true;
            i += 1;
        }

        i == len && inc0 && dec0 && inc1
    }
}

#[test]
fn test() {
    assert!(Solution::is_trionic(vec![1, 3, 5, 4, 2, 6]));
    assert!(!Solution::is_trionic(vec![2, 1, 3]));
}
