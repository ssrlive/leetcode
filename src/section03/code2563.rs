#![allow(dead_code)]

/*

// 2563. Count the Number of Fair Pairs
// https://leetcode.com/problems/count-the-number-of-fair-pairs/
// https://leetcode.cn/problems/count-the-number-of-fair-pairs/
//
// Medium
//
// Given a 0-indexed integer array nums of size n and two integers lower and upper, return the number of fair pairs.

A pair (i, j) is fair if:

    0 <= i < j < n, and
    lower <= nums[i] + nums[j] <= upper

Example 1:

Input: nums = [0,1,7,4,4,5], lower = 3, upper = 6
Output: 6
Explanation: There are 6 fair pairs: (0,3), (0,4), (0,5), (1,3), (1,4), and (1,5).

Example 2:

Input: nums = [1,7,9,2,5], lower = 11, upper = 11
Output: 1
Explanation: There is a single fair pair: (2,3).

Constraints:

    1 <= nums.length <= 10^5
    nums.length == n
    -10^9 <= nums[i] <= 10^9
    -10^9 <= lower <= upper <= 10^9
*/

struct Solution;

impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut ans = 0i64;
        for (idx, num) in nums.iter().copied().enumerate() {
            let l_bound = nums[idx + 1..].partition_point(|&n| n + num < lower);
            let u_bound = nums[idx + 1..].partition_point(|&n| n + num <= upper);
            ans += (u_bound - l_bound) as i64;
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![(vec![0, 1, 7, 4, 4, 5], 3, 6, 6), (vec![1, 7, 9, 2, 5], 11, 11, 1)];
    for (nums, lower, upper, expected) in cases {
        assert_eq!(Solution::count_fair_pairs(nums, lower, upper), expected);
    }
}
