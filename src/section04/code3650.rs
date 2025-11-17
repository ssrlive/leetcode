#![allow(dead_code)]

// 3650. Minimum Cost Path with Edge Reversals
// https://leetcode.com/problems/minimum-cost-path-with-edge-reversals/
// https://leetcode.cn/problems/minimum-cost-path-with-edge-reversals/
//
// Medium
//
// You are given a directed, weighted graph with n nodes labeled from 0 to n - 1, and an array edges where edges[i] = [ui, vi, wi] represents a directed edge from node ui to node vi with cost wi.
//
// Each node ui has a switch that can be used at most once: when you arrive at ui and have not yet used its switch, you may activate it on one of its incoming edges vi → ui reverse that edge to ui → vi and immediately traverse it.
//
// The reversal is only valid for that single move, and using a reversed edge costs 2 * wi.
//
// Return the minimum total cost to travel from node 0 to node n - 1. If it is not possible, return -1.
//
// Example 1:
//
// Input: n = 4, edges = [[0,1,3],[3,1,1],[2,3,4],[0,2,2]]
//
// Output: 5
//
// Explanation:
//
// Use the path 0 → 1 (cost 3).
// At node 1 reverse the original edge 3 → 1 into 1 → 3 and traverse it at cost 2 * 1 = 2.
// Total cost is 3 + 2 = 5.
//
// Example 2:
//
// Input: n = 4, edges = [[0,2,1],[2,1,1],[1,3,1],[2,3,3]]
//
// Output: 3
//
// Explanation:
//
// No reversal is needed. Take the path 0 → 2 (cost 1), then 2 → 1 (cost 1), then 1 → 3 (cost 1).
// Total cost is 1 + 1 + 1 = 3.
//
// Constraints:
//
// 2 <= n <= 5 * 10^4
// 1 <= edges.length <= 10^5
// edges[i] = [ui, vi, wi]
// 0 <= ui, vi <= n - 1
// 1 <= wi <= 1000

struct Solution;

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for e in edges.iter() {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let w = e[2];
            adj[u].push((v, w as i64));
            adj[v].push((u, (w as i64) << 1)); // Reverse edge with double cost
        }

        let inf = 1_000_000_000_i64;
        let mut d = vec![inf; n];
        let mut pq = BinaryHeap::new();
        pq.push(Reverse((0i64, 0usize)));
        d[0] = 0;

        while let Some(Reverse((dis, u))) = pq.pop() {
            if dis != d[u] {
                continue;
            }
            if u == n - 1 {
                return dis as i32;
            }
            for &(v, w) in adj[u].iter() {
                if dis + w < d[v] {
                    d[v] = dis + w;
                    pq.push(Reverse((d[v], v)));
                }
            }
        }

        -1
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::min_cost(4, vec![vec![0, 1, 3], vec![3, 1, 1], vec![2, 3, 4], vec![0, 2, 2]]),
        5
    );
    assert_eq!(
        Solution::min_cost(4, vec![vec![0, 2, 1], vec![2, 1, 1], vec![1, 3, 1], vec![2, 3, 3]]),
        3
    );
}
