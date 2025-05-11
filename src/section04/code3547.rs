#![allow(dead_code)]

// 3547. Maximum Sum of Edge Values in a Graph
// https://leetcode.com/problems/maximum-sum-of-edge-values-in-a-graph
// https://leetcode.cn/problems/maximum-sum-of-edge-values-in-a-graph
//
// Hard
//
// You are given an undirected graph of n nodes, numbered from 0 to n - 1. Each node is connected to at most 2 other nodes.
//
// The graph consists of m edges, represented by a 2D array edges, where edges[i] = [ai, bi]
// indicates that there is an edge between nodes ai and bi.
//
// You have to assign a unique value from 1 to n to each node. The value of an edge will be the product
// of the values assigned to the two nodes it connects.
//
// Your score is the sum of the values of all edges in the graph.
//
// Return the maximum score you can achieve.
//
// Example 1:
//
// Input: n = 7, edges = [[0,1],[1,2],[2,0],[3,4],[4,5],[5,6]]
//
// Output: 130
//
// Explanation:
//
// The diagram above illustrates an optimal assignment of values to nodes.
// The sum of the values of the edges is: (7 * 6) + (7 * 5) + (6 * 5) + (1 * 3) + (3 * 4) + (4 * 2) = 130.
//
// Example 2:
//
// Input: n = 6, edges = [[0,3],[4,5],[2,0],[1,3],[2,4],[1,5]]
//
// Output: 82
//
// Explanation:
//
// The diagram above illustrates an optimal assignment of values to nodes. The sum of the values of the
// edges is: (1 * 2) + (2 * 4) + (4 * 6) + (6 * 5) + (5 * 3) + (3 * 1) = 82.
//
// Constraints:
//
//     1 <= n <= 5 * 10^4
//     m == edges.length
//     1 <= m <= n
//     edges[i].length == 2
//     0 <= ai, bi < n
//     ai != bi
//     There are no repeated edges.
//     Each node is connected to at most 2 other nodes.
//

struct Solution;

impl Solution {
    pub fn max_score(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        fn compute_graph(size: i64, n: i64, is_cycle: bool) -> i64 {
            let mut ans = 0;
            for i in 0..(size - 1) / 2 {
                ans += (n - i * 2) * (n - i * 2 - 2);
            }
            for i in 0..size / 2 {
                ans += (n - i * 2 + if i > 0 { 1 } else { 0 }) * (n - i * 2 - 1);
            }
            if is_cycle {
                if size % 2 == 1 {
                    ans += (n - (size / 2) * 2) * (n - (size / 2) * 2 + 1);
                } else {
                    ans += (n - ((size / 2) - 1) * 2) * (n - (size / 2) * 2 + 1);
                }
            }
            ans
        }
        fn dfs(node: i64, adj: &Vec<Vec<i64>>, parent: i64, visited: &mut Vec<bool>, size: &mut i64) -> bool {
            visited[node as usize] = true;
            *size += 1;
            for &n in &adj[node as usize] {
                if !visited[n as usize] {
                    if dfs(n, adj, node, visited, size) {
                        return true;
                    }
                } else if n != parent {
                    return true;
                }
            }
            false
        }

        let mut n = n as i64;
        let mut adj = vec![vec![]; n as usize];
        let mut cycles = vec![];
        let mut non_cycles = vec![];
        let mut visited = vec![false; n as usize];
        for e in &edges {
            adj[e[0] as usize].push(e[1] as i64);
            adj[e[1] as usize].push(e[0] as i64);
        }
        for i in 0..n {
            if visited[i as usize] {
                continue;
            }
            let mut cnt = 0;
            if dfs(i, &adj, -1, &mut visited, &mut cnt) {
                cycles.push((cnt, i));
            } else {
                non_cycles.push((cnt, i));
            }
        }
        cycles.sort_by(|a, b| b.cmp(a));
        non_cycles.sort_by(|a, b| b.cmp(a));
        let mut ans = 0;
        for c in &cycles {
            ans += compute_graph(c.0, n, true);
            n -= c.0;
        }
        for nc in &non_cycles {
            ans += compute_graph(nc.0, n, false);
            n -= nc.0;
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::max_score(7, vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![3, 4], vec![4, 5], vec![5, 6]]),
        130
    );
    assert_eq!(
        Solution::max_score(6, vec![vec![0, 3], vec![4, 5], vec![2, 0], vec![1, 3], vec![2, 4], vec![1, 5]]),
        82
    );
}
