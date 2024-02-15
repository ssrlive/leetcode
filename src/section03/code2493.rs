#![allow(dead_code)]

// 2493. Divide Nodes Into the Maximum Number of Groups
// https://leetcode.com/problems/divide-nodes-into-the-maximum-number-of-groups/
// https://leetcode.cn/problems/divide-nodes-into-the-maximum-number-of-groups/
//
// You are given a positive integer n representing the number of nodes in an undirected graph. The nodes are labeled from 1 to n.
//
// You are also given a 2D integer array edges, where edges[i] = [ai, bi] indicates that there is a bidirectional
// edge between nodes ai and bi. Notice that the given graph may be disconnected.
//
// Divide the nodes of the graph into m groups (1-indexed) such that:
//
// Each node in the graph belongs to exactly one group.
// For every pair of nodes in the graph that are connected by an edge [ai, bi], if ai belongs to the group with index x, and bi belongs to the group with index y, then |y - x| = 1.
// Return the maximum number of groups (i.e., maximum m) into which you can divide the nodes. Return -1 if it is impossible to group the nodes with the given conditions.
//
// Example 1:
//
// Input: n = 6, edges = [[1,2],[1,4],[1,5],[2,6],[2,3],[4,6]]
// Output: 4
// Explanation: As shown in the image we:
// - Add node 5 to the first group.
// - Add node 1 to the second group.
// - Add nodes 2 and 4 to the third group.
// - Add nodes 3 and 6 to the fourth group.
// We can see that every edge is satisfied.
// It can be shown that that if we create a fifth group and move any node from the third or fourth group to it, at least on of the edges will not be satisfied.
//
// Example 2:
//
// Input: n = 3, edges = [[1,2],[2,3],[3,1]]
// Output: -1
// Explanation: If we add node 1 to the first group, node 2 to the second group, and node 3 to the third
//              group to satisfy the first two edges, we can see that the third edge will not be satisfied.
// It can be shown that no grouping is possible.
//
// Constraints:
//
// - 1 <= n <= 500
// - 1 <= edges.length <= 10^4
// - edges[i].length == 2
// - 1 <= a[i], b[i] <= n
// - a[i] != b[i]
// - There is at most one edge between any pair of vertices.
//

struct Solution;

impl Solution {
    pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        use std::collections::VecDeque;

        fn bfs(adj: &[Vec<usize>], s: usize) -> (i32, usize, i32) {
            let n = adj.len();
            let mut id = usize::MAX;
            let mut groups = 0;
            let mut level = vec![0; n + 1];
            level[s] = 1;
            let mut q = VecDeque::new();
            q.push_back((s, 1));

            while let Some((node, depth)) = q.pop_front() {
                groups = groups.max(depth);
                id = id.min(node);
                for &next in &adj[node] {
                    if level[next] == 0 {
                        level[next] = depth + 1;
                        q.push_back((next, depth + 1));
                    } else if level[next] == depth {
                        return (-1, 0, 0);
                    }
                }
            }
            (0, id, groups)
        }

        let n = n as usize;
        let mut adj = vec![vec![]; n + 1];
        for e in edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            adj[a].push(b);
            adj[b].push(a);
        }
        let mut map: HashMap<usize, i32> = HashMap::new();
        for i in 1..=n {
            let (f, id, groups) = bfs(&adj, i);
            if f == -1 {
                return -1;
            }
            if let Some(&v) = map.get(&id) {
                map.insert(id, v.max(groups));
            } else {
                map.insert(id, groups);
            }
        }
        let mut res = 0;
        for &v in map.values() {
            res += v;
        }
        res
    }
}
