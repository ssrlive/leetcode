#![allow(dead_code)]

/*

// 1755. Closest Subsequence Sum
// https://leetcode.com/problems/closest-subsequence-sum/
// https://leetcode.cn/problems/closest-subsequence-sum/
//
// Hard
//
// You are given an integer array nums and an integer goal.

You want to choose a subsequence of nums such that the sum of its elements is the closest possible to goal. That is, if the sum of the subsequence's elements is sum, then you want to minimize the absolute difference abs(sum - goal).

Return the minimum possible value of abs(sum - goal).

Note that a subsequence of an array is an array formed by removing some elements (possibly all or none) of the original array.

Example 1:

Input: nums = [5,-7,3,5], goal = 6
Output: 0
Explanation: Choose the whole array as a subsequence, with a sum of 6.
This is equal to the goal, so the absolute difference is 0.

Example 2:

Input: nums = [7,-9,15,-2], goal = -5
Output: 1
Explanation: Choose the subsequence [7,-9,-2], with a sum of -4.
The absolute difference is abs(-4 - (-5)) = abs(1) = 1, which is the minimum.

Example 3:

Input: nums = [1,2,3], goal = -7
Output: 7

Constraints:

    1 <= nums.length <= 40
    -10^7 <= nums[i] <= 10^7
    -10^9 <= goal <= 10^9
*/

struct Solution;

impl Solution {
    pub fn min_abs_difference(nums: Vec<i32>, goal: i32) -> i32 {
        let n = nums.len();
        let m1 = n / 2;
        let m2 = n - m1;
        let (sz1, sz2) = (1 << m1, 1 << m2);

        let (mut v1, mut v2) = (vec![0; sz1], vec![0; sz2]);

        for (i, v1_i) in v1.iter_mut().enumerate() {
            for (k, &num) in nums.iter().enumerate().take(m1) {
                if i & (1 << k) > 0 {
                    *v1_i += num as i64;
                }
            }
        }

        for (i, v2_i) in v2.iter_mut().enumerate() {
            for k in 0..m2 {
                if i & (1 << k) > 0 {
                    *v2_i += nums[m1 + k] as i64;
                }
            }
        }

        v2.sort();

        let mut ret = i64::MAX;
        for a in v1 {
            let i = Self::binary_search(&v2, goal as i64 - a);
            ret = ret.min(i64::abs(goal as i64 - a - v2[i]));
            if i + 1 < v2.len() {
                ret = ret.min(i64::abs(goal as i64 - a - v2[i + 1]));
            }
        }

        ret as i32
    }

    fn binary_search(v2: &Vec<i64>, val: i64) -> usize {
        if val >= v2[v2.len() - 1] {
            return v2.len() - 1;
        }
        let (mut left, mut right) = (0, v2.len() - 1);
        while left < right {
            let mid = right - (right - left) / 2;
            if v2[mid] <= val {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        left
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![5, -7, 3, 5], 6, 0),
        (vec![7, -9, 15, -2], -5, 1),
        (vec![1, 2, 3], -7, 7),
    ];
    for (nums, goal, expected) in cases {
        assert_eq!(Solution::min_abs_difference(nums, goal), expected);
    }
}
