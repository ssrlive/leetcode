#![allow(dead_code)]

// 3600. Maximize Spanning Tree Stability with Upgrades
// https://leetcode.com/problems/maximize-spanning-tree-stability-with-upgrades/
// https://leetcode.cn/problems/maximize-spanning-tree-stability-with-upgrades/
//
// Hard
//
// You are given an integer n, representing n nodes numbered from 0 to n - 1 and a list of edges, where edges[i] = [ui, vi, si, musti]:
//
//     ui and vi indicates an undirected edge between nodes ui and vi.
//     si is the strength of the edge.
//     musti is an integer (0 or 1). If musti == 1, the edge must be included in the spanning tree. These edges cannot be upgraded.
//
// You are also given an integer k, the maximum number of upgrades you can perform.
// Each upgrade doubles the strength of an edge, and each eligible edge (with musti == 0) can be upgraded at most once.
//
// The stability of a spanning tree is defined as the minimum strength score among all edges included in it.
//
// Return the maximum possible stability of any valid spanning tree. If it is impossible to connect all nodes, return -1.
//
// Note: A spanning tree of a graph with n nodes is a subset of the edges that connects all nodes together
// (i.e. the graph is connected) without forming any cycles, and uses exactly n - 1 edges.
//
// Example 1:
//
// Input: n = 3, edges = [[0,1,2,1],[1,2,3,0]], k = 1
//
// Output: 2
//
// Explanation:
//
//     Edge [0,1] with strength = 2 must be included in the spanning tree.
//     Edge [1,2] is optional and can be upgraded from 3 to 6 using one upgrade.
//     The resulting spanning tree includes these two edges with strengths 2 and 6.
//     The minimum strength in the spanning tree is 2, which is the maximum possible stability.
//
// Example 2:
//
// Input: n = 3, edges = [[0,1,4,0],[1,2,3,0],[0,2,1,0]], k = 2
//
// Output: 6
//
// Explanation:
//
//     Since all edges are optional and up to k = 2 upgrades are allowed.
//     Upgrade edges [0,1] from 4 to 8 and [1,2] from 3 to 6.
//     The resulting spanning tree includes these two edges with strengths 8 and 6.
//     The minimum strength in the tree is 6, which is the maximum possible stability.
//
// Example 3:
//
// Input: n = 3, edges = [[0,1,1,1],[1,2,1,1],[2,0,1,1]], k = 0
//
// Output: -1
//
// Explanation:
//
//     All edges are mandatory and form a cycle, which violates the spanning tree property of acyclicity. Thus, the answer is -1.
//
// Constraints:
//
//     2 <= n <= 10^5
//     1 <= edges.length <= 10^5
//     edges[i] = [ui, vi, si, musti]
//     0 <= ui, vi < n
//     ui != vi
//     1 <= si <= 10^5
//     musti is either 0 or 1.
//     0 <= k <= n
//     There are no duplicate edges.
//

struct Solution;

impl Solution {
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        fn find(i: usize, ds: &mut Vec<i32>) -> usize {
            if ds[i] < 0 {
                i
            } else {
                ds[i] = find(ds[i] as usize, ds) as i32;
                ds[i] as usize
            }
        }

        let mut s_u_v: Vec<(i32, i32, i32)> = Vec::new();
        let mut ds = vec![-1; n as usize];
        let mut used_e = 0;
        let mut res = 200_000;
        let mut min_single = 200_000;
        let mut min_double = 100_000;
        for e in &edges {
            if e[3] == 1 {
                let a = find(e[0] as usize, &mut ds);
                let b = find(e[1] as usize, &mut ds);
                if a == b {
                    return -1;
                } else {
                    used_e += 1;
                    ds[b] = a as i32;
                    res = res.min(e[2]);
                }
            } else {
                s_u_v.push((e[2], e[1], e[0]));
            }
        }
        s_u_v.sort_by(|a, b| b.0.cmp(&a.0));
        for (s, u, v) in s_u_v {
            let a = find(u as usize, &mut ds);
            let b = find(v as usize, &mut ds);
            if a != b {
                ds[b] = a as i32;
                used_e += 1;
                if used_e == n as usize - 1 - k as usize {
                    min_single = s;
                }
                min_double = s;
            }
        }
        if used_e == n as usize - 1 {
            return res.min(min_single).min(min_double * if k > 0 { 2 } else { 1 });
        }
        -1
    }
}

#[test]
fn test() {
    let n = 3;
    let edges = vec![vec![0, 1, 2, 1], vec![1, 2, 3, 0]];
    let k = 1;
    assert_eq!(Solution::max_stability(n, edges, k), 2);

    let n = 3;
    let edges = vec![vec![0, 1, 4, 0], vec![1, 2, 3, 0], vec![0, 2, 1, 0]];
    let k = 2;
    assert_eq!(Solution::max_stability(n, edges, k), 6);

    let n = 3;
    let edges = vec![vec![0, 1, 1, 1], vec![1, 2, 1, 1], vec![2, 0, 1, 1]];
    let k = 0;
    assert_eq!(Solution::max_stability(n, edges, k), -1);
}
