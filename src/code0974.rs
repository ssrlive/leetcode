#![allow(dead_code)]

// 974. Subarray Sums Divisible by K
// https://leetcode.com/problems/subarray-sums-divisible-by-k/
// https://leetcode.cn/problems/subarray-sums-divisible-by-k/
//
// Given an integer array nums and an integer k, return the number of non-empty subarrays that have a sum divisible by k.
//
// A subarray is a contiguous part of an array.
//
// Example 1:
//
// Input: nums = [4,5,0,-2,-3,1], k = 5
// Output: 7
// Explanation: There are 7 subarrays with a sum divisible by k = 5:
// [4, 5, 0, -2, -3, 1], [5], [5, 0], [5, 0, -2, -3], [0], [0, -2, -3], [-2, -3]
//
// Example 2:
//
// Input: nums = [5], k = 9
// Output: 0
//
// Constraints:
//
// - 1 <= nums.length <= 3 * 10^4
// - -104 <= nums[i] <= 10^4
// - 2 <= k <= 10^4
//

struct Solution;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::*;
        let nums = nums.into_iter().map(|v| (v + k) % k).collect::<Vec<i32>>();
        let n = nums.len();
        let mut memo = vec![0; n + 1];
        for i in 0..n {
            memo[i + 1] = memo[i] + nums[i];
            memo[i + 1] += k;
            memo[i + 1] %= k;
        }

        let mut map = HashMap::new();
        for v in memo {
            *map.entry(v).or_insert(0) += 1;
        }

        let mut result = 0;
        for (_, v) in map {
            result += (v - 1) * v / 2;
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![(vec![4, 5, 0, -2, -3, 1], 5, 7), (vec![5], 9, 0), (vec![1, 2, 3], 3, 3)];
    for (nums, k, result) in cases {
        assert_eq!(Solution::subarrays_div_by_k(nums, k), result);
    }
}
