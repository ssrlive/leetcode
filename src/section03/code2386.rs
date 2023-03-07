#![allow(dead_code)]

/*

// 2386. Find the K-Sum of an Array
// https://leetcode.com/problems/find-the-k-sum-of-an-array/
// https://leetcode.cn/problems/find-the-k-sum-of-an-array/
//
// Hard
//
// You are given an integer array nums and a positive integer k. You can choose any subsequence of the array and sum all of its elements together.

We define the K-Sum of the array as the kth largest subsequence sum that can be obtained (not necessarily distinct).

Return the K-Sum of the array.

A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.

Note that the empty subsequence is considered to have a sum of 0.

Example 1:

Input: nums = [2,4,-2], k = 5
Output: 2
Explanation: All the possible subsequence sums that we can obtain are the following sorted in decreasing order:
- 6, 4, 4, 2, 2, 0, 0, -2.
The 5-Sum of the array is 2.

Example 2:

Input: nums = [1,-2,3,4,-10,12], k = 16
Output: 10
Explanation: The 16-Sum of the array is 10.

Constraints:

    n == nums.length
    1 <= n <= 10^5
    -10^9 <= nums[i] <= 10^9
    1 <= k <= min(2000, 2n)
*/

struct Solution;

impl Solution {
    pub fn k_sum(nums: Vec<i32>, k: i32) -> i64 {
        use std::collections::BinaryHeap;
        let mut sum = 0i64;
        let n = nums.len();
        let mut data = vec![];
        for a in nums {
            if a > 0 {
                sum += a as i64;
            }
            data.push(i64::abs(a as i64));
        }
        data.sort();
        let mut pq = BinaryHeap::<(i64, usize)>::new();
        let mut ret = sum;
        pq.push((-data[0], 0));
        for _ in 2..=k {
            let (val, i) = pq.pop().unwrap();
            ret = sum + val;
            if i == n - 1 {
                continue;
            }
            pq.push((val - data[i + 1], i + 1));
            pq.push((val + data[i] - data[i + 1], i + 1));
        }
        ret
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![2, 4, -2], 5, 2),
        (vec![1, -2, 3, 4, -10, 12], 16, 10),
        (vec![1, -2, 3, 4, -10, 12], 10, 14),
    ];
    for (nums, k, expected) in cases {
        assert_eq!(Solution::k_sum(nums, k), expected);
    }
}
