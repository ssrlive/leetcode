#![allow(dead_code)]

/*

// 1489. Find Critical and Pseudo-Critical Edges in Minimum Spanning Tree
// https://leetcode.com/problems/find-critical-and-pseudo-critical-edges-in-minimum-spanning-tree/
// https://leetcode.cn/problems/find-critical-and-pseudo-critical-edges-in-minimum-spanning-tree/
//
// Hard
//
// Given a weighted undirected connected graph with n vertices numbered from 0 to n - 1, and an array edges where edges[i] = [ai, bi, weighti] represents a bidirectional and weighted edge between nodes ai and bi. A minimum spanning tree (MST) is a subset of the graph's edges that connects all vertices without cycles and with the minimum possible total edge weight.

Find all the critical and pseudo-critical edges in the given graph's minimum spanning tree (MST). An MST edge whose deletion from the graph would cause the MST weight to increase is called a critical edge. On the other hand, a pseudo-critical edge is that which can appear in some MSTs but not all.

Note that you can return the indices of the edges in any order.

Example 1:

Input: n = 5, edges = [[0,1,1],[1,2,1],[2,3,2],[0,3,2],[0,4,3],[3,4,3],[1,4,6]]
Output: [[0,1],[2,3,4,5]]
Explanation: The figure above describes the graph.
The following figure shows all the possible MSTs:

Notice that the two edges 0 and 1 appear in all MSTs, therefore they are critical edges, so we return them in the first list of the output.
The edges 2, 3, 4, and 5 are only part of some MSTs, therefore they are considered pseudo-critical edges. We add them to the second list of the output.

Example 2:

Input: n = 4, edges = [[0,1,1],[1,2,1],[2,3,1],[0,3,1]]
Output: [[],[0,1,2,3]]
Explanation: We can observe that since all 4 edges have equal weight, choosing any 3 edges from the given 4 will yield an MST. Therefore all 4 edges are pseudo-critical.

Constraints:

    2 <= n <= 100
    1 <= edges.length <= min(200, n * (n - 1) / 2)
    edges[i].length == 3
    0 <= ai < bi < n
    1 <= weighti <= 1000
    All pairs (ai, bi) are distinct.
*/

struct Solution;

impl Solution {
    pub fn find_critical_and_pseudo_critical_edges(_n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut edges = edges
            .into_iter()
            .enumerate()
            .map(|(idx, edge)| (edge[2], edge[0], edge[1], idx as i32))
            .collect::<Vec<_>>();
        edges.sort_unstable_by_key(|edge| edge.0);

        let mut uf = UnionFind::new();
        let mut mst_edges = HashSet::new();

        // Calculate the sum of the weights of the MST
        let mut minimal_cost = 0;
        for (cost, from, to, edge_id) in edges.iter().copied() {
            if uf.union(from, to) {
                minimal_cost += cost;
                mst_edges.insert(edge_id);
            }
        }
        let minimal_cost = minimal_cost;

        // For each unprocessed edge, check if it is a pseudo-critical edge
        // by forcing it to be a part of the minimum spanning tree. If the
        // the total cost of the MST does not increase - then the edge is
        // pseudo-critical, otherwise it is not part of any MST
        let mut candidates = vec![];
        for (cost, from, to, required_edge) in edges.iter().copied() {
            // skip already known pseudo critical edges
            if mst_edges.contains(&required_edge) {
                continue;
            }

            candidates.clear();
            candidates.push(required_edge);

            uf.reset();
            uf.union(from, to);

            let mut total_cost = cost;
            for (cost, from, to, edge_id) in edges.iter().copied() {
                if required_edge != edge_id && uf.union(from, to) {
                    total_cost += cost;
                    candidates.push(edge_id);
                }
            }

            if total_cost == minimal_cost {
                mst_edges.extend(candidates.drain(..));
            }
        }

        let mut critical = vec![];
        let mut pseudo_critical = vec![];

        // Some of the pseudo-critical edges, a re actually critical edges.
        // Exclude the pseudo-critical edges from the MST one by one and check
        // if its total cost changes. If it does - then the excluded edge is
        // a critical edge
        for skipped_edge in mst_edges {
            uf.reset();

            let mut total_cost = 0;
            for (cost, from, to, edge_id) in edges.iter().copied() {
                if skipped_edge == edge_id {
                    continue;
                }

                if uf.union(from, to) {
                    total_cost += cost;
                }
            }

            if total_cost == minimal_cost {
                pseudo_critical.push(skipped_edge);
            } else {
                critical.push(skipped_edge);
            }
        }

        vec![critical, pseudo_critical]
    }
}

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

// Not the fastest implementation because of the hashmap, but can easily be resued in
// many different problems :)
struct UnionFind<T> {
    parents: HashMap<T, T>,
    ranks: HashMap<T, usize>,
}

impl<T> UnionFind<T>
where
    T: Eq + Hash + Copy,
{
    pub fn new() -> Self {
        Self {
            parents: HashMap::new(),
            ranks: HashMap::new(),
        }
    }

    fn reset(&mut self) {
        self.parents.clear();
        self.ranks.clear();
    }

    pub fn find(&mut self, key: T) -> T {
        match self.parents.get(&key) {
            None => key,
            Some(&parent) => {
                // Compress path on 'find()' by moving the key's node
                // directly under the representative of this set
                let parent = self.find(parent);
                self.parents.insert(key, parent);
                parent
            }
        }
    }

    pub fn union(&mut self, a: T, b: T) -> bool {
        let x = self.find(a);
        let y = self.find(b);

        // A and B are already in the same set -> nothing to do
        if x == y {
            return false;
        }

        let xr = self.ranks.get(&x).copied().unwrap_or(0);
        let yr = self.ranks.get(&y).copied().unwrap_or(0);

        match xr.cmp(&yr) {
            std::cmp::Ordering::Less => {
                self.parents.insert(x, y);
            }
            std::cmp::Ordering::Greater => {
                self.parents.insert(y, x);
            }
            std::cmp::Ordering::Equal => {
                self.parents.insert(x, y);
                *self.ranks.entry(y).or_insert(0) += 1;
            }
        }

        true
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            5,
            vec![
                vec![0, 1, 1],
                vec![1, 2, 1],
                vec![2, 3, 2],
                vec![0, 3, 2],
                vec![0, 4, 3],
                vec![3, 4, 3],
                vec![1, 4, 6],
            ],
            vec![vec![0, 1], vec![2, 3, 4, 5]],
        ),
        (
            4,
            vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 1], vec![0, 3, 1]],
            vec![vec![], vec![0, 1, 2, 3]],
        ),
    ];
    for (n, edges, expected) in cases {
        let mut v = Solution::find_critical_and_pseudo_critical_edges(n, edges);
        v.iter_mut().for_each(|x| x.sort_unstable());
        v.sort_unstable();
        assert_eq!(v, expected);
    }
}
