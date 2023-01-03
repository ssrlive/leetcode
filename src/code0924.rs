#![allow(dead_code)]

// 924. Minimize Malware Spread
// https://leetcode.com/problems/minimize-malware-spread/
// https://leetcode.cn/problems/minimize-malware-spread/
//
// You are given a network of n nodes represented as an n x n adjacency matrix graph,
// where the ith node is directly connected to the jth node if graph[i][j] == 1.
//
// Some nodes initial are initially infected by malware. Whenever two nodes are directly connected,
// and at least one of those two nodes is infected by malware, both nodes will be infected by malware.
// This spread of malware will continue until no more nodes can be infected in this manner.
//
// Suppose M(initial) is the final number of nodes infected with malware in the entire network after the spread of malware stops.
// We will remove exactly one node from initial.
//
// Return the node that, if removed, would minimize M(initial). If multiple nodes could be removed to minimize M(initial),
// return such a node with the smallest index.
//
// Note that if a node was removed from the initial list of infected nodes, it might still be infected later due to the malware spread.
//
// Example 1:
//
// Input: graph = [[1,1,0],[1,1,0],[0,0,1]], initial = [0,1]
// Output: 0
//
// Example 2:
//
// Input: graph = [[1,0,0],[0,1,0],[0,0,1]], initial = [0,2]
// Output: 0
//
// Example 3:
//
// Input: graph = [[1,1,1],[1,1,1],[1,1,1]], initial = [1,2]
// Output: 1
//
// Constraints:
//
// - n == graph.length
// - n == graph[i].length
// - 2 <= n <= 300
// - graph[i][j] is 0 or 1.
// - graph[i][j] == graph[j][i]
// - graph[i][i] == 1
// - 1 <= initial.length <= n
// - 0 <= initial[i] <= n - 1
// - All the integers in initial are unique.
//

struct Solution;

impl Solution {
    pub fn min_malware_spread(graph: Vec<Vec<i32>>, initial: Vec<i32>) -> i32 {
        let mut uf = UnionFind::new(graph.len());

        for (r, item) in graph.iter().enumerate() {
            for (c, &obj) in item.iter().enumerate() {
                if obj == 1 {
                    uf.union(r, c);
                }
            }
        }

        let mut group_size = vec![0; graph.len()];
        for node in 0..graph.len() {
            group_size[uf.find(node)] += 1;
        }

        let mut min_initial = initial[0];
        let mut infected_per_group = vec![0; graph.len()];
        for &node in initial.iter() {
            infected_per_group[uf.find(node as usize)] += 1;
            min_initial = node.min(min_initial);
        }

        let mut selected = min_initial;
        let mut largest_group_size = 0;

        for &node in initial.iter() {
            let group_id = uf.find(node as usize);

            if infected_per_group[group_id] == 1 {
                match group_size[group_id].cmp(&largest_group_size) {
                    Ordering::Greater => {
                        largest_group_size = group_size[group_id];
                        selected = node;
                    }
                    Ordering::Equal => {
                        selected = selected.min(node);
                    }
                    Ordering::Less => {}
                }
            }
        }

        selected
    }
}

use std::cmp::Ordering;

struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parents: (0..size).collect(),
            ranks: vec![0; size],
        }
    }

    pub fn find(&mut self, key: usize) -> usize {
        if self.parents[key] == key {
            return key;
        }

        let parent = self.find(self.parents[key]);
        self.parents[key] = parent;
        parent
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);

        if px == py {
            return false;
        }

        let rx = self.ranks[px];
        let ry = self.ranks[py];

        match rx.cmp(&ry) {
            Ordering::Less => self.parents[px] = py,
            Ordering::Greater => self.parents[py] = px,
            Ordering::Equal => {
                self.parents[px] = py;
                self.ranks[py] += 1;
            }
        }

        true
    }
}

#[test]
fn test() {
    let graph = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
    let initial = vec![0, 1];
    assert_eq!(Solution::min_malware_spread(graph, initial), 0);

    let graph = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
    let initial = vec![0, 2];
    assert_eq!(Solution::min_malware_spread(graph, initial), 0);

    let graph = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
    let initial = vec![1, 2];
    assert_eq!(Solution::min_malware_spread(graph, initial), 1);
}
