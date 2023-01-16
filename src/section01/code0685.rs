#![allow(dead_code)]

// 685. Redundant Connection II
// https://leetcode.com/problems/redundant-connection-ii/description/
// https://leetcode.cn/problems/redundant-connection-ii/description/
//
// In this problem, a rooted tree is a directed graph such that, there is exactly one node (the root)
// for which all other nodes are descendants of this node, plus every node has exactly one parent,
// except for the root node which has no parents.
//
// The given input is a directed graph that started as a rooted tree with n nodes (with distinct values from 1 to n),
// with one additional directed edge added. The added edge has two different vertices chosen from 1 to n,
// and was not an edge that already existed.
//
// The resulting graph is given as a 2D-array of edges. Each element of edges is a pair [ui, vi] that represents
// a directed edge connecting nodes ui and vi, where ui is a parent of child vi.
//
// Return an edge that can be removed so that the resulting graph is a rooted tree of n nodes.
// If there are multiple answers, return the answer that occurs last in the given 2D-array.
//
// Example 1:
//
// Input: edges = [[1,2],[1,3],[2,3]]
// Output: [2,3]
//
// Example 2:
//
// Input: edges = [[1,2],[2,3],[3,4],[4,1],[1,5]]
// Output: [4,1]
//
// Constraints:
//
// - n == edges.length
// - 3 <= n <= 1000
// - edges[i].length == 2
// - 1 <= u.i, v.i <= n
// - u.i != v.i
//

struct Solution;

impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut indegrees = vec![0; n + 1];
        let mut node_with_indegree_of_2 = None;
        for edge in &edges {
            let target = edge[1] as usize;
            if indegrees[target] == 1 {
                node_with_indegree_of_2 = Some(target);
                break;
            }
            indegrees[target] += 1;
        }

        if let Some(node) = node_with_indegree_of_2 {
            // The added edge is directed to a non-root node. It must be one of the two edges
            // that are directed to that node.
            let mut candidates = vec![];
            let mut uf = UnionFind::new(n + 1);
            for edge in &edges {
                let (source, target) = (edge[0] as usize, edge[1] as usize);
                if target == node {
                    candidates.push(source);
                } else {
                    uf.union(source, target);
                }
            }
            // If the first candidate doesn't cause a cycle, we could happily remove the second candidate.
            // Otherwise we have to remove the first one.
            if uf.union(candidates[0], node) {
                vec![candidates[1] as i32, node as i32]
            } else {
                vec![candidates[0] as i32, node as i32]
            }
        } else {
            // The added edge is directed to the root. Remove any edge in the cycle would work.
            // We prefer the one occurs last.
            let mut uf = UnionFind::new(n + 1);
            for edge in edges {
                let (source, target) = (edge[0] as usize, edge[1] as usize);
                if !uf.union(source, target) {
                    return edge;
                }
            }
            unreachable!("");
        }
    }
}

struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parents: (0..size).collect(),
            sizes: vec![1; size],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        let mut root = x;
        let Self { parents, .. } = self;
        while parents[root] != root {
            // Path splitting
            let parent = parents[root];
            parents[root] = parents[parent];
            root = parent;
        }
        root
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let mut root1 = self.find(x);
        let mut root2 = self.find(y);
        if root1 == root2 {
            return false;
        }

        // Union by size
        let Self { parents, sizes } = self;
        if sizes[root1] > sizes[root2] {
            std::mem::swap(&mut root1, &mut root2);
        }
        parents[root1] = root2;
        sizes[root2] += sizes[root1];
        true
    }
}

#[test]
fn test() {
    let edges = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
    assert_eq!(Solution::find_redundant_directed_connection(edges), vec![2, 3]);
    let edges = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 1], vec![1, 5]];
    assert_eq!(Solution::find_redundant_directed_connection(edges), vec![4, 1]);
}
