#![allow(dead_code)]

// 1129. Shortest Path with Alternating Colors
// https://leetcode.com/problems/shortest-path-with-alternating-colors/
// https://leetcode.cn/problems/shortest-path-with-alternating-colors/
//
// You are given an integer n, the number of nodes in a directed graph where the nodes are labeled from 0 to n - 1.
// Each edge is red or blue in this graph, and there could be self-edges and parallel edges.
//
// You are given two arrays redEdges and blueEdges where:
//
// redEdges[i] = [ai, bi] indicates that there is a directed red edge from node ai to node bi in the graph, and
// blueEdges[j] = [uj, vj] indicates that there is a directed blue edge from node uj to node vj in the graph.
// Return an array answer of length n, where each answer[x] is the length of the shortest path
// from node 0 to node x such that the edge colors alternate along the path, or -1 if such a path does not exist.
//
// Example 1:
//
// Input: n = 3, redEdges = [[0,1],[1,2]], blueEdges = []
// Output: [0,1,-1]
//
// Example 2:
//
// Input: n = 3, redEdges = [[0,1]], blueEdges = [[2,1]]
// Output: [0,1,-1]
//
// Constraints:
//
// - 1 <= n <= 100
// - 0 <= redEdges.length, blueEdges.length <= 400
// - redEdges[i].length == blueEdges[j].length == 2
// - 0 <= ai, bi, uj, vj < n
//

struct Solution;

impl Solution {
    pub fn shortest_alternating_paths(n: i32, red_edges: Vec<Vec<i32>>, blue_edges: Vec<Vec<i32>>) -> Vec<i32> {
        const INF: i32 = 1_000_000_000;
        fn bfs(g: &[Vec<usize>], start: usize, goal: usize) -> i32 {
            let n = g.len();
            let mut memo = vec![INF; n];

            let mut stack = vec![(start, 0)];
            memo[start] = 0;
            while !stack.is_empty() {
                let mut new_stack = vec![];
                while let Some((ci, v)) = stack.pop() {
                    let nv = v + 1;
                    for i in 0..g[ci].len() {
                        let ni = g[ci][i];
                        if nv < memo[ni] {
                            memo[ni] = nv;
                            new_stack.push((ni, nv));
                        }
                    }
                }
                stack = new_stack;
            }
            std::cmp::min(memo[goal], memo[goal + n / 2])
        }

        let n = n as usize;
        let mut g = vec![vec![]; 2 * n];
        for arr in red_edges {
            let a = arr[0] as usize;
            let b = arr[1] as usize + n;
            g[a].push(b);
        }

        for arr in blue_edges {
            let a = arr[0] as usize + n;
            let b = arr[1] as usize;
            g[a].push(b);
        }

        let mut result = vec![-1; n];
        for (i, item) in result.iter_mut().enumerate() {
            let av = bfs(&g, 0, i);
            let bv = bfs(&g, n, i);

            let v = std::cmp::min(av, bv);
            if v != INF {
                *item = v;
            }
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (3, vec![vec![0, 1], vec![1, 2]], vec![], vec![0, 1, -1]),
        (3, vec![vec![0, 1]], vec![vec![2, 1]], vec![0, 1, -1]),
        (3, vec![vec![1, 0]], vec![vec![2, 1]], vec![0, -1, -1]),
        (3, vec![vec![0, 1]], vec![vec![1, 2]], vec![0, 1, 2]),
        (3, vec![vec![0, 1], vec![0, 2]], vec![vec![1, 0]], vec![0, 1, 1]),
        (
            5,
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]],
            vec![vec![1, 2], vec![2, 3], vec![3, 1]],
            vec![0, 1, 2, 3, 7],
        ),
    ];
    for (n, red_edges, blue_edges, expect) in cases {
        let result = Solution::shortest_alternating_paths(n, red_edges, blue_edges);
        assert_eq!(result, expect);
    }
}
