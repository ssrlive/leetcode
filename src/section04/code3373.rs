#![allow(dead_code)]

// 3373. Maximize the Number of Target Nodes After Connecting Trees II
// https://leetcode.com/problems/maximize-the-number-of-target-nodes-after-connecting-trees-ii/
// https://leetcode.cn/problems/maximize-the-number-of-target-nodes-after-connecting-trees-ii/
//
// Hard
//
// There exist two undirected trees with n and m nodes, labeled from [0, n - 1] and [0, m - 1], respectively.
//
// You are given two 2D integer arrays edges1 and edges2 of lengths n - 1 and m - 1,
// respectively, where edges1[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the first tree
// and edges2[i] = [ui, vi] indicates that there is an edge between nodes ui and vi in the second tree.
//
// Node u is target to node v if the number of edges on the path from u to v is even. Note that a node is always target to itself.
//
// Return an array of n integers answer, where answer[i] is the maximum possible number of nodes that are target
// to node i of the first tree if you had to connect one node from the first tree to another node in the second tree.
//
// Note that queries are independent from each other. That is, for every query you will remove the added edge before proceeding to the next query.
//
// Example 1:
//
// Input: edges1 = [[0,1],[0,2],[2,3],[2,4]], edges2 = [[0,1],[0,2],[0,3],[2,7],[1,4],[4,5],[4,6]]
//
// Output: [8,7,7,8,8]
//
// Explanation:
//
// For i = 0, connect node 0 from the first tree to node 0 from the second tree.
// For i = 1, connect node 1 from the first tree to node 4 from the second tree.
// For i = 2, connect node 2 from the first tree to node 7 from the second tree.
// For i = 3, connect node 3 from the first tree to node 0 from the second tree.
// For i = 4, connect node 4 from the first tree to node 4 from the second tree.
//
// Example 2:
//
// Input: edges1 = [[0,1],[0,2],[0,3],[0,4]], edges2 = [[0,1],[1,2],[2,3]]
//
// Output: [3,6,6,6,6]
//
// Explanation:
//
// For every i, connect node i of the first tree with any node of the second tree.
//
// Constraints:
//
// 2 <= n, m <= 10^5
// edges1.length == n - 1
// edges2.length == m - 1
// edges1[i].length == edges2[i].length == 2
// edges1[i] = [ai, bi]
// 0 <= ai, bi < n
// edges2[i] = [ui, vi]
// 0 <= ui, vi < m
// The input is generated such that edges1 and edges2 represent valid trees.
//

struct Solution;

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        fn dfs(i: i32, p: i32, al: &Vec<Vec<i32>>, parity: &mut Vec<bool>, even: bool) -> i32 {
            let mut targets = if even { 1 } else { 0 };
            parity[i as usize] = even;
            for &j in al[i as usize].iter() {
                if j != p {
                    targets += dfs(j, i, al, parity, !even);
                }
            }
            targets
        }

        fn adjacency_list(edges: &[Vec<i32>]) -> Vec<Vec<i32>> {
            let mut al = vec![vec![]; edges.len() + 1];
            for e in edges.iter() {
                al[e[0] as usize].push(e[1]);
                al[e[1] as usize].push(e[0]);
            }
            al
        }

        let m = edges1.len() as i32 + 1;
        let n = edges2.len() as i32 + 1;
        let mut parity = vec![false; m as usize];
        let mut ignored = vec![false; n as usize];
        let even1 = dfs(0, -1, &adjacency_list(&edges1), &mut parity, true);
        let odd1 = m - even1;
        let even2 = dfs(0, -1, &adjacency_list(&edges2), &mut ignored, true);
        let odd2 = n - even2;
        let mut res = vec![0; m as usize];
        for i in 0..m {
            res[i as usize] = std::cmp::max(even2, odd2) + if parity[i as usize] { even1 } else { odd1 };
        }
        res
    }
}

#[test]
fn test() {
    let edges1 = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]];
    let edges2 = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![2, 7], vec![1, 4], vec![4, 5], vec![4, 6]];
    let output = vec![8, 7, 7, 8, 8];
    assert_eq!(Solution::max_target_nodes(edges1, edges2), output);

    let edges1 = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]];
    let edges2 = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
    let output = vec![3, 6, 6, 6, 6];
    assert_eq!(Solution::max_target_nodes(edges1, edges2), output);
}
