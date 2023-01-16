#![allow(dead_code)]

// 907. Sum of Subarray Minimums
// https://leetcode.com/problems/sum-of-subarray-minimums/
// https://leetcode.cn/problems/sum-of-subarray-minimums/
//
// Given an array of integers arr, find the sum of min(b), where b ranges over every (contiguous) subarray of arr.
// Since the answer may be large, return the answer modulo 109 + 7.
//
// Example 1:
//
// Input: arr = [3,1,2,4]
// Output: 17
// Explanation:
// Subarrays are [3], [1], [2], [4], [3,1], [1,2], [2,4], [3,1,2], [1,2,4], [3,1,2,4].
// Minimums are 3, 1, 2, 4, 1, 1, 2, 1, 1, 1.
// Sum is 17.
//
// Example 2:
//
// Input: arr = [11,81,94,43,3]
// Output: 444
//
// Constraints:
//
// - 1 <= arr.length <= 3 * 10^4
// - 1 <= arr[i] <= 3 * 10^4
//

struct Solution;

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        use std::collections::VecDeque;
        const MOD: i32 = 1000000007;
        let mut dp = vec![0; arr.len()];
        let mut stack = VecDeque::new();

        for i in 0..arr.len() {
            while !stack.is_empty() && arr[*stack.back().unwrap() as usize] >= arr[i] {
                stack.pop_back();
            }

            if stack.is_empty() {
                dp[i] = (i + 1) as i32 * arr[i];
            } else {
                let j = *stack.back().unwrap();
                dp[i] = dp[j as usize] + (i as i32 - j) * arr[i];
            }

            stack.push_back(i as i32);
        }

        dp.iter().fold(0, |acc, v| (acc + v) % MOD) % MOD
    }
}

#[test]
fn test() {
    assert_eq!(Solution::sum_subarray_mins(vec![3, 1, 2, 4]), 17);
    assert_eq!(Solution::sum_subarray_mins(vec![11, 81, 94, 43, 3]), 444);
}
