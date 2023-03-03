#![allow(dead_code)]

/*

// 2099. Find Subsequence of Length K With the Largest Sum
// https://leetcode.com/problems/find-subsequence-of-length-k-with-the-largest-sum/
// https://leetcode.cn/problems/find-subsequence-of-length-k-with-the-largest-sum/
//
// Easy
//
// You are given an integer array nums and an integer k. You want to find a subsequence of nums of length k that has the largest sum.

Return any such subsequence as an integer array of length k.

A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.

Example 1:

Input: nums = [2,1,3,3], k = 2
Output: [3,3]
Explanation:
The subsequence has the largest sum of 3 + 3 = 6.

Example 2:

Input: nums = [-1,-2,3,4], k = 3
Output: [-1,3,4]
Explanation:
The subsequence has the largest sum of -1 + 3 + 4 = 6.

Example 3:

Input: nums = [3,4,3,3], k = 2
Output: [3,4]
Explanation:
The subsequence has the largest sum of 3 + 4 = 7.
Another possible subsequence is [4, 3].

Constraints:

    1 <= nums.length <= 1000
    -10^5 <= nums[i] <= 10^5
    1 <= k <= nums.length
*/

struct Solution;

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == nums.len() as i32 || nums.len() == 1 {
            return nums;
        }

        let mut num_idx = nums.iter().enumerate().collect::<Vec<_>>();
        num_idx.sort_unstable_by_key(|(_, x)| **x);

        let mut idx = vec![false; nums.len()];
        num_idx
            .into_iter()
            .rev()
            .take(k as usize)
            .for_each(|(i, _)| idx[i] = true);

        nums.into_iter()
            .enumerate()
            .filter_map(|(i, x)| if idx[i] { Some(x) } else { None })
            .collect()
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![2, 1, 3, 3], 2, vec![3, 3]),
        (vec![-1, -2, 3, 4], 3, vec![-1, 3, 4]),
        (vec![3, 4, 3, 3], 2, vec![4, 3]),
        (vec![1, 2, 3, 4], 4, vec![1, 2, 3, 4]),
        (vec![1, 2, 3, 4], 3, vec![2, 3, 4]),
        (vec![1, 2, 3, 4], 2, vec![3, 4]),
        (vec![1, 2, 3, 4], 1, vec![4]),
    ];
    for (nums, k, expect) in cases {
        assert_eq!(Solution::max_subsequence(nums, k), expect);
    }
}
