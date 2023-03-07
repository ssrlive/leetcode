#![allow(dead_code)]

/*

// 2368. Reachable Nodes With Restrictions
// https://leetcode.com/problems/reachable-nodes-in-subdivided-graph/
// https://leetcode.cn/problems/reachable-nodes-in-subdivided-graph/
//
// Medium
//
// There is an undirected tree with n nodes labeled from 0 to n - 1 and n - 1 edges.

You are given a 2D integer array edges of length n - 1 where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree. You are also given an integer array restricted which represents restricted nodes.

Return the maximum number of nodes you can reach from node 0 without visiting a restricted node.

Note that node 0 will not be a restricted node.

Example 1:

Input: n = 7, edges = [[0,1],[1,2],[3,1],[4,0],[0,5],[5,6]], restricted = [4,5]
Output: 4
Explanation: The diagram above shows the tree.
We have that [0,1,2,3] are the only nodes that can be reached from node 0 without visiting a restricted node.

Example 2:

Input: n = 7, edges = [[0,1],[0,2],[0,5],[0,4],[3,2],[6,5]], restricted = [4,2,1]
Output: 3
Explanation: The diagram above shows the tree.
We have that [0,5,6] are the only nodes that can be reached from node 0 without visiting a restricted node.

Constraints:

    2 <= n <= 10^5
    edges.length == n - 1
    edges[i].length == 2
    0 <= ai, bi < n
    ai != bi
    edges represents a valid tree.
    1 <= restricted.length < n
    1 <= restricted[i] < n
    All the values of restricted are unique.
*/

struct Solution;

impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n as usize];
        let mut s: HashSet<usize> = HashSet::new();
        for r in restricted {
            s.insert(r as usize);
        }
        for e in edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            if s.contains(&u) || s.contains(&v) {
                continue;
            }
            graph[u].push(v);
            graph[v].push(u);
        }
        if s.contains(&0) {
            return 0;
        }
        let mut sk: Vec<usize> = vec![];
        let mut ret = 0;
        let mut flag = vec![0; n as usize];
        sk.push(0);
        flag[0] = 1;
        while let Some(u) = sk.pop() {
            ret += 1;
            for v in &graph[u] {
                if flag[*v] == 1 {
                    continue;
                }
                flag[*v] = 1;
                sk.push(*v);
            }
        }
        ret
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            7,
            vec![vec![0, 1], vec![1, 2], vec![3, 1], vec![4, 0], vec![0, 5], vec![5, 6]],
            vec![4, 5],
            4,
        ),
        (
            7,
            vec![vec![0, 1], vec![0, 2], vec![0, 5], vec![0, 4], vec![3, 2], vec![6, 5]],
            vec![4, 2, 1],
            3,
        ),
    ];
    for (n, edges, restricted, expect) in cases {
        assert_eq!(Solution::reachable_nodes(n, edges, restricted), expect);
    }
}
