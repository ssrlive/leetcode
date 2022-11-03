#![allow(dead_code)]

// 310. Minimum Height Trees
// https://leetcode.com/problems/minimum-height-trees/
//
// A tree is an undirected graph in which any two vertices are connected by exactly one path.
// In other words, any connected graph without simple cycles is a tree.
//
// Given a tree of n nodes labelled from 0 to n - 1, and an array of n - 1 edges
// where edges[i] = [ai, bi] indicates that there is an undirected edge between the two nodes ai and bi in the tree,
// you can choose any node of the tree as the root. When you select a node x as the root, the result tree has height h.
// Among all possible rooted trees, those with minimum height (i.e. min(h))  are called minimum height trees (MHTs).
//
// Return a list of all MHTs' root labels. You can return the answer in any order.
//
// The height of a rooted tree is the number of edges on the longest downward path between the root and a leaf.
//
// Example 1:
//
// Input: n = 4, edges = [[1,0],[1,2],[1,3]]
// Output: [1]
// Explanation: As shown, the height of the tree is 1 when the root is the node with label 1 which is the only MHT.
//
// Example 2:
//
// Input: n = 6, edges = [[3,0],[3,1],[3,2],[3,4],[5,4]]
// Output: [3,4]
//
// Constraints:
//
// - 1 <= n <= 2 * 104
// - edges.length == n - 1
// - 0 <= ai, bi < n
// - ai != bi
// - All the pairs (ai, bi) are distinct.
// - The given input is guaranteed to be a tree and there will be no repeated edges.

struct Solution;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut tree: Vec<Vec<i32>> = vec![vec![]; n as _];
        let mut node_edges_count = vec![0; n as _];
        let mut nodes_count = n;

        edges.iter().for_each(|i| {
            tree[i[0] as usize].push(i[1]);
            tree[i[1] as usize].push(i[0]);
            node_edges_count[i[0] as usize] += 1;
            node_edges_count[i[1] as usize] += 1;
        });

        let mut leaves: Vec<i32> = node_edges_count
            .iter_mut()
            .enumerate()
            .filter(|(_, j)| **j < 2)
            .map(|(i, j)| {
                *j = 0;
                i as i32
            })
            .collect();

        while nodes_count > 2 {
            let mut next_leaves: Vec<i32> = Vec::new();
            leaves.iter().for_each(|j| {
                tree[*j as usize].iter().for_each(|k| {
                    if node_edges_count[*k as usize] > 0 {
                        node_edges_count[*k as usize] -= 1;
                        if node_edges_count[*k as usize] == 1 {
                            next_leaves.push(*k);
                        }
                    }
                });
                nodes_count -= 1;
            });
            leaves = next_leaves;
        }
        leaves
    }
}

#[test]
fn test_minimum_height_trees() {
    assert_eq!(
        Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]]),
        vec![1]
    );
    assert_eq!(
        Solution::find_min_height_trees(
            6,
            vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]]
        ),
        vec![3, 4]
    );
}
