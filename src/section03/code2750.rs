#![allow(dead_code)]

// 2750. Ways to Split Array Into Good Subarrays
// https://leetcode.com/problems/ways-to-split-array-into-good-subarrays/
// https://leetcode.cn/problems/ways-to-split-array-into-good-subarrays/
//
// Medium
//
// You are given a binary array nums.
//
// A subarray of an array is good if it contains exactly one element with the value 1.
//
// Return an integer denoting the number of ways to split the array nums into good subarrays.
// As the number may be too large, return it modulo 109 + 7.
//
// A subarray is a contiguous non-empty sequence of elements within an array.
//
// Example 1:
//
// Input: nums = [0,1,0,0,1]
// Output: 3
// Explanation: There are 3 ways to split nums into good subarrays:
// - [0,1] [0,0,1]
// - [0,1,0] [0,1]
// - [0,1,0,0] [1]
//
// Example 2:
//
// Input: nums = [0,1,0]
// Output: 1
// Explanation: There is 1 way to split nums into good subarrays:
// - [0,1,0]
//
// Constraints:
//
//     1 <= nums.length <= 10^5
//     0 <= nums[i] <= 1
//

struct Solution;

impl Solution {
    pub fn number_of_good_subarray_splits(nums: Vec<i32>) -> i32 {
        let indices = nums
            .iter()
            .enumerate()
            .filter(|&(_i, &n)| (n == 1))
            .map(|(i, &_n)| i)
            .collect::<Vec<_>>();
        if indices.is_empty() {
            return 0;
        }

        let mut ans = 1usize;
        for w in indices.windows(2) {
            ans = (ans * (w[1] - w[0])) % 1000000007;
        }
        ans as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_good_subarray_splits(vec![0, 1, 0, 0, 1]), 3);
    assert_eq!(Solution::number_of_good_subarray_splits(vec![0, 1, 0]), 1);
}
