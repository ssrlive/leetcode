#![allow(dead_code)]

// 2699. Modify Graph Edge Weights
// https://leetcode.com/problems/modify-graph-edge-weights/
// https://leetcode.cn/problems/modify-graph-edge-weights/
//
// Hard
//
// You are given an undirected weighted connected graph containing n nodes labeled from 0 to n - 1, and an integer
// array edges where edges[i] = [ai, bi, wi] indicates that there is an edge between nodes ai and bi with weight wi.
//
// Some edges have a weight of -1 (wi = -1), while others have a positive weight (wi > 0).
//
// Your task is to modify all edges with a weight of -1 by assigning them positive integer values in the range [1, 2 * 109]
// so that the shortest distance between the nodes source and destination becomes equal to an integer target.
// If there are multiple modifications that make the shortest distance between source and destination equal to target,
// any of them will be considered correct.
//
// Return an array containing all edges (even unmodified ones) in any order if it is possible to make the shortest distance
// from source to destination equal to target, or an empty array if it's impossible.
//
// Note: You are not allowed to modify the weights of edges with initial positive weights.
//
// Example 1:
//                  3
//                 / \
//                /   \
//               0     4
//                \   /
//                 2 1
//
// Input: n = 5, edges = [[4,1,-1],[2,0,-1],[0,3,-1],[4,3,-1]], source = 0, destination = 1, target = 5
// Output: [[4,1,1],[2,0,1],[0,3,3],[4,3,1]]
// Explanation: The graph above shows a possible modification to the edges, making the distance from 0 to 1 equal to 5.
//
// Example 2:
//              0
//             / \
//            2   1
//
// Input: n = 3, edges = [[0,1,-1],[0,2,5]], source = 0, destination = 2, target = 6
// Output: []
// Explanation: The graph above contains the initial edges. It is not possible to make the distance from 0 to 2
// equal to 6 by modifying the edge with weight -1. So, an empty array is returned.
//
// Example 3:
//          2 ---- 3
//          |      |
//          1 ---- 0
//
// Input: n = 4, edges = [[1,0,4],[1,2,3],[2,3,5],[0,3,-1]], source = 0, destination = 2, target = 6
// Output: [[1,0,4],[1,2,3],[2,3,5],[0,3,1]]
// Explanation: The graph above shows a modified graph having the shortest distance from 0 to 2 as 6.
//
// Constraints:
//
//     1 <= n <= 100
//     1 <= edges.length <= n * (n - 1) / 2
//     edges[i].length == 3
//     0 <= ai, bi < n
//     wi = -1 or 1 <= wi <= 10^7
//     ai != bi
//     0 <= source, destination < n
//     source != destination
//     1 <= target <= 10^9
//     The graph is connected, and there are no self-loops or repeated edges
//

struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn modified_graph_edges(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32, target: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut graph = vec![HashMap::<usize, i32>::new(); n];
        let mut s = vec![];

        for e in &edges {
            let (u, v, w) = (e[0] as usize, e[1] as usize, e[2]);
            if w == -1 {
                s.push((u, v));
            }
            if w != -1 {
                graph[u].insert(v, w);
            }
            if w != -1 {
                graph[v].insert(u, w);
            }
        }

        let (mut dist, mut dist2) = (vec![0; n], vec![0; n]);
        Self::dfs(&graph, &mut dist, source as usize);
        while dist[destination as usize] > target && !s.is_empty() {
            Self::dfs(&graph, &mut dist2, destination as usize);
            let (u, v) = s.pop().unwrap();
            let t = (dist[u] + dist2[v]).min(dist[v] + dist2[u]);
            let adjust = if t >= target { 1 } else { target - t };
            graph[u].insert(v, adjust);
            graph[v].insert(u, adjust);
            Self::dfs(&graph, &mut dist, source as usize);
        }
        if dist[destination as usize] != target {
            return vec![];
        }

        let mut ret = vec![];
        for e in edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            if let Some(w) = graph[u].get(&v) {
                ret.push(vec![u as i32, v as i32, *w]);
            } else {
                ret.push(vec![u as i32, v as i32, 1_000_000_000]);
            }
        }
        ret
    }

    fn dfs(graph: &Vec<HashMap<usize, i32>>, dist: &mut [i32], source: usize) {
        for item in dist.iter_mut().take(graph.len()) {
            *item = 1_000_000_000;
        }
        dist[source] = 0;
        let mut pq = BinaryHeap::new();
        pq.push(Reverse((0, source)));

        while let Some(Reverse((d, u))) = pq.pop() {
            if d > dist[u] {
                continue;
            }

            for p in &graph[u] {
                let (v, w) = (*p.0, *p.1);
                if dist[v] - dist[u] <= w {
                    continue;
                }
                dist[v] = w + dist[u];
                pq.push(Reverse((dist[v], v)));
            }
        }
    }
}

#[test]
fn test() {
    let n = 5;
    let edges = vec![vec![4, 1, -1], vec![2, 0, -1], vec![0, 3, -1], vec![4, 3, -1]];
    let source = 0;
    let destination = 1;
    let target = 5;
    let res = [
        vec![vec![4, 1, 1], vec![2, 0, 1], vec![0, 3, 3], vec![4, 3, 1]],
        vec![vec![4, 1, 3], vec![2, 0, 1], vec![0, 3, 1], vec![4, 3, 1]],
    ];
    let r = Solution::modified_graph_edges(n, edges, source, destination, target);
    assert!(res.contains(&r));

    let n = 3;
    let edges = vec![vec![0, 1, -1], vec![0, 2, 5]];
    let source = 0;
    let destination = 2;
    let target = 6;
    let res: Vec<Vec<i32>> = vec![];
    assert_eq!(Solution::modified_graph_edges(n, edges, source, destination, target), res);

    let n = 4;
    let edges = vec![vec![1, 0, 4], vec![1, 2, 3], vec![2, 3, 5], vec![0, 3, -1]];
    let source = 0;
    let destination = 2;
    let target = 6;
    let res = vec![vec![1, 0, 4], vec![1, 2, 3], vec![2, 3, 5], vec![0, 3, 1]];
    assert_eq!(Solution::modified_graph_edges(n, edges, source, destination, target), res);
}
