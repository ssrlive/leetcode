#![allow(dead_code)]

/*

// 2203. Minimum Weighted Subgraph With the Required Paths
// https://leetcode.com/problems/minimum-weighted-subgraph-with-the-required-paths/
// https://leetcode.cn/problems/minimum-weighted-subgraph-with-the-required-paths/
//
// Hard
//
// You are given an integer n denoting the number of nodes of a weighted directed graph. The nodes are numbered from 0 to n - 1.

You are also given a 2D integer array edges where edges[i] = [fromi, toi, weighti] denotes that there exists a directed edge from fromi to toi with weight weighti.

Lastly, you are given three distinct integers src1, src2, and dest denoting three distinct nodes of the graph.

Return the minimum weight of a subgraph of the graph such that it is possible to reach dest from both src1 and src2 via a set of edges of this subgraph. In case such a subgraph does not exist, return -1.

A subgraph is a graph whose vertices and edges are subsets of the original graph. The weight of a subgraph is the sum of weights of its constituent edges.

Example 1:

Input: n = 6, edges = [[0,2,2],[0,5,6],[1,0,3],[1,4,5],[2,1,1],[2,3,3],[2,3,4],[3,4,2],[4,5,1]], src1 = 0, src2 = 1, dest = 5
Output: 9
Explanation:
The above figure represents the input graph.
The blue edges represent one of the subgraphs that yield the optimal answer.
Note that the subgraph [[1,0,3],[0,5,6]] also yields the optimal answer. It is not possible to get a subgraph with less weight satisfying all the constraints.

Example 2:

Input: n = 3, edges = [[0,1,1],[2,1,1]], src1 = 0, src2 = 1, dest = 2
Output: -1
Explanation:
The above figure represents the input graph.
It can be seen that there does not exist any path from node 1 to node 2, hence there are no subgraphs satisfying all the constraints.

Constraints:

    3 <= n <= 10^5
    0 <= edges.length <= 10^5
    edges[i].length == 3
    0 <= fromi, toi, src1, src2, dest <= n - 1
    fromi != toi
    src1, src2, and dest are pairwise distinct.
    1 <= weight[i] <= 10^5
*/

struct Solution;

impl Solution {
    pub fn minimum_weight(n: i32, edges: Vec<Vec<i32>>, src1: i32, src2: i32, dest: i32) -> i64 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        let mut adj_rev = vec![vec![]; n];

        for e in edges {
            adj[e[0] as usize].push((e[1] as usize, e[2] as i64));
            adj_rev[e[1] as usize].push((e[0] as usize, e[2] as i64));
        }

        fn dijkstra(adj: &Vec<Vec<(usize, i64)>>, s: usize) -> Vec<Option<i64>> {
            let mut dist = vec![None; adj.len()];
            dist[s] = Some(0);

            let mut pqueue = BinaryHeap::new();
            pqueue.push((Reverse(0), s));

            while let Some((Reverse(p), u)) = pqueue.pop() {
                if matches!(dist[u], Some(d) if p > d) {
                    continue;
                }

                for &(v, w) in &adj[u] {
                    if !matches!(dist[v], Some(d) if d <= p + w) {
                        dist[v] = Some(p + w);
                        pqueue.push((Reverse(p + w), v));
                    }
                }
            }
            dist
        }

        let dist_s1 = dijkstra(&adj, src1 as usize);
        let dist_s2 = dijkstra(&adj, src2 as usize);
        let dist_de = dijkstra(&adj_rev, dest as usize);

        let mut min_dist = None;
        for u in 0..n {
            if let (Some(d1), Some(d2), Some(dd)) = (dist_s1[u], dist_s2[u], dist_de[u]) {
                if !matches!(min_dist, Some(d) if d <= d1 + d2 + dd) {
                    min_dist = Some(d1 + d2 + dd);
                }
            }
        }
        min_dist.unwrap_or(-1)
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            6,
            vec![
                vec![0, 2, 2],
                vec![0, 5, 6],
                vec![1, 0, 3],
                vec![1, 4, 5],
                vec![2, 1, 1],
                vec![2, 3, 3],
                vec![2, 3, 4],
                vec![3, 4, 2],
                vec![4, 5, 1],
            ],
            0,
            1,
            5,
            9,
        ),
        (3, vec![vec![0, 1, 1], vec![2, 1, 1]], 0, 1, 2, -1),
    ];
    for (n, edges, src1, src2, dest, expected) in cases {
        assert_eq!(Solution::minimum_weight(n, edges, src1, src2, dest), expected);
    }
}
