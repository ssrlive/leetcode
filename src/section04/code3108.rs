#![allow(dead_code)]

// 3108. Minimum Cost Walk in Weighted Graph
// https://leetcode.com/problems/minimum-cost-walk-in-weighted-graph/
// https://leetcode.cn/problems/minimum-cost-walk-in-weighted-graph/
//
// Hard
//
// There is an undirected weighted graph with n vertices labeled from 0 to n - 1.
//
// You are given the integer n and an array edges, where edges[i] = [ui, vi, wi] indicates
// that there is an edge between vertices ui and vi with a weight of wi.
//
// A walk on a graph is a sequence of vertices and edges. The walk starts and ends with a vertex,
// and each edge connects the vertex that comes before it and the vertex that comes after it.
// It's important to note that a walk may visit the same edge or vertex more than once.
//
// The cost of a walk starting at node u and ending at node v is defined as the bitwise AND of the weights of the edges traversed during the walk.
// In other words, if the sequence of edge weights encountered during the walk is w0, w1, w2, ..., wk,
// then the cost is calculated as w0 & w1 & w2 & ... & wk, where & denotes the bitwise AND operator.
//
// You are also given a 2D array query, where query[i] = [si, ti]. For each query, you need to find the minimum
// cost of the walk starting at vertex si and ending at vertex ti. If there exists no such walk, the answer is -1.
//
// Return the array answer, where answer[i] denotes the minimum cost of a walk for query i.
//
// Example 1:
//
// Input: n = 5, edges = [[0,1,7],[1,3,7],[1,2,1]], query = [[0,3],[3,4]]
//
// Output: [1,-1]
//
// Explanation:
//
// To achieve the cost of 1 in the first query, we need to move on the following edges: 0->1 (weight 7), 1->2 (weight 1), 2->1 (weight 1), 1->3 (weight 7).
//
// In the second query, there is no walk between nodes 3 and 4, so the answer is -1.
//
// Example 2:
//
// Input: n = 3, edges = [[0,2,7],[0,1,15],[1,2,6],[1,2,1]], query = [[1,2]]
//
// Output: [0]
//
// Explanation:
//
// To achieve the cost of 0 in the first query, we need to move on the following edges: 1->2 (weight 1), 2->1 (weight 6), 1->2 (weight 1).
//
// Constraints:
//
//     2 <= n <= 10^5
//     0 <= edges.length <= 10^5
//     edges[i].length == 3
//     0 <= ui, vi <= n - 1
//     ui != vi
//     0 <= wi <= 10^5
//     1 <= query.length <= 10^5
//     query[i].length == 2
//     0 <= si, ti <= n - 1
//     si != ti
//

struct Solution;

impl Solution {
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        let mut dsu = Dsu::new(n);
        for e in edges {
            dsu.union(e[0], e[1], e[2]);
        }
        query.into_iter().map(|q| dsu.dist(q[0], q[1])).collect()
    }
}

struct Dsu {
    parents: Vec<i32>,
    ranks: Vec<i32>,
    distances: Vec<i32>,
}

impl Dsu {
    fn new(n: i32) -> Self {
        Self {
            parents: (0..n).collect(),
            ranks: vec![0; n as usize],
            distances: vec![-1; n as usize],
        }
    }

    fn find(&mut self, a: i32) -> i32 {
        if self.parents[a as usize] != a {
            self.parents[a as usize] = self.find(self.parents[a as usize]);
        }
        self.parents[a as usize]
    }

    fn union(&mut self, a: i32, b: i32, weight: i32) {
        let a = self.find(a);
        let b = self.find(b);

        let dist = self.distances[a as usize] & self.distances[b as usize] & weight;
        self.distances[a as usize] = dist;
        self.distances[b as usize] = dist;

        if a == b {
            return;
        }

        match self.ranks[a as usize].cmp(&self.ranks[b as usize]) {
            std::cmp::Ordering::Less => {
                self.parents[a as usize] = b;
            }
            std::cmp::Ordering::Greater => {
                self.parents[b as usize] = a;
            }
            std::cmp::Ordering::Equal => {
                self.ranks[a as usize] += 1;
                self.parents[b as usize] = a;
            }
        }
    }

    fn dist(&mut self, a: i32, b: i32) -> i32 {
        let a = self.find(a);
        if a != self.find(b) {
            return -1;
        }
        self.distances[a as usize]
    }
}

#[test]
fn test() {
    let n = 5;
    let edges = vec![vec![0, 1, 7], vec![1, 3, 7], vec![1, 2, 1]];
    let query = vec![vec![0, 3], vec![3, 4]];
    assert_eq!(Solution::minimum_cost(n, edges, query), vec![1, -1]);

    let n = 3;
    let edges = vec![vec![0, 2, 7], vec![0, 1, 15], vec![1, 2, 6], vec![1, 2, 1]];
    let query = vec![vec![1, 2]];
    assert_eq!(Solution::minimum_cost(n, edges, query), vec![0]);
}
