#![allow(dead_code)]

/*

// 1483. Kth Ancestor of a Tree Node
// https://leetcode.com/problems/kth-ancestor-of-a-tree-node/
// https://leetcode.cn/problems/kth-ancestor-of-a-tree-node/
//
// Hard
//
// You are given a tree with n nodes numbered from 0 to n - 1 in the form of a parent array parent where parent[i] is the parent of ith node. The root of the tree is node 0. Find the kth ancestor of a given node.

The kth ancestor of a tree node is the kth node in the path from that node to the root node.

Implement the TreeAncestor class:

    TreeAncestor(int n, int[] parent) Initializes the object with the number of nodes in the tree and the parent array.
    int getKthAncestor(int node, int k) return the kth ancestor of the given node node. If there is no such ancestor, return -1.

Example 1:

Input
["TreeAncestor", "getKthAncestor", "getKthAncestor", "getKthAncestor"]
[[7, [-1, 0, 0, 1, 1, 2, 2]], [3, 1], [5, 2], [6, 3]]
Output
[null, 1, 0, -1]

Explanation
TreeAncestor treeAncestor = new TreeAncestor(7, [-1, 0, 0, 1, 1, 2, 2]);
treeAncestor.getKthAncestor(3, 1); // returns 1 which is the parent of 3
treeAncestor.getKthAncestor(5, 2); // returns 0 which is the grandparent of 5
treeAncestor.getKthAncestor(6, 3); // returns -1 because there is no such ancestor

Constraints:

    1 <= k <= n <= 5 * 10^4
    parent.length == n
    parent[0] == -1
    0 <= parent[i] < n for all 0 < i < n
    0 <= node < n
    There will be at most 5 * 10^4 queries.
*/

struct TreeAncestor {
    parent: Vec<Vec<i32>>,
}

impl TreeAncestor {
    fn new(n: i32, parent: Vec<i32>) -> Self {
        let mut parent2 = vec![vec![-1; 20]; n as usize];
        for i in 0..n as usize {
            parent2[i][0] = parent[i];
        }
        for j in 1..20 {
            for i in 0..n as usize {
                if parent2[i][j - 1] != -1 {
                    parent2[i][j] = parent2[parent2[i][j - 1] as usize][j - 1];
                }
            }
        }
        Self { parent: parent2 }
    }

    fn get_kth_ancestor(&self, node: i32, k: i32) -> i32 {
        let mut node = node;
        let mut k = k;
        for i in 0..20 {
            if k & 1 == 1 {
                node = self.parent[node as usize][i];
                if node == -1 {
                    return -1;
                }
            }
            k >>= 1;
        }
        node
    }
}

#[test]
fn test() {
    let tree_ancestor = TreeAncestor::new(7, vec![-1, 0, 0, 1, 1, 2, 2]);
    assert_eq!(tree_ancestor.get_kth_ancestor(3, 1), 1);
    assert_eq!(tree_ancestor.get_kth_ancestor(5, 2), 0);
    assert_eq!(tree_ancestor.get_kth_ancestor(6, 3), -1);
}
