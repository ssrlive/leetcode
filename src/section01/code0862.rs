#![allow(dead_code)]

// 862. Shortest Subarray with Sum at Least K
// https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k/
// https://leetcode.cn/problems/shortest-subarray-with-sum-at-least-k/
//
// Given an integer array nums and an integer k, return the length of the shortest non-empty subarray
// of nums with a sum of at least k. If there is no such subarray, return -1.
//
// A subarray is a contiguous part of an array.
//
// Example 1:
//
// Input: nums = [1], k = 1
// Output: 1
//
// Example 2:
//
// Input: nums = [1,2], k = 4
// Output: -1
//
// Example 3:
//
// Input: nums = [2,-1,2], k = 3
// Output: 3
//
// Constraints:
//
// - 1 <= nums.length <= 10^5
// - -10^5 <= nums[i] <= 10^5
// - 1 <= k <= 10^9
//

struct Solution;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        fn _shortest_subarray(nums: Vec<i64>, k: i64) -> i64 {
            use std::collections::VecDeque;

            let mut ans = i64::MAX;
            let mut queue: VecDeque<(i64, i64)> = VecDeque::new();
            queue.push_back((-1, 0));
            let mut prefix = 0;
            for (i, x) in nums.iter().enumerate() {
                prefix += x;
                while !queue.is_empty() && prefix - queue[0].1 >= k {
                    ans = ans.min(i as i64 - queue[0].0);
                    queue.pop_front();
                }

                while !queue.is_empty() && queue[queue.len() - 1].1 >= prefix {
                    queue.pop_back();
                }

                queue.push_back((i as i64, prefix));
            }
            if ans == i64::MAX { -1 } else { ans }
        }
        _shortest_subarray(nums.iter().map(|x| *x as i64).collect(), k as i64) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::shortest_subarray(vec![1], 1), 1);
    assert_eq!(Solution::shortest_subarray(vec![1, 2], 4), -1);
    assert_eq!(Solution::shortest_subarray(vec![2, -1, 2], 3), 3);
    assert_eq!(Solution::shortest_subarray(vec![2, -1, 2], 4), -1);
}
