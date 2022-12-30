#![allow(dead_code)]

// 882. Reachable Nodes In Subdivided Graph
// https://leetcode.com/problems/reachable-nodes-in-subdivided-graph/
// https://leetcode.cn/problems/reachable-nodes-in-subdivided-graph/
//
// You are given an undirected graph (the "original graph") with n nodes labeled from 0 to n - 1.
// You decide to subdivide each edge in the graph into a chain of nodes, with the number of new nodes varying between each edge.
//
// The graph is given as a 2D array of edges where edges[i] = [ui, vi, cnti] indicates that there is an edge
// between nodes ui and vi in the original graph, and cnti is the total number of new nodes that you will subdivide the edge into.
// Note that cnti == 0 means you will not subdivide the edge.
//
// To subdivide the edge [ui, vi], replace it with (cnti + 1) new edges and cnti new nodes.
// The new nodes are x1, x2, ..., xcnti, and the new edges are [ui, x1], [x1, x2], [x2, x3], ..., [xcnti-1, xcnti], [xcnti, vi].
//
// In this new graph, you want to know how many nodes are reachable from the node 0,
// where a node is reachable if the distance is maxMoves or less.
//
// Given the original graph and maxMoves, return the number of nodes that are reachable from node 0 in the new graph.
//
// Example 1:
//
// Input: edges = [[0,1,10],[0,2,1],[1,2,2]], maxMoves = 6, n = 3
// Output: 13
// Explanation: The edge subdivisions are shown in the image above.
// The nodes that are reachable are highlighted in yellow.
//
// Example 2:
//
// Input: edges = [[0,1,4],[1,2,6],[0,2,8],[1,3,1]], maxMoves = 10, n = 4
// Output: 23
//
// Example 3:
//
// Input: edges = [[1,2,4],[1,4,5],[1,3,1],[2,3,4],[3,4,5]], maxMoves = 17, n = 5
// Output: 1
// Explanation: Node 0 is disconnected from the rest of the graph, so only node 0 is reachable.
//
// Constraints:
//
// - 0 <= edges.length <= min(n * (n - 1) / 2, 10^4)
// - edges[i].length == 3
// - 0 <= ui < vi < n
// - There are no multiple edges in the graph.
// - 0 <= cnti <= 10^4
// - 0 <= maxMoves <= 10^9
// - 1 <= n <= 3000
//

struct Solution;

impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut graph = vec![Vec::new(); n as usize];
        for edge in &edges {
            graph[edge[0] as usize].push((edge[1] as usize, edge[2]));
            graph[edge[1] as usize].push((edge[0] as usize, edge[2]));
        }
        let mut remains = vec![None; n as usize];
        let mut bh = BinaryHeap::new();
        bh.push((Reverse(0), 0));
        let mut answer = 0;
        while let Some((Reverse(min), u)) = bh.pop() {
            if min > max_moves {
                break;
            }
            if remains[u].is_some() {
                continue;
            }
            answer += 1;
            remains[u] = Some(max_moves - min);
            for &(v, c) in graph[u].iter().filter(|(v, _)| remains[*v].is_none()) {
                bh.push((Reverse(min + c + 1), v));
            }
        }
        for edge in &edges {
            answer += edge[2].min(remains[edge[0] as usize].unwrap_or(0) + remains[edge[1] as usize].unwrap_or(0));
        }
        answer
    }
}

#[test]
fn test() {
    let edges = vec![vec![0, 1, 10], vec![0, 2, 1], vec![1, 2, 2]];
    let max_moves = 6;
    let n = 3;
    assert_eq!(Solution::reachable_nodes(edges, max_moves, n), 13);

    let edges = vec![vec![0, 1, 4], vec![1, 2, 6], vec![0, 2, 8], vec![1, 3, 1]];
    let max_moves = 10;
    let n = 4;
    assert_eq!(Solution::reachable_nodes(edges, max_moves, n), 23);

    let edges = vec![
        vec![1, 2, 4],
        vec![1, 4, 5],
        vec![1, 3, 1],
        vec![2, 3, 4],
        vec![3, 4, 5],
    ];
    let max_moves = 17;
    let n = 5;
    assert_eq!(Solution::reachable_nodes(edges, max_moves, n), 1);
}
