#![allow(dead_code)]

/*

// 2608. Shortest Cycle in a Graph
// https://leetcode.com/problems/shortest-cycle-in-a-graph/
// https://leetcode.cn/problems/shortest-cycle-in-a-graph/
//
// Hard
//
// There is a bi-directional graph with n vertices, where each vertex is labeled from 0 to n - 1.
// The edges in the graph are represented by a given 2D integer array edges,
// where edges[i] = [ui, vi] denotes an edge between vertex ui and vertex vi.
// Every vertex pair is connected by at most one edge, and no vertex has an edge to itself.

Return the length of the shortest cycle in the graph. If no cycle exists, return -1.

A cycle is a path that starts and ends at the same node, and each edge in the path is used only once.

Example 1:

Input: n = 7, edges = [[0,1],[1,2],[2,0],[3,4],[4,5],[5,6],[6,3]]
Output: 3
Explanation: The cycle with the smallest length is : 0 -> 1 -> 2 -> 0

Example 2:

Input: n = 4, edges = [[0,1],[0,2]]
Output: -1
Explanation: There are no cycles in this graph.

Constraints:

    2 <= n <= 1000
    1 <= edges.length <= 1000
    edges[i].length == 2
    0 <= ui, vi < n
    ui != vi
    There are no repeated edges.
*/

struct Solution;

impl Solution {
    pub fn find_shortest_cycle(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut g = vec![vec![]; n as usize];
        for e in edges.iter() {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        let inf = 10000;
        let mut res = inf;
        let root = |i: i32| {
            let mut dis = vec![inf; n as usize];
            dis[i as usize] = 0;
            let mut bfs = vec![i];
            while !bfs.is_empty() {
                let i = bfs.remove(0) as usize;
                for &jj in g[i].iter() {
                    let j = jj as usize;
                    if dis[j] == inf {
                        dis[j] = 1 + dis[i];
                        bfs.push(jj);
                    } else if dis[i] <= dis[j] {
                        return dis[i] + dis[j] + 1;
                    }
                }
            }
            inf
        };
        for i in 0..n {
            res = res.min(root(i));
        }
        if res < inf { res } else { -1 }
    }
}

#[test]
fn test() {
    let n = 7;
    let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 3]];
    assert_eq!(Solution::find_shortest_cycle(n, edges), 3);

    let n = 4;
    let edges = vec![vec![0, 1], vec![0, 2]];
    assert_eq!(Solution::find_shortest_cycle(n, edges), -1);
}
