#![allow(dead_code)]

/*

// 1546. Maximum Number of Non-Overlapping Subarrays With Sum Equals Target
// https://leetcode.com/problems/maximum-number-of-non-overlapping-subarrays-with-sum-equals-target/
// https://leetcode.cn/problems/maximum-number-of-non-overlapping-subarrays-with-sum-equals-target/
//
// Medium
//
// Given an array nums and an integer target, return the maximum number of non-empty non-overlapping subarrays such that the sum of values in each subarray is equal to target.

Example 1:

Input: nums = [1,1,1,1,1], target = 2
Output: 2
Explanation: There are 2 non-overlapping subarrays [1,1,1,1,1] with sum equals to target(2).

Example 2:

Input: nums = [-1,3,5,1,4,2,-9], target = 6
Output: 2
Explanation: There are 3 subarrays with sum equal to 6.
([5,1], [4,2], [3,5,1,4,2,-9]) but only the first 2 are non-overlapping.

Constraints:

    1 <= nums.length <= 10^5
    -10^4 <= nums[i] <= 10^4
    0 <= target <= 10^6
*/

struct Solution;

impl Solution {
    pub fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32 {
        use std::collections::*;
        let n = nums.len();
        let mut memo = vec![0; n + 1];
        for i in 0..n {
            memo[i + 1] = memo[i] + nums[i];
        }

        let mut count = 0;
        let mut set = HashSet::new();
        for i in (0..=n).rev() {
            let v = memo[i];
            let need = v + target;
            if set.contains(&need) {
                count += 1;
                set = HashSet::new();
            }
            set.insert(v);
        }
        count
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 1, 1, 1, 1], 2, 2),
        (vec![-1, 3, 5, 1, 4, 2, -9], 6, 2),
        (vec![0, 0, 0], 0, 3),
    ];
    for (nums, target, expected) in cases {
        assert_eq!(Solution::max_non_overlapping(nums, target), expected);
    }
}
