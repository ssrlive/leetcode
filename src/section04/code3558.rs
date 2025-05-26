#![allow(dead_code)]

// 3558. Number of Ways to Assign Edge Weights I
// https://leetcode.com/problems/number-of-ways-to-assign-edge-weights-i/
// https://leetcode.cn/problems/number-of-ways-to-assign-edge-weights-i/
//
// Medium
//
// There is an undirected tree with n nodes labeled from 1 to n, rooted at node 1. The tree is represented by a 2D integer
// array edges of length n - 1, where edges[i] = [ui, vi] indicates that there is an edge between nodes ui and vi.
//
// Initially, all edges have a weight of 0. You must assign each edge a weight of either 1 or 2.
//
// The cost of a path between any two nodes u and v is the total weight of all edges in the path connecting them.
//
// Select any one node x at the maximum depth. Return the number of ways to assign edge weights in the
// path from node 1 to x such that its total cost is odd.
//
// Since the answer may be large, return it modulo 109 + 7.
//
// Note: Ignore all edges not in the path from node 1 to x.
//
// Example 1:
//
// Input: edges = [[1,2]]
//
// Output: 1
//
// Explanation:
//
//     The path from Node 1 to Node 2 consists of one edge (1 → 2).
//     Assigning weight 1 makes the cost odd, while 2 makes it even. Thus, the number of valid assignments is 1.
//
// Example 2:
//
// Input: edges = [[1,2],[1,3],[3,4],[3,5]]
//
// Output: 2
//
// Explanation:
//
//     The maximum depth is 2, with nodes 4 and 5 at the same depth. Either node can be selected for processing.
//     For example, the path from Node 1 to Node 4 consists of two edges (1 → 3 and 3 → 4).
//     Assigning weights (1,2) or (2,1) results in an odd cost. Thus, the number of valid assignments is 2.
//
// Constraints:
//
//     2 <= n <= 10^5
//     edges.length == n - 1
//     edges[i] == [ui, vi]
//     1 <= ui, vi <= n
//     edges represents a valid tree.
//

struct Solution;

impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
        fn dfs(node: i32, par: i32, depth: i32, graph: &Vec<Vec<i32>>, max_depth: &mut i32) {
            *max_depth = (*max_depth).max(depth);
            for &x in &graph[node as usize] {
                if x != par {
                    dfs(x, node, depth + 1, graph, max_depth);
                }
            }
        }
        fn power_mod(x: i64, y: i64, mod_val: i64) -> i64 {
            let mut result = 1;
            let mut base = x % mod_val;
            let mut exp = y;

            while exp > 0 {
                if exp % 2 == 1 {
                    result = (result * base) % mod_val;
                }
                base = (base * base) % mod_val;
                exp /= 2;
            }
            result
        }
        const MOD: i64 = 1_000_000_007;
        let n = edges.len() as i32 + 1;
        let mut graph = vec![vec![]; (n + 1) as usize];
        for edge in edges {
            graph[edge[0] as usize].push(edge[1]);
            graph[edge[1] as usize].push(edge[0]);
        }
        let mut max_depth = 0;
        dfs(1, -1, 0, &graph, &mut max_depth);
        (power_mod(2, max_depth as i64 - 1, MOD) % MOD) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::assign_edge_weights(vec![vec![1, 2]]), 1);
    assert_eq!(
        Solution::assign_edge_weights(vec![vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]]),
        2
    );
}
