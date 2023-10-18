#![allow(dead_code)]

// 2858. Minimum Edge Reversals So Every Node Is Reachable
// https://leetcode.com/problems/minimum-edge-reversals-so-every-node-is-reachable/
// https://leetcode.cn/problems/minimum-edge-reversals-so-every-node-is-reachable/
//
// Hard
//
// There is a simple directed graph with n nodes labeled from 0 to n - 1. The graph would form a tree if its edges were bi-directional.
//
// You are given an integer n and a 2D integer array edges, where edges[i] = [ui, vi] represents a directed edge going from node ui to node vi.
//
// An edge reversal changes the direction of an edge, i.e., a directed edge going from node ui to node vi becomes a directed edge going from node vi to node ui.
//
// For every node i in the range [0, n - 1], your task is to independently calculate the minimum number of edge reversals
// required so it is possible to reach any other node starting from node i through a sequence of directed edges.
//
// Return an integer array answer, where answer[i] is the minimum number of edge reversals required
// so it is possible to reach any other node starting from node i through a sequence of directed edges.
//
// Example 1:
//              2
//            /   \
//           0     1
//            \
//              3
//
// Input: n = 4, edges = [[2,0],[2,1],[1,3]]
// Output: [1,1,0,2]
// Explanation: The image above shows the graph formed by the edges.
// For node 0: after reversing the edge [2,0], it is possible to reach any other node starting from node 0.
// So, answer[0] = 1.
// For node 1: after reversing the edge [2,1], it is possible to reach any other node starting from node 1.
// So, answer[1] = 1.
// For node 2: it is already possible to reach any other node starting from node 2.
// So, answer[2] = 0.
// For node 3: after reversing the edges [1,3] and [2,1], it is possible to reach any other node starting from node 3.
// So, answer[3] = 2.
//
// Example 2:
//              2
//            /   \
//           1     0
//
// Input: n = 3, edges = [[1,2],[2,0]]
// Output: [2,0,1]
// Explanation: The image above shows the graph formed by the edges.
// For node 0: after reversing the edges [2,0] and [1,2], it is possible to reach any other node starting from node 0.
// So, answer[0] = 2.
// For node 1: it is already possible to reach any other node starting from node 1.
// So, answer[1] = 0.
// For node 2: after reversing the edge [1, 2], it is possible to reach any other node starting from node 2.
// So, answer[2] = 1.
//
// Constraints:
//
// 2 <= n <= 10^5
// edges.length == n - 1
// edges[i].length == 2
// 0 <= ui == edges[i][0] < n
// 0 <= vi == edges[i][1] < n
// ui != vi
// The input is generated such that if the edges were bi-directional, the graph would be a tree.
//

struct Solution;

impl Solution {
    pub fn min_edge_reversals(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj = vec![vec![]; n as usize];
        for i in edges {
            adj[i[0] as usize].push((i[1] as usize, 0));
            adj[i[1] as usize].push((i[0] as usize, 1));
        }
        let mut ans = vec![0; n as usize];

        fn f(curr: usize, par: i32, ans: &mut Vec<i32>, adj: &Vec<Vec<(usize, i32)>>) {
            for (next, wt) in adj[curr].iter() {
                if *next as i32 == par {
                    continue;
                }
                ans[0] += wt;
                f(*next, curr as i32, ans, adj);
            }
        }
        f(0, -1, &mut ans, &adj);

        fn dfs(curr: usize, par: i32, ans: &mut Vec<i32>, adj: &Vec<Vec<(usize, i32)>>) {
            for (next, wt) in adj[curr].iter() {
                if *next as i32 == par {
                    continue;
                }
                ans[*next] = ans[curr] + if *wt == 1 { -1 } else { 1 };
                dfs(*next, curr as i32, ans, adj);
            }
        }
        dfs(0, -1, &mut ans, &adj);

        ans
    }
}

#[test]
fn test() {
    let n = 4;
    let edges = vec![vec![2, 0], vec![2, 1], vec![1, 3]];
    let ans = vec![1, 1, 0, 2];
    assert_eq!(Solution::min_edge_reversals(n, edges), ans);

    let n = 3;
    let edges = vec![vec![1, 2], vec![2, 0]];
    let ans = vec![2, 0, 1];
    assert_eq!(Solution::min_edge_reversals(n, edges), ans);
}
