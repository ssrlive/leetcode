#![allow(dead_code)]

/*

// 2006. Count Number of Pairs With Absolute Difference K
// https://leetcode.com/problems/count-number-of-pairs-with-absolute-difference-k/
// https://leetcode.cn/problems/count-number-of-pairs-with-absolute-difference-k/
//
// Easy
//
// Given an integer array nums and an integer k, return the number of pairs (i, j) where i < j such that |nums[i] - nums[j]| == k.

The value of |x| is defined as:

    x if x >= 0.
    -x if x < 0.

Example 1:

Input: nums = [1,2,2,1], k = 1
Output: 4
Explanation: The pairs with an absolute difference of 1 are:
- [1,2,2,1]
- [1,2,2,1]
- [1,2,2,1]
- [1,2,2,1]

Example 2:

Input: nums = [1,3], k = 3
Output: 0
Explanation: There are no pairs with an absolute difference of 3.

Example 3:

Input: nums = [3,2,1,5,4], k = 2
Output: 3
Explanation: The pairs with an absolute difference of 2 are:
- [3,2,1,5,4]
- [3,2,1,5,4]
- [3,2,1,5,4]

Constraints:

    1 <= nums.length <= 200
    1 <= nums[i] <= 100
    1 <= k <= 99
*/

struct Solution;

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter()
            .enumerate()
            .map(|(ind, &x)| nums[ind + 1..].iter().filter(|&&y| (x - y).abs() == k).count() as i32)
            .sum::<_>()
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 2, 1], 1, 4),
        (vec![1, 3], 3, 0),
        (vec![3, 2, 1, 5, 4], 2, 3),
    ];
    for (nums, k, expected) in cases {
        assert_eq!(Solution::count_k_difference(nums, k), expected);
    }
}
