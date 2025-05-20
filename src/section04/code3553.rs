#![allow(dead_code)]

// 3553. Minimum Weighted Subgraph With the Required Paths II
// https://leetcode.com/problems/minimum-weighted-subgraph-with-the-required-paths-ii/
// https://leetcode.cn/problems/minimum-weighted-subgraph-with-the-required-paths-ii/
//
// Hard
//
// You are given an undirected weighted tree with n nodes, numbered from 0 to n - 1. It is represented by a 2D integer array
// edges of length n - 1, where edges[i] = [ui, vi, wi] indicates that there is an edge between nodes ui and vi with weight wi.​
//
// Additionally, you are given a 2D integer array queries, where queries[j] = [src1j, src2j, destj].
//
// Return an array answer of length equal to queries.length, where answer[j] is the minimum total weight of a subtree such
// that it is possible to reach destj from both src1j and src2j using edges in this subtree.
//
// A subtree here is any connected subset of nodes and edges of the original tree forming a valid tree.
//
// Example 1:
//
// Input: edges = [[0,1,2],[1,2,3],[1,3,5],[1,4,4],[2,5,6]], queries = [[2,3,4],[0,2,5]]
//
// Output: [12,11]
//
// Explanation:
//
// The blue edges represent one of the subtrees that yield the optimal answer.
//
//     answer[0]: The total weight of the selected subtree that ensures a path from src1 = 2 and src2 = 3 to dest = 4 is 3 + 5 + 4 = 12.
//
//     answer[1]: The total weight of the selected subtree that ensures a path from src1 = 0 and src2 = 2 to dest = 5 is 2 + 3 + 6 = 11.
//
// Example 2:
//
// Input: edges = [[1,0,8],[0,2,7]], queries = [[0,1,2]]
//
// Output: [15]
//
// Explanation:
//
//     answer[0]: The total weight of the selected subtree that ensures a path from src1 = 0 and src2 = 1 to dest = 2 is 8 + 7 = 15.
//
// Constraints:
//
//     3 <= n <= 10^5
//     edges.length == n - 1
//     edges[i].length == 3
//     0 <= ui, vi < n
//     1 <= wi <= 10^4
//     1 <= queries.length <= 10^5
//     queries[j].length == 3
//     0 <= src1j, src2j, destj < n
//     src1j, src2j, and destj are pairwise distinct.
//     The input is generated such that edges represents a valid tree.
//

struct Solution;

impl Solution {
    pub fn minimum_weight(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        #[allow(clippy::too_many_arguments)]
        fn dfs(
            u: i32,
            par: i32,
            lvl: i32,
            wt: i32,
            levels: &mut Vec<i32>,
            weights: &mut Vec<i32>,
            graph: &Vec<Vec<(i32, i32)>>,
            lca: &mut Vec<Vec<i32>>,
        ) {
            levels[u as usize] = lvl;
            weights[u as usize] = wt;
            lca[u as usize][0] = par;

            for (v, w) in &graph[u as usize] {
                if *v != par {
                    dfs(*v, u, lvl + 1, wt + w, levels, weights, graph, lca);
                }
            }
        }
        fn init(n: i32, max_n: i32, lca: &mut [Vec<i32>]) {
            for i in 1..=max_n {
                for j in 0..n {
                    if lca[j as usize][i as usize - 1] != -1 {
                        let par = lca[j as usize][i as usize - 1];
                        lca[j as usize][i as usize] = lca[par as usize][i as usize - 1];
                    }
                }
            }
        }
        fn find_lca(mut a: i32, mut b: i32, max_n: i32, levels: &[i32], lca: &[Vec<i32>]) -> i32 {
            if levels[a as usize] > levels[b as usize] {
                std::mem::swap(&mut a, &mut b);
            }

            let mut d = levels[b as usize] - levels[a as usize];
            while d > 0 {
                let jump = d.ilog2();
                b = lca[b as usize][jump as usize];
                d -= 1 << jump;
            }

            if a == b {
                return a;
            }

            for i in (0..=max_n).rev() {
                if lca[a as usize][i as usize] != -1 && lca[a as usize][i as usize] != lca[b as usize][i as usize] {
                    a = lca[a as usize][i as usize];
                    b = lca[b as usize][i as usize];
                }
            }

            lca[a as usize][0]
        }

        fn get_distance(u: i32, v: i32, max_n: i32, levels: &[i32], weights: &[i32], lca: &[Vec<i32>]) -> i32 {
            let lca = find_lca(u, v, max_n, levels, lca);
            weights[u as usize] + weights[v as usize] - 2 * weights[lca as usize]
        }

        let n = edges.len() as i32 + 1;
        let m = queries.len() as i32;
        let mut graph = vec![vec![]; n as usize];
        for edge in &edges {
            graph[edge[0] as usize].push((edge[1], edge[2]));
            graph[edge[1] as usize].push((edge[0], edge[2]));
        }
        let max_n = n.ilog2() as i32;
        let mut levels = vec![0; n as usize];
        let mut weights = vec![0; n as usize];
        let mut lca = vec![vec![-1; (max_n + 1) as usize]; n as usize];
        dfs(0, -1, 0, 0, &mut levels, &mut weights, &graph, &mut lca);
        init(n, max_n, &mut lca);
        let mut ans = vec![0; m as usize];
        for i in 0..m {
            let src1 = queries[i as usize][0];
            let src2 = queries[i as usize][1];
            let dest = queries[i as usize][2];

            let total = (get_distance(src1, dest, max_n, &levels, &weights, &lca)
                + get_distance(src2, dest, max_n, &levels, &weights, &lca)
                + get_distance(src1, src2, max_n, &levels, &weights, &lca))
                / 2;
            ans[i as usize] = total;
        }
        ans
    }
}

#[test]
fn test() {
    let edges = vec![vec![0, 1, 2], vec![1, 2, 3], vec![1, 3, 5], vec![1, 4, 4], vec![2, 5, 6]];
    let queries = vec![vec![2, 3, 4], vec![0, 2, 5]];
    let result = Solution::minimum_weight(edges, queries);
    assert_eq!(result, vec![12, 11]);

    let edges = vec![vec![1, 0, 8], vec![0, 2, 7]];
    let queries = vec![vec![0, 1, 2]];
    let result = Solution::minimum_weight(edges, queries);
    assert_eq!(result, vec![15]);

    let edges = vec![vec![0, 1, 10], vec![2, 0, 2], vec![3, 2, 8]];
    let queries = vec![vec![1, 3, 2], vec![2, 1, 3], vec![2, 3, 1], vec![1, 3, 0]];
    let result = Solution::minimum_weight(edges, queries);
    assert_eq!(result, vec![20, 20, 20, 20]);
}
