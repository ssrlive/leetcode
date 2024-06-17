#![allow(dead_code)]

// 3112. Minimum Time to Visit Disappearing Nodes
// https://leetcode.com/problems/minimum-time-to-visit-disappearing-nodes/
// https://leetcode.cn/problems/minimum-time-to-visit-disappearing-nodes/
//
// Medium
//
// There is an undirected graph of n nodes. You are given a 2D array edges, where edges[i] = [ui, vi, lengthi]
// describes an edge between node ui and node vi with a traversal time of lengthi units.
//
// Additionally, you are given an array disappear, where disappear[i] denotes the time
// when the node i disappears from the graph and you won't be able to visit it.
//
// Notice that the graph might be disconnected and might contain multiple edges.
//
// Return the array answer, with answer[i] denoting the minimum units of time required to reach node i from node 0.
// If node i is unreachable from node 0 then answer[i] is -1.
//
// Example 1:
//
// Input: n = 3, edges = [[0,1,2],[1,2,1],[0,2,4]], disappear = [1,1,5]
//
// Output: [0,-1,4]
//
// Explanation:
//
// We are starting our journey from node 0, and our goal is to find the minimum time required to reach each node before it disappears.
//
//     For node 0, we don't need any time as it is our starting point.
//     For node 1, we need at least 2 units of time to traverse edges[0]. Unfortunately, it disappears at that moment, so we won't be able to visit it.
//     For node 2, we need at least 4 units of time to traverse edges[2].
//
// Example 2:
//
// Input: n = 3, edges = [[0,1,2],[1,2,1],[0,2,4]], disappear = [1,3,5]
//
// Output: [0,2,3]
//
// Explanation:
//
// We are starting our journey from node 0, and our goal is to find the minimum time required to reach each node before it disappears.
//
//     For node 0, we don't need any time as it is the starting point.
//     For node 1, we need at least 2 units of time to traverse edges[0].
//     For node 2, we need at least 3 units of time to traverse edges[0] and edges[1].
//
// Example 3:
//
// Input: n = 2, edges = [[0,1,1]], disappear = [1,1]
//
// Output: [0,-1]
//
// Explanation:
//
// Exactly when we reach node 1, it disappears.
//
// Constraints:
//
//     1 <= n <= 5 * 10^4
//     0 <= edges.length <= 10^5
//     edges[i] == [ui, vi, lengthi]
//     0 <= ui, vi <= n - 1
//     1 <= lengthi <= 10^5
//     disappear.length == n
//     1 <= disappear[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn minimum_time(n: i32, edges: Vec<Vec<i32>>, disappear: Vec<i32>) -> Vec<i32> {
        use std::cmp::Reverse;
        let mut q = std::collections::BinaryHeap::new();
        let mut adj = vec![vec![]; n as usize];
        for edges_i in &edges {
            let a = edges_i[0] as usize;
            let b = edges_i[1] as usize;
            let t = edges_i[2];
            if a == b {
                continue;
            }
            adj[a].push((b, t));
            adj[b].push((a, t));
        }
        q.push(Reverse((0, 0)));
        let mut dist = vec![i32::MAX; n as usize];
        let mut vis = vec![false; n as usize];
        dist[0] = 0;
        while let Some(Reverse((t, s))) = q.pop() {
            if vis[s] {
                continue;
            }
            if t >= disappear[s] {
                continue;
            }
            vis[s] = true;
            for &(i_first, ti) in &adj[s] {
                if t + ti < dist[i_first] && t + ti <= disappear[i_first] {
                    dist[i_first] = t + ti;
                    q.push(Reverse((t + ti, i_first)));
                }
            }
        }
        let mut ans = vec![];
        for i in 0..n as usize {
            if dist[i] < disappear[i] {
                ans.push(dist[i]);
            } else {
                ans.push(-1);
            }
        }
        ans
    }
}

#[test]
fn test() {
    let n = 3;
    let edges = vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]];
    let disappear = vec![1, 1, 5];
    let output = vec![0, -1, 4];
    assert_eq!(Solution::minimum_time(n, edges, disappear), output);

    let n = 3;
    let edges = vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]];
    let disappear = vec![1, 3, 5];
    let output = vec![0, 2, 3];
    assert_eq!(Solution::minimum_time(n, edges, disappear), output);

    let n = 2;
    let edges = vec![vec![0, 1, 1]];
    let disappear = vec![1, 1];
    let output = vec![0, -1];
    assert_eq!(Solution::minimum_time(n, edges, disappear), output);

    let n = 5;
    let edges = vec![
        vec![2, 0, 3],
        vec![1, 1, 1],
        vec![1, 4, 2],
        vec![1, 2, 4],
        vec![0, 2, 1],
        vec![2, 2, 5],
        vec![2, 4, 6],
        vec![4, 4, 1],
    ];
    let disappear = vec![7, 14, 12, 9, 19];
    let output = vec![0, 5, 1, -1, 7];
    assert_eq!(Solution::minimum_time(n, edges, disappear), output);
}
