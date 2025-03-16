#![allow(dead_code)]

// 3488. Closest Equal Element Queries
// https://leetcode.com/problems/closest-equal-element-queries/
// https://leetcode.cn/problems/closest-equal-element-queries/
//
// Medium
//
// You are given a circular array nums and an array queries.
//
// For each query i, you have to find the following:
//
// - The minimum distance between the element at index queries[i] and any other index j in the circular array,
//   where nums[j] == nums[queries[i]]. If no such index exists, the answer for that query should be -1.
//
// Return an array answer of the same size as queries, where answer[i] represents the result for query i.
//
// Example 1:
//
// Input: nums = [1,3,1,4,1,3,2], queries = [0,3,5]
//
// Output: [2,-1,3]
//
// Explanation:
//
// - Query 0: The element at queries[0] = 0 is nums[0] = 1. The nearest index with the same value is 2, and the distance between them is 2.
// - Query 1: The element at queries[1] = 3 is nums[3] = 4. No other index contains 4, so the result is -1.
// - Query 2: The element at queries[2] = 5 is nums[5] = 3. The nearest index with the same value is 1,
//            and the distance between them is 3 (following the circular path: 5 -> 6 -> 0 -> 1).
//
// Example 2:
//
// Input: nums = [1,2,3,4], queries = [0,1,2,3]
//
// Output: [-1,-1,-1,-1]
//
// Explanation:
//
// Each value in nums is unique, so no index shares the same value as the queried element. This results in -1 for all queries.
//
// Constraints:
//
//     1 <= queries.length <= nums.length <= 10^5
//     1 <= nums[i] <= 10^6
//     0 <= queries[i] < nums.length
//

struct Solution;

impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut left_seen = HashMap::new();
        let mut right_seen = HashMap::new();
        let mut dist = vec![i32::MAX; nums.len()];
        let mut ans = Vec::new();
        for i in 0..nums.len() * 2 {
            if let Some(&left) = left_seen.get(&nums[i % nums.len()]) {
                dist[i % nums.len()] = i as i32 - left;
            }
            left_seen.insert(nums[i % nums.len()], i as i32);
        }
        for i in (0..nums.len() * 2).rev() {
            if let Some(&right) = right_seen.get(&nums[i % nums.len()]) {
                dist[i % nums.len()] = dist[i % nums.len()].min(right - i as i32);
            }
            right_seen.insert(nums[i % nums.len()], i as i32);
        }
        for q in queries {
            if dist[q as usize] >= nums.len() as i32 {
                ans.push(-1);
            } else {
                ans.push(dist[q as usize]);
            }
        }
        ans
    }
}

#[test]
fn test() {
    let nums = vec![1, 3, 1, 4, 1, 3, 2];
    let queries = vec![0, 3, 5];
    let output = vec![2, -1, 3];
    assert_eq!(Solution::solve_queries(nums, queries), output);

    let nums = vec![1, 2, 3, 4];
    let queries = vec![0, 1, 2, 3];
    let output = vec![-1, -1, -1, -1];
    assert_eq!(Solution::solve_queries(nums, queries), output);
}
