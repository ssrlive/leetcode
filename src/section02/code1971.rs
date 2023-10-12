#![allow(dead_code)]

/*

// 1971. Find if Path Exists in Graph
// https://leetcode.com/problems/find-if-path-exists-in-graph/
// https://leetcode.cn/problems/find-if-path-exists-in-graph/
//
// Easy
//
// There is a bi-directional graph with n vertices, where each vertex is labeled from 0 to n - 1 (inclusive). The edges in the graph are represented as a 2D integer array edges, where each edges[i] = [ui, vi] denotes a bi-directional edge between vertex ui and vertex vi. Every vertex pair is connected by at most one edge, and no vertex has an edge to itself.

You want to determine if there is a valid path that exists from vertex source to vertex destination.

Given edges and the integers n, source, and destination, return true if there is a valid path from source to destination, or false otherwise.

Example 1:

Input: n = 3, edges = [[0,1],[1,2],[2,0]], source = 0, destination = 2
Output: true
Explanation: There are two paths from vertex 0 to vertex 2:
- 0 → 1 → 2
- 0 → 2

Example 2:

Input: n = 6, edges = [[0,1],[0,2],[3,5],[5,4],[4,3]], source = 0, destination = 5
Output: false
Explanation: There is no path from vertex 0 to vertex 5.

Constraints:

    1 <= n <= 2 * 10^5
    0 <= edges.length <= 2 * 10^5
    edges[i].length == 2
    0 <= ui, vi <= n - 1
    ui != vi
    0 <= source, destination <= n - 1
    There are no duplicate edges.
    There are no self edges.
*/

struct Solution;

impl Solution {
    pub fn valid_path(_n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        use std::collections::{HashMap, HashSet, VecDeque};
        let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
        for head in edges {
            graph.entry(head[0]).or_default().insert(head[1]);
            graph.entry(head[1]).or_default().insert(head[0]);
        }
        let mut visited = HashSet::<i32>::new();
        let mut next_batch = VecDeque::<i32>::new();
        next_batch.push_back(source);
        while let Some(node) = next_batch.pop_front() {
            if node == destination {
                return true;
            }
            if let Some(connections) = graph.get(&node) {
                for &con in connections {
                    if !visited.contains(&con) {
                        visited.insert(con);
                        next_batch.push_back(con);
                    }
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    let cases = vec![
        (3, vec![vec![0, 1], vec![1, 2], vec![2, 0]], 0, 2, true),
        (6, vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]], 0, 5, false),
    ];
    for (n, edges, source, destination, expected) in cases {
        assert_eq!(Solution::valid_path(n, edges, source, destination), expected);
    }
}
