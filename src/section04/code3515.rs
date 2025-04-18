#![allow(dead_code)]

// 3515. Shortest Path in a Weighted Tree
// https://leetcode.com/problems/shortest-path-in-a-weighted-tree/
// https://leetcode.cn/problems/shortest-path-in-a-weighted-tree/
//
// Hard
//
// You are given an integer n and an undirected, weighted tree rooted at node 1 with n nodes numbered from 1 to n.
// This is represented by a 2D array edges of length n - 1, where edges[i] = [ui, vi, wi] indicates an undirected edge from node ui to vi with weight wi.
//
// You are also given a 2D integer array queries of length q, where each queries[i] is either:
//
//     [1, u, v, w'] – Update the weight of the edge between nodes u and v to w', where (u, v) is guaranteed to be an edge present in edges.
//     [2, x] – Compute the shortest path distance from the root node 1 to node x.
//
// Return an integer array answer, where answer[i] is the shortest path distance from node 1 to x for the ith query of [2, x].
//
// Example 1:
//
// Input: n = 2, edges = [[1,2,7]], queries = [[2,2],[1,1,2,4],[2,2]]
//
// Output: [7,4]
//
// Explanation:
//
//     Query [2,2]: The shortest path from root node 1 to node 2 is 7.
//     Query [1,1,2,4]: The weight of edge (1,2) changes from 7 to 4.
//     Query [2,2]: The shortest path from root node 1 to node 2 is 4.
//
// Example 2:
//
// Input: n = 3, edges = [[1,2,2],[1,3,4]], queries = [[2,1],[2,3],[1,1,3,7],[2,2],[2,3]]
//
// Output: [0,4,2,7]
//
// Explanation:
//
//     Query [2,1]: The shortest path from root node 1 to node 1 is 0.
//     Query [2,3]: The shortest path from root node 1 to node 3 is 4.
//     Query [1,1,3,7]: The weight of edge (1,3) changes from 4 to 7.
//     Query [2,2]: The shortest path from root node 1 to node 2 is 2.
//     Query [2,3]: The shortest path from root node 1 to node 3 is 7.
//
// Example 3:
//
// Input: n = 4, edges = [[1,2,2],[2,3,1],[3,4,5]], queries = [[2,4],[2,3],[1,2,3,3],[2,2],[2,3]]
//
// Output: [8,3,2,5]
//
// Explanation:
//
//     Query [2,4]: The shortest path from root node 1 to node 4 consists of edges (1,2), (2,3), and (3,4) with weights 2 + 1 + 5 = 8.
//     Query [2,3]: The shortest path from root node 1 to node 3 consists of edges (1,2) and (2,3) with weights 2 + 1 = 3.
//     Query [1,2,3,3]: The weight of edge (2,3) changes from 1 to 3.
//     Query [2,2]: The shortest path from root node 1 to node 2 is 2.
//     Query [2,3]: The shortest path from root node 1 to node 3 consists of edges (1,2) and (2,3) with updated weights 2 + 3 = 5.
//
// Constraints:
//
//     1 <= n <= 105
//     edges.length == n - 1
//     edges[i] == [ui, vi, wi]
//     1 <= ui, vi <= n
//     1 <= wi <= 104
//     The input is generated such that edges represents a valid tree.
//     1 <= queries.length == q <= 105
//     queries[i].length == 2 or 4
//         queries[i] == [1, u, v, w'] or,
//         queries[i] == [2, x]
//         1 <= u, v, x <= n
//         (u, v) is always an edge from edges.
//         1 <= w' <= 104
//

struct Solution;

impl Solution {
    pub fn tree_queries(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        #[allow(clippy::too_many_arguments)]
        fn dfs(
            v: usize,
            p: i32,
            w: i32,
            euler: &mut Vec<usize>,
            first_oc: &mut Vec<usize>,
            last_oc: &mut Vec<usize>,
            st: &mut SegTree,
            adj: &[Vec<(usize, i32)>],
        ) {
            euler.push(v);
            first_oc[v] = euler.len() - 1;
            last_oc[v] = euler.len() - 1;
            st.update(euler.len() - 1, w);

            for nei in &adj[v] {
                if nei.0 != p as usize {
                    dfs(nei.0, v as i32, nei.1, euler, first_oc, last_oc, st, adj);
                    euler.push(v);
                    st.update(euler.len() - 1, -nei.1);
                    last_oc[v] = euler.len() - 1;
                }
            }
        }

        let n = n as usize;
        let mut euler = Vec::new();
        let mut first_oc = vec![0; n + 1];
        let mut last_oc = vec![0; n + 1];
        let mut adj = vec![Vec::new(); n + 1];
        let mut st = SegTree::new(2 * n + 2 * edges.len());

        for e in &edges {
            adj[e[0] as usize].push((e[1] as usize, e[2]));
            adj[e[1] as usize].push((e[0] as usize, e[2]));
        }

        dfs(1, -1, 0, &mut euler, &mut first_oc, &mut last_oc, &mut st, &adj);

        let mut ans = Vec::new();

        for q in queries {
            let type_ = q[0];
            if type_ == 1 {
                let mut u = q[1] as usize;
                let mut v = q[2] as usize;
                let ww = q[3];
                if first_oc[u] > first_oc[v] {
                    std::mem::swap(&mut u, &mut v);
                }
                st.update(first_oc[v], ww);
                st.update(last_oc[v] + 1, -ww);
            } else {
                let x = q[1] as usize;
                ans.push(st.query(0, first_oc[x] + 1));
            }
        }
        ans
    }
}

struct SegTree {
    n: usize,
    t: Vec<i32>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        Self { n, t: vec![0; n << 1] }
    }

    fn f(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    fn update(&mut self, pos: usize, val: i32) {
        let mut pos = pos + self.n;
        self.t[pos] = val;
        while pos > 1 {
            pos /= 2;
            self.t[pos] = self.f(self.t[pos * 2], self.t[pos * 2 + 1]);
        }
    }

    fn query(&self, l: usize, r: usize) -> i32 {
        let mut ra = 0;
        let mut rb = 0;
        let mut l = l + self.n;
        let mut r = r + self.n;
        while l < r {
            if l % 2 == 1 {
                ra = self.f(ra, self.t[l]);
                l += 1;
            }
            if r % 2 == 1 {
                rb = self.f(rb, self.t[r - 1]);
                r -= 1;
            }
            l /= 2;
            r /= 2;
        }
        self.f(ra, rb)
    }
}

#[test]
fn test() {
    let n = 2;
    let edges = vec![vec![1, 2, 7]];
    let queries = vec![vec![2, 2], vec![1, 1, 2, 4], vec![2, 2]];
    let result = Solution::tree_queries(n, edges, queries);
    assert_eq!(result, vec![7, 4]);

    let n = 3;
    let edges = vec![vec![1, 2, 2], vec![1, 3, 4]];
    let queries = vec![vec![2, 1], vec![2, 3], vec![1, 1, 3, 7], vec![2, 2], vec![2, 3]];
    let result = Solution::tree_queries(n, edges, queries);
    assert_eq!(result, vec![0, 4, 2, 7]);
}
