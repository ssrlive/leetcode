#![allow(dead_code)]

// 3203. Find Minimum Diameter After Merging Two Trees
// https://leetcode.com/problems/find-minimum-diameter-after-merging-two-trees/
// https://leetcode.cn/problems/find-minimum-diameter-after-merging-two-trees/
//
// Hard
//
// There exist two undirected trees with n and m nodes, numbered from 0 to n - 1 and from 0 to m - 1, respectively.
// You are given two 2D integer arrays edges1 and edges2 of lengths n - 1 and m - 1, respectively,
// where edges1[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the first tree
// and edges2[i] = [ui, vi] indicates that there is an edge between nodes ui and vi in the second tree.
//
// You must connect one node from the first tree with another node from the second tree with an edge.
//
// Return the minimum possible diameter of the resulting tree.
//
// The diameter of a tree is the length of the longest path between any two nodes in the tree.
//
// Example 1:
//
// Input: edges1 = [[0,1],[0,2],[0,3]], edges2 = [[0,1]]
//
// Output: 3
//
// Explanation:
//
// We can obtain a tree of diameter 3 by connecting node 0 from the first tree with any node from the second tree.
//
// Example 2:
//
// Input: edges1 = [[0,1],[0,2],[0,3],[2,4],[2,5],[3,6],[2,7]], edges2 = [[0,1],[0,2],[0,3],[2,4],[2,5],[3,6],[2,7]]
//
// Output: 5
//
// Explanation:
//
// We can obtain a tree of diameter 5 by connecting node 0 from the first tree with node 0 from the second tree.
//
// Constraints:
//
//     1 <= n, m <= 10^5
//     edges1.length == n - 1
//     edges2.length == m - 1
//     edges1[i].length == edges2[i].length == 2
//     edges1[i] = [ai, bi]
//     0 <= ai, bi < n
//     edges2[i] = [ui, vi]
//     0 <= ui, vi < m
//     The input is generated such that edges1 and edges2 represent valid trees.
//

struct Solution;

impl Solution {
    fn build_tree(edges: &[Vec<i32>]) -> Vec<Vec<usize>> {
        let mut tree = vec![Vec::new(); edges.len() + 1];
        for edge in edges {
            let (v1, v2) = (edge[0] as usize, edge[1] as usize);
            tree[v1].push(v2);
            tree[v2].push(v1);
        }
        tree
    }

    fn find_dist(tree: &[Vec<usize>], v0: usize) -> (usize, i32) {
        let mut dist = vec![-1; tree.len()];
        dist[v0] = 0;
        let mut frontier = vec![v0];
        let mut d = 0;

        while !frontier.is_empty() {
            let mut new_frontier = Vec::new();
            d += 1;
            for v1 in frontier {
                for &v2 in &tree[v1] {
                    if dist[v2] == -1 {
                        dist[v2] = d;
                        new_frontier.push(v2);
                    }
                }
            }
            frontier = new_frontier;
        }

        for (i, &dist_i) in dist.iter().enumerate() {
            if dist_i == d - 1 {
                return (i, d - 1);
            }
        }

        (0, 0) // should not happen
    }

    fn tree_diam_center(tree: &[Vec<usize>]) -> i32 {
        let (v, _) = Self::find_dist(tree, 0);
        let (_, d) = Self::find_dist(tree, v);
        d
    }

    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        let d1 = Self::tree_diam_center(&Self::build_tree(&edges1));
        let d2 = Self::tree_diam_center(&Self::build_tree(&edges2));
        ((d1 + 1) / 2 + (d2 + 1) / 2 + 1).max(d1).max(d2)
    }
}

#[test]
fn test() {
    let edges1 = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
    let edges2 = vec![vec![0, 1]];
    assert_eq!(Solution::minimum_diameter_after_merge(edges1, edges2), 3);

    let edges1 = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![2, 7]];
    let edges2 = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![2, 7]];
    assert_eq!(Solution::minimum_diameter_after_merge(edges1, edges2), 5);
}
