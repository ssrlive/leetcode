#![allow(dead_code)]

/*

// 1786. Number of Restricted Paths From First to Last Node
Medium
827
152
Companies

There is an undirected weighted connected graph. You are given a positive integer n which denotes that the graph has n nodes labeled from 1 to n, and an array edges where each edges[i] = [ui, vi, weighti] denotes that there is an edge between nodes ui and vi with weight equal to weighti.

A path from node start to node end is a sequence of nodes [z0, z1, z2, ..., zk] such that z0 = start and zk = end and there is an edge between zi and zi+1 where 0 <= i <= k-1.

The distance of a path is the sum of the weights on the edges of the path. Let distanceToLastNode(x) denote the shortest distance of a path between node n and node x. A restricted path is a path that also satisfies that distanceToLastNode(zi) > distanceToLastNode(zi+1) where 0 <= i <= k-1.

Return the number of restricted paths from node 1 to node n. Since that number may be too large, return it modulo 109 + 7.

Example 1:

Input: n = 5, edges = [[1,2,3],[1,3,3],[2,3,1],[1,4,2],[5,2,2],[3,5,1],[5,4,10]]
Output: 3
Explanation: Each circle contains the node number in black and its distanceToLastNode value in blue. The three restricted paths are:
1) 1 --> 2 --> 5
2) 1 --> 2 --> 3 --> 5
3) 1 --> 3 --> 5

Example 2:

Input: n = 7, edges = [[1,3,1],[4,1,2],[7,3,4],[2,5,3],[5,6,1],[6,7,2],[7,5,3],[2,6,4]]
Output: 1
Explanation: Each circle contains the node number in black and its distanceToLastNode value in blue. The only restricted path is 1 --> 3 --> 7.

Constraints:

    1 <= n <= 2 * 10^4
    n - 1 <= edges.length <= 4 * 10^4
    edges[i].length == 3
    1 <= ui, vi <= n
    ui != vi
    1 <= weighti <= 10^5
    There is at most one edge between any two nodes.
    There is at least one path between any two nodes.
*/

struct Solution;

impl Solution {
    pub fn count_restricted_paths(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        fn eval(dp: &mut Vec<i32>, dist: &Vec<i32>, graph: &Vec<Vec<(usize, i32)>>, u: usize) -> i32 {
            if dp[u] != -1 {
                return dp[u];
            }
            dp[u] = 0;
            for (v, _) in &graph[u] {
                if dist[*v] <= dist[u] {
                    continue;
                }
                dp[u] = (dp[u] + eval(dp, dist, graph, *v)) % 1_000_000_007;
            }
            dp[u]
        }

        use std::collections::BinaryHeap;
        let n = n as usize;
        let mut graph = vec![vec![]; n];

        for e in edges {
            let (u, v, w) = (e[0] as usize - 1, e[1] as usize - 1, e[2]);
            graph[u].push((v, w));
            graph[v].push((u, w));
        }

        let mut dist = vec![i32::MAX; n];
        let mut pq = BinaryHeap::<(i32, usize)>::new();

        dist[n - 1] = 0i32;
        pq.push((0, n - 1));

        while let Some((d, u)) = pq.pop() {
            let d = -d;
            if dist[u] < d {
                continue;
            }
            for (v, w) in &graph[u] {
                if dist[*v] <= d + w {
                    continue;
                }
                dist[*v] = d + w;
                pq.push((-dist[*v], *v));
            }
        }

        let mut dp = vec![-1; n];
        dp[0] = 1;
        eval(&mut dp, &dist, &graph, n - 1)
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            5,
            vec![
                vec![1, 2, 3],
                vec![1, 3, 3],
                vec![2, 3, 1],
                vec![1, 4, 2],
                vec![5, 2, 2],
                vec![3, 5, 1],
                vec![5, 4, 10],
            ],
            3,
        ),
        (
            7,
            vec![
                vec![1, 3, 1],
                vec![4, 1, 2],
                vec![7, 3, 4],
                vec![2, 5, 3],
                vec![5, 6, 1],
                vec![6, 7, 2],
                vec![7, 5, 3],
                vec![2, 6, 4],
            ],
            1,
        ),
    ];
    for (n, edges, expected) in cases {
        assert_eq!(Solution::count_restricted_paths(n, edges), expected);
    }
}
