#![allow(dead_code)]

// 3543. Maximum Weighted K-Edge Path
// https://leetcode.com/problems/maximum-weighted-k-edge-path
// https://leetcode.cn/problems/maximum-weighted-k-edge-path
//
// Medium
//
// You are given an integer n and a Directed Acyclic Graph (DAG) with n nodes labeled from 0 to n - 1.
// This is represented by a 2D array edges, where edges[i] = [ui, vi, wi] indicates a directed edge from node ui to vi with weight wi.
//
// You are also given two integers, k and t.
//
// Your task is to determine the maximum possible sum of edge weights for any path in the graph such that:
//
//     The path contains exactly k edges.
//     The total sum of edge weights in the path is strictly less than t.
//
// Return the maximum possible sum of weights for such a path. If no such path exists, return -1.
//
// Example 1:
//
// Input: n = 3, edges = [[0,1,1],[1,2,2]], k = 2, t = 4
//
// Output: 3
//
// Explanation:
//
//     The only path with k = 2 edges is 0 -> 1 -> 2 with weight 1 + 2 = 3 < t.
//     Thus, the maximum possible sum of weights less than t is 3.
//
// Example 2:
//
// Input: n = 3, edges = [[0,1,2],[0,2,3]], k = 1, t = 3
//
// Output: 2
//
// Explanation:
//
//     There are two paths with k = 1 edge:
//         0 -> 1 with weight 2 < t.
//         0 -> 2 with weight 3 = t, which is not strictly less than t.
//     Thus, the maximum possible sum of weights less than t is 2.
//
// Example 3:
//
// Input: n = 3, edges = [[0,1,6],[1,2,8]], k = 1, t = 6
//
// Output: -1
//
// Explanation:
//
//     There are two paths with k = 1 edge:
//         0 -> 1 with weight 6 = t, which is not strictly less than t.
//         1 -> 2 with weight 8 > t, which is not strictly less than t.
//     Since there is no path with sum of weights strictly less than t, the answer is -1.
//
// Constraints:
//
//     1 <= n <= 300
//     0 <= edges.length <= 300
//     edges[i] = [ui, vi, wi]
//     0 <= ui, vi < n
//     ui != vi
//     1 <= wi <= 10
//     0 <= k <= 300
//     1 <= t <= 600
//     The input graph is guaranteed to be a DAG.
//     There are no duplicate edges.
//

struct Solution;

impl Solution {
    pub fn max_weight(n: i32, edges: Vec<Vec<i32>>, k: i32, t: i32) -> i32 {
        // Build adjacency list for DAG
        let mut adj = vec![vec![]; n as usize];
        for e in &edges {
            adj[e[0] as usize].push((e[1], e[2]));
        }

        let mut dp = vec![vec![std::collections::HashSet::new(); (k + 1) as usize]; n as usize];

        // Base Case
        for u in 0..n {
            dp[u as usize][0].insert(0);
        }

        for e in 0..k {
            for u in 0..n {
                let (left, right) = dp.split_at_mut(u as usize + 1);
                for &(v, wt) in &adj[u as usize] {
                    for &w in &left[u as usize][e as usize] {
                        let new_w = w + wt;
                        if new_w < t {
                            right[v as usize - (u as usize + 1)][(e + 1) as usize].insert(new_w); // Correct indexing
                        }
                    }
                }
            }
        }

        // Find the maximum weight across all nodes that used exactly k edges
        let mut ans = -1;
        for u in 0..n {
            if !dp[u as usize][k as usize].is_empty() {
                ans = ans.max(*dp[u as usize][k as usize].iter().max().unwrap());
            }
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_weight(3, vec![vec![0, 1, 1], vec![1, 2, 2]], 2, 4), 3);
    assert_eq!(Solution::max_weight(3, vec![vec![0, 1, 2], vec![0, 2, 3]], 1, 3), 2);
    assert_eq!(Solution::max_weight(3, vec![vec![0, 1, 6], vec![1, 2, 8]], 1, 6), -1);
}
