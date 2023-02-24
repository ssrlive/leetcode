#![allow(dead_code)]

/*

// 1856. Maximum Subarray Min-Product
// https://leetcode.com/problems/maximum-subarray-min-product/
// https://leetcode.cn/problems/maximum-subarray-min-product/
//
// Medium
//
// The min-product of an array is equal to the minimum value in the array multiplied by the array's sum.

    For example, the array [3,2,5] (minimum value is 2) has a min-product of 2 * (3+2+5) = 2 * 10 = 20.

Given an array of integers nums, return the maximum min-product of any non-empty subarray of nums. Since the answer may be large, return it modulo 109 + 7.

Note that the min-product should be maximized before performing the modulo operation. Testcases are generated such that the maximum min-product without modulo will fit in a 64-bit signed integer.

A subarray is a contiguous part of an array.

Example 1:

Input: nums = [1,2,3,2]
Output: 14
Explanation: The maximum min-product is achieved with the subarray [2,3,2] (minimum value is 2).
2 * (2+3+2) = 2 * 7 = 14.

Example 2:

Input: nums = [2,3,3,1,2]
Output: 18
Explanation: The maximum min-product is achieved with the subarray [3,3] (minimum value is 3).
3 * (3+3) = 3 * 6 = 18.

Example 3:

Input: nums = [3,1,5,6,4,2]
Output: 60
Explanation: The maximum min-product is achieved with the subarray [5,6,4] (minimum value is 4).
4 * (5+6+4) = 4 * 15 = 60.

Constraints:

    1 <= nums.length <= 10^5
    1 <= nums[i] <= 10^7
*/

struct Solution;

impl Solution {
    pub fn max_sum_min_product(nums: Vec<i32>) -> i32 {
        use std::collections::VecDeque;

        fn get_sum(lo: usize, hi: usize, prefix_sums: &[u64]) -> u64 {
            prefix_sums[hi + 1] - prefix_sums[lo]
        }

        const MOD: u64 = 1e9 as u64 + 7;
        let len_n = nums.len();
        let mut left_bound: Vec<usize> = vec![0; len_n];
        let mut right_bound: Vec<usize> = vec![0; len_n];
        let mut stack: VecDeque<usize> = VecDeque::new();
        for i in 0..len_n {
            while let Some(&top) = stack.back() {
                if nums[top] >= nums[i] {
                    stack.pop_back();
                } else {
                    break;
                }
            }
            if let Some(&top) = stack.back() {
                left_bound[i] = top + 1;
            } else {
                left_bound[i] = 0;
            }
            stack.push_back(i);
        }
        stack.clear();
        for i in (0..len_n).rev() {
            while let Some(&top) = stack.back() {
                if nums[top] >= nums[i] {
                    stack.pop_back();
                } else {
                    break;
                }
            }
            if let Some(&top) = stack.back() {
                right_bound[i] = top - 1;
            } else {
                right_bound[i] = len_n - 1;
            }
            stack.push_back(i);
        }
        let mut prefix_sums: Vec<u64> = vec![0; len_n + 1];
        for i in 0..len_n {
            prefix_sums[i + 1] = prefix_sums[i] + nums[i] as u64;
        }
        let mut max_product: u64 = 0;
        for i in 0..len_n {
            max_product = std::cmp::max(
                max_product,
                get_sum(left_bound[i], right_bound[i], &prefix_sums) * (nums[i] as u64),
            );
        }
        (max_product % MOD) as i32
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (vec![1, 2, 3, 2], 14),
        (vec![2, 3, 3, 1, 2], 18),
        (vec![3, 1, 5, 6, 4, 2], 60),
    ];
    for (nums, expected) in cases {
        assert_eq!(Solution::max_sum_min_product(nums), expected);
    }
}
