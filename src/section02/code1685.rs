#![allow(dead_code)]

/*

// 1685. Sum of Absolute Differences in a Sorted Array
// https://leetcode.com/problems/sum-of-absolute-differences-in-a-sorted-array/
// https://leetcode.cn/problems/sum-of-absolute-differences-in-a-sorted-array/
//
// Medium
//
// You are given an integer array nums sorted in non-decreasing order.

Build and return an integer array result with the same length as nums such that result[i] is equal to the summation of absolute differences between nums[i] and all the other elements in the array.

In other words, result[i] is equal to sum(|nums[i]-nums[j]|) where 0 <= j < nums.length and j != i (0-indexed).

Example 1:

Input: nums = [2,3,5]
Output: [4,3,5]
Explanation: Assuming the arrays are 0-indexed, then
result[0] = |2-2| + |2-3| + |2-5| = 0 + 1 + 3 = 4,
result[1] = |3-2| + |3-3| + |3-5| = 1 + 0 + 2 = 3,
result[2] = |5-2| + |5-3| + |5-5| = 3 + 2 + 0 = 5.

Example 2:

Input: nums = [1,4,6,8,10]
Output: [24,15,13,15,21]

Constraints:

    2 <= nums.length <= 10^5
    1 <= nums[i] <= nums[i + 1] <= 10^4
*/

struct Solution;

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut prefix = vec![0; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i];
        }
        let mut res = vec![0; n];
        for i in 0..n {
            let left = nums[i] * i as i32 - prefix[i];
            let right = prefix[n] - prefix[i + 1] - nums[i] * (n - i - 1) as i32;
            res[i] = left + right;
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![2, 3, 5], vec![4, 3, 5]),
        (vec![1, 4, 6, 8, 10], vec![24, 15, 13, 15, 21]),
    ];
    for (nums, ex) in cases {
        assert_eq!(Solution::get_sum_absolute_differences(nums), ex);
    }
}
