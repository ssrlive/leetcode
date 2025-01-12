#![allow(dead_code)]

// 3419. Minimize the Maximum Edge Weight of Graph
// https://leetcode.com/problems/minimize-the-maximum-edge-weight-of-graph/
// https://leetcode.cn/problems/minimize-the-maximum-edge-weight-of-graph/
//
// Medium
//
// You are given two integers, n and threshold, as well as a directed weighted graph of n nodes numbered from 0 to n - 1.
// The graph is represented by a 2D integer array edges, where edges[i] = [Ai, Bi, Wi] indicates
// that there is an edge going from node Ai to node Bi with weight Wi.
//
// You have to remove some edges from this graph (possibly none), so that it satisfies the following conditions:
//
//     Node 0 must be reachable from all other nodes.
//     The maximum edge weight in the resulting graph is minimized.
//     Each node has at most threshold outgoing edges.
//
// Return the minimum possible value of the maximum edge weight after removing the necessary edges.
// If it is impossible for all conditions to be satisfied, return -1.
//
// Example 1:
//
// Input: n = 5, edges = [[1,0,1],[2,0,2],[3,0,1],[4,3,1],[2,1,1]], threshold = 2
//
// Output: 1
//
// Explanation:
//
// Remove the edge 2 -> 0. The maximum weight among the remaining edges is 1.
//
// Example 2:
//
// Input: n = 5, edges = [[0,1,1],[0,2,2],[0,3,1],[0,4,1],[1,2,1],[1,4,1]], threshold = 1
//
// Output: -1
//
// Explanation:
//
// It is impossible to reach node 0 from node 2.
//
// Example 3:
//
// Input: n = 5, edges = [[1,2,1],[1,3,3],[1,4,5],[2,3,2],[3,4,2],[4,0,1]], threshold = 1
//
// Output: 2
//
// Explanation:
//
// Remove the edges 1 -> 3 and 1 -> 4. The maximum weight among the remaining edges is 2.
//
// Example 4:
//
// Input: n = 5, edges = [[1,2,1],[1,3,3],[1,4,5],[2,3,2],[4,0,1]], threshold = 1
//
// Output: -1
//
// Constraints:
//
//     2 <= n <= 10^5
//     1 <= threshold <= n - 1
//     1 <= edges.length <= min(10^5, n * (n - 1) / 2).
//     edges[i].length == 3
//     0 <= Ai, Bi < n
//     Ai != Bi
//     1 <= Wi <= 10^6
//     There may be multiple edges between a pair of nodes, but they must have unique weights.
//

struct Solution;

impl Solution {
    pub fn min_max_weight(n: i32, edges: Vec<Vec<i32>>, _threshold: i32) -> i32 {
        fn dfs(i: usize, m: i32, ral: &[Vec<(usize, i32)>], vis: &mut [i32]) -> i32 {
            let mut res = 1;
            vis[i] = 1;
            for (j, w) in &ral[i] {
                if *w <= m && vis[*j] == 0 {
                    res += dfs(*j, m, ral, vis);
                }
            }
            res
        }

        let n = n as usize;
        let mut ral = vec![vec![]; n];
        for e in &edges {
            ral[e[1] as usize].push((e[0] as usize, e[2]));
        }
        let mut l = 1;
        let mut r = 1000001;
        while l < r {
            let m = (l + r) / 2;
            let mut vis = vec![0; n];
            if dfs(0, m, &ral, &mut vis) == n as i32 {
                r = m;
            } else {
                l = m + 1;
            }
        }
        if l == 1000001 {
            -1
        } else {
            l
        }
    }
}

#[test]
fn test() {
    let n = 5;
    let edges = vec![vec![1, 0, 1], vec![2, 0, 2], vec![3, 0, 1], vec![4, 3, 1], vec![2, 1, 1]];
    let threshold = 2;
    let res = 1;
    assert_eq!(Solution::min_max_weight(n, edges, threshold), res);

    let n = 5;
    let edges = vec![
        vec![0, 1, 1],
        vec![0, 2, 2],
        vec![0, 3, 1],
        vec![0, 4, 1],
        vec![1, 2, 1],
        vec![1, 4, 1],
    ];
    let threshold = 1;
    let res = -1;
    assert_eq!(Solution::min_max_weight(n, edges, threshold), res);

    let n = 5;
    let edges = vec![
        vec![1, 2, 1],
        vec![1, 3, 3],
        vec![1, 4, 5],
        vec![2, 3, 2],
        vec![3, 4, 2],
        vec![4, 0, 1],
    ];
    let threshold = 1;
    let res = 2;
    assert_eq!(Solution::min_max_weight(n, edges, threshold), res);

    let n = 5;
    let edges = vec![vec![1, 2, 1], vec![1, 3, 3], vec![1, 4, 5], vec![2, 3, 2], vec![4, 0, 1]];
    let threshold = 1;
    let res = -1;
    assert_eq!(Solution::min_max_weight(n, edges, threshold), res);
}
