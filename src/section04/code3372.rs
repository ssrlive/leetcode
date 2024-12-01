#![allow(dead_code)]

// 3372. Maximize the Number of Target Nodes After Connecting Trees I
// https://leetcode.com/problems/maximize-the-number-of-target-nodes-after-connecting-trees-i/
// https://leetcode.cn/problems/maximize-the-number-of-target-nodes-after-connecting-trees-i/
//
// Medium
//
// There exist two undirected trees with n and m nodes, with distinct labels in ranges [0, n - 1] and [0, m - 1], respectively.
//
// You are given two 2D integer arrays edges1 and edges2 of lengths n - 1 and m - 1, respectively, where edges1[i] = [ai, bi]
// indicates that there is an edge between nodes ai and bi in the first tree and edges2[i] = [ui, vi] indicates that there is an
// edge between nodes ui and vi in the second tree. You are also given an integer k.
//
// Node u is target to node v if the number of edges on the path from u to v is less than or equal to k. Note that a node is always target to itself.
//
// Return an array of n integers answer, where answer[i] is the maximum possible number of nodes target to node i of the first
// tree if you have to connect one node from the first tree to another node in the second tree.
//
// Note that queries are independent from each other. That is, for every query you will remove the added edge before proceeding to the next query.
//
// Example 1:
//
// Input: edges1 = [[0,1],[0,2],[2,3],[2,4]], edges2 = [[0,1],[0,2],[0,3],[2,7],[1,4],[4,5],[4,6]], k = 2
//
// Output: [9,7,9,8,8]
//
// Explanation:
//
// For i = 0, connect node 0 from the first tree to node 0 from the second tree.
// For i = 1, connect node 1 from the first tree to node 0 from the second tree.
// For i = 2, connect node 2 from the first tree to node 4 from the second tree.
// For i = 3, connect node 3 from the first tree to node 4 from the second tree.
// For i = 4, connect node 4 from the first tree to node 4 from the second tree.
//
// Example 2:
//
// Input: edges1 = [[0,1],[0,2],[0,3],[0,4]], edges2 = [[0,1],[1,2],[2,3]], k = 1
//
// Output: [6,3,3,3,3]
//
// Explanation:
//
// For every i, connect node i of the first tree with any node of the second tree.
//
// Constraints:
//
// 2 <= n, m <= 1000
// edges1.length == n - 1
// edges2.length == m - 1
// edges1[i].length == edges2[i].length == 2
// edges1[i] = [a[i], b[i]]
// 0 <= a[i], b[i] < n
// edges2[i] = [u[i], v[i]]
// 0 <= u[i], v[i] < m
// The input is generated such that edges1 and edges2 represent valid trees.
// 0 <= k <= 1000
//

struct Solution;

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        fn build_graph(edges: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let mut g = vec![vec![]; edges.len() + 1];
            for e in edges {
                g[e[0] as usize].push(e[1]);
                g[e[1] as usize].push(e[0]);
            }
            g
        }

        fn dfs(g: &Vec<Vec<i32>>, root: i32, par: i32, k: i32, mut count: i32) -> i32 {
            if k < 0 {
                return 0;
            }
            for &node in &g[root as usize] {
                if node != par {
                    count += dfs(g, node, root, k - 1, 1);
                }
            }
            count
        }

        let g1 = build_graph(&edges1);
        let g2 = build_graph(&edges2);
        let mut count = 0;
        let n = edges1.len() as i32 + 1;
        let m = edges2.len() as i32 + 1;
        let mut ans = vec![];
        for i in 0..m {
            count = count.max(dfs(&g2, i, -1, k - 1, 1));
        }
        for i in 0..n {
            ans.push(count + dfs(&g1, i, -1, k, 1));
        }
        ans
    }
}

#[test]
fn test() {
    let edges1 = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]];
    let edges2 = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![2, 7], vec![1, 4], vec![4, 5], vec![4, 6]];
    let k = 2;
    let output = vec![9, 7, 9, 8, 8];
    assert_eq!(Solution::max_target_nodes(edges1, edges2, k), output);

    let edges1 = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]];
    let edges2 = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
    let k = 1;
    let output = vec![6, 3, 3, 3, 3];
    assert_eq!(Solution::max_target_nodes(edges1, edges2, k), output);
}
