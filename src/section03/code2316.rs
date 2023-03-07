#![allow(dead_code)]

/*

// 2316. Count Unreachable Pairs of Nodes in an Undirected Graph
// https://leetcode.com/problems/count-unreachable-pairs-of-nodes-in-an-undirected-graph/
// https://leetcode.cn/problems/count-unreachable-pairs-of-nodes-in-an-undirected-graph/
//
// Medium
//
// You are given an integer n. There is an undirected graph with n nodes, numbered from 0 to n - 1.
// You are given a 2D integer array edges where edges[i] = [ai, bi] denotes that there exists
// an undirected edge connecting nodes ai and bi.

Return the number of pairs of different nodes that are unreachable from each other.

Example 1:

Input: n = 3, edges = [[0,1],[0,2],[1,2]]
Output: 0
Explanation: There are no pairs of nodes that are unreachable from each other. Therefore, we return 0.

Example 2:

Input: n = 7, edges = [[0,2],[0,5],[2,4],[1,6],[5,4]]
Output: 14
Explanation: There are 14 pairs of nodes that are unreachable from each other:
[[0,1],[0,3],[0,6],[1,2],[1,3],[1,4],[1,5],[2,3],[2,6],[3,4],[3,5],[3,6],[4,6],[5,6]].
Therefore, we return 14.

Constraints:

    1 <= n <= 10^5
    0 <= edges.length <= 2 * 10^5
    edges[i].length == 2
    0 <= ai, bi < n
    ai != bi
    There are no repeated edges.
*/

struct Solution;

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        use std::collections::HashMap;
        let n = n as usize;
        let graph = edges
            .iter()
            .flat_map(|edge| [[edge[0], edge[1]], [edge[1], edge[0]]])
            .fold(HashMap::with_capacity(n), |mut g, edge| {
                g.entry(edge[0] as usize).or_insert(vec![]).push(edge[1] as usize);
                g
            });
        let mut visited: Vec<bool> = vec![false; n];

        fn dfs(curr: usize, parent: usize, graph: &HashMap<usize, Vec<usize>>, visited: &mut Vec<bool>) -> i64 {
            if visited[curr] {
                return 0;
            }
            visited[curr] = true;
            graph.get(&curr).map_or(1, |children| {
                1i64 + children
                    .iter()
                    .filter(|&&neigh| neigh != parent)
                    .map(|&child| dfs(child, curr, graph, visited))
                    .sum::<i64>()
            })
        }

        let mut cnt = 0;
        (0..visited.len()).for_each(|i| {
            if !visited[i] {
                let cc_cnt = dfs(i, 0, &graph, &mut visited);
                cnt += cc_cnt * (n as i64 - cc_cnt);
            }
        });
        cnt / 2
    }
}

#[test]
fn test() {
    let cases = vec![
        (3, vec![vec![0, 1], vec![0, 2], vec![1, 2]], 0),
        (7, vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]], 14),
    ];
    for (n, edges, expected) in cases {
        assert_eq!(Solution::count_pairs(n, edges), expected);
    }
}
