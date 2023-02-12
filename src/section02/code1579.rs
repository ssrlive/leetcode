#![allow(dead_code)]

/*

// 1579. Remove Max Number of Edges to Keep Graph Fully Traversable
Hard
918
9
Companies

Alice and Bob have an undirected graph of n nodes and three types of edges:

    Type 1: Can be traversed by Alice only.
    Type 2: Can be traversed by Bob only.
    Type 3: Can be traversed by both Alice and Bob.

Given an array edges where edges[i] = [typei, ui, vi] represents a bidirectional edge of type typei between nodes ui and vi, find the maximum number of edges you can remove so that after removing the edges, the graph can still be fully traversed by both Alice and Bob. The graph is fully traversed by Alice and Bob if starting from any node, they can reach all other nodes.

Return the maximum number of edges you can remove, or return -1 if Alice and Bob cannot fully traverse the graph.

Example 1:

Input: n = 4, edges = [[3,1,2],[3,2,3],[1,1,3],[1,2,4],[1,1,2],[2,3,4]]
Output: 2
Explanation: If we remove the 2 edges [1,1,2] and [1,1,3]. The graph will still be fully traversable by Alice and Bob. Removing any additional edge will not make it so. So the maximum number of edges we can remove is 2.

Example 2:

Input: n = 4, edges = [[3,1,2],[3,2,3],[1,1,4],[2,1,4]]
Output: 0
Explanation: Notice that removing any edge will not make the graph fully traversable by Alice and Bob.

Example 3:

Input: n = 4, edges = [[3,2,3],[1,1,2],[2,3,4]]
Output: -1
Explanation: In the current graph, Alice cannot reach node 4 from the other nodes. Likewise, Bob cannot reach 1. Therefore it's impossible to make the graph fully traversable.

Constraints:

    1 <= n <= 105
    1 <= edges.length <= min(105, 3 * n * (n - 1) / 2)
    edges[i].length == 3
    1 <= typei <= 3
    1 <= ui < vi <= n
    All tuples (typei, ui, vi) are distinct.
*/

struct Solution;

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut edges = edges;
        edges.sort_by_key(|e| e[0]);
        let mut uf = UnionFind::new(n as usize);
        let mut ans = 0;
        for e in edges.iter() {
            if e[0] == 3 && !uf.union(e[1] as usize - 1, e[2] as usize - 1) {
                ans += 1;
            }
        }
        let mut uf1 = uf.clone();
        let mut uf2 = uf;
        for e in edges.iter() {
            if (e[0] == 1 && !uf1.union(e[1] as usize - 1, e[2] as usize - 1))
                || (e[0] == 2 && !uf2.union(e[1] as usize - 1, e[2] as usize - 1))
            {
                ans += 1;
            }
        }
        if uf1.set_count != 1 || uf2.set_count != 1 {
            -1
        } else {
            ans
        }
    }
}

#[derive(Clone)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    set_count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        for (i, item) in parent.iter_mut().enumerate().take(n) {
            *item = i;
        }
        UnionFind {
            parent,
            rank: vec![0; n],
            set_count: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            return false;
        }
        if self.rank[x] < self.rank[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent[y] = x;
        if self.rank[x] == self.rank[y] {
            self.rank[x] += 1;
        }
        self.set_count -= 1;
        true
    }
}

#[test]
fn test() {
    let n = 4;
    let edges = vec![
        vec![3, 1, 2],
        vec![3, 2, 3],
        vec![1, 1, 3],
        vec![1, 2, 4],
        vec![1, 1, 2],
        vec![2, 3, 4],
    ];
    assert_eq!(Solution::max_num_edges_to_remove(n, edges), 2);
}
