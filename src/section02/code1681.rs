#![allow(dead_code)]

/*

// 1681. Minimum Incompatibility
// https://leetcode.com/problems/minimum-incompatibility/
// https://leetcode.cn/problems/minimum-incompatibility/
//
// Hard
//
// You are given an integer array nums​​​ and an integer k. You are asked to distribute this array into k subsets of equal size such that there are no two equal elements in the same subset.

A subset's incompatibility is the difference between the maximum and minimum elements in that array.

Return the minimum possible sum of incompatibilities of the k subsets after distributing the array optimally, or return -1 if it is not possible.

A subset is a group integers that appear in the array with no particular order.

Example 1:

Input: nums = [1,2,1,4], k = 2
Output: 4
Explanation: The optimal distribution of subsets is [1,2] and [1,4].
The incompatibility is (2-1) + (4-1) = 4.
Note that [1,1] and [2,4] would result in a smaller sum, but the first subset contains 2 equal elements.

Example 2:

Input: nums = [6,3,8,1,3,1,2,2], k = 4
Output: 6
Explanation: The optimal distribution of subsets is [1,2], [2,3], [6,8], and [1,3].
The incompatibility is (2-1) + (3-2) + (8-6) + (3-1) = 6.

Example 3:

Input: nums = [5,3,3,6,3,3], k = 3
Output: -1
Explanation: It is impossible to distribute nums into 3 subsets where no two elements are equal in the same subset.

Constraints:

    1 <= k <= nums.length <= 16
    nums.length is divisible by k
    1 <= nums[i] <= nums.length
*/

struct Solution;

impl Solution {
    pub fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::VecDeque;
        fn dfs(
            ind: usize,
            nums: &Vec<i32>,
            buckets: &mut Vec<VecDeque<i32>>,
            k: usize,
            bucket_len: usize,
            total: i32,
            res: &mut i32,
        ) {
            if ind == nums.len() {
                *res = total;
                return;
            }

            for i in 0..buckets.len().min(k + 1) {
                if buckets[i].len() < bucket_len && (buckets[i].is_empty() || *buckets[i].back().unwrap() < nums[ind]) {
                    let cur = nums[ind] - *buckets[i].back().unwrap_or(&nums[ind]) + total;
                    if cur < *res {
                        buckets[i].push_back(nums[ind]);
                        dfs(ind + 1, nums, buckets, k.max(i + 1), bucket_len, cur, res);
                        buckets[i].pop_back();
                    }
                }
            }
        }

        let mut nums = nums;
        nums.sort();
        let k = k as usize;
        let mut buckets = vec![VecDeque::with_capacity(nums.len() / k); k];
        let mut res = std::i32::MAX;
        dfs(0, &nums, &mut buckets, 0, nums.len() / k, 0, &mut res);
        if res == std::i32::MAX {
            return -1;
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 1, 4], 2, 4),
        (vec![6, 3, 8, 1, 3, 1, 2, 2], 4, 6),
        (vec![5, 3, 3, 6, 3, 3], 3, -1),
    ];
    for (nums, k, expected) in cases {
        assert_eq!(Solution::minimum_incompatibility(nums, k), expected);
    }
}
