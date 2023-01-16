#![allow(dead_code)]

// 684. Redundant Connection
// https://leetcode.com/problems/redundant-connection/description/
// https://leetcode.cn/problems/redundant-connection/description/
//
// In this problem, a tree is an undirected graph that is connected and has no cycles.
//
// You are given a graph that started as a tree with n nodes labeled from 1 to n, with one additional edge added.
// The added edge has two different vertices chosen from 1 to n, and was not an edge that already existed.
// The graph is represented as an array edges of length n where edges[i] = [ai, bi] indicates that there is an edge
// between nodes ai and bi in the graph.
//
// Return an edge that can be removed so that the resulting graph is a tree of n nodes.
// If there are multiple answers, return the answer that occurs last in the input.
//
// Example 1:
//
// Input: edges = [[1,2],[1,3],[2,3]]
// Output: [2,3]
//
// Example 2:
//
// Input: edges = [[1,2],[2,3],[3,4],[1,4],[1,5]]
// Output: [1,4]
//
// Constraints:
//
// - n == edges.length
// - 3 <= n <= 1000
// - edges[i].length == 2
// - 1 <= a.i < b.i <= edges.length
// - a.i != b.i
// - There are no repeated edges.
// - The given graph is connected.
//

struct Solution;

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut uf = UnionFind::new(edges.len() + 1);
        for edge in edges {
            if !uf.union(edge[0] as usize, edge[1] as usize) {
                return edge;
            }
        }
        unreachable!()
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        let mut rank = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
            rank.push(0);
        }
        UnionFind { parent, rank }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root == y_root {
            return false;
        }
        match self.rank[x_root].cmp(&self.rank[y_root]) {
            std::cmp::Ordering::Less => self.parent[x_root] = y_root,
            std::cmp::Ordering::Greater => self.parent[y_root] = x_root,
            std::cmp::Ordering::Equal => {
                self.parent[y_root] = x_root;
                self.rank[x_root] += 1;
            }
        }
        true
    }
}

#[test]
fn test() {
    let edges = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
    assert_eq!(Solution::find_redundant_connection(edges), vec![2, 3]);
    let edges = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]];
    assert_eq!(Solution::find_redundant_connection(edges), vec![1, 4]);
}
