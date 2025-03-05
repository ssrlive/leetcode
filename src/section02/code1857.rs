#![allow(dead_code)]

/*

// 1857. Largest Color Value in a Directed Graph
// https://leetcode.com/problems/largest-color-value-in-a-directed-graph/
// https://leetcode.cn/problems/largest-color-value-in-a-directed-graph/
//
// Hard
//
// There is a directed graph of n colored nodes and m edges. The nodes are numbered from 0 to n - 1.

You are given a string colors where colors[i] is a lowercase English letter representing the color of the ith node in this graph (0-indexed). You are also given a 2D array edges where edges[j] = [aj, bj] indicates that there is a directed edge from node aj to node bj.

A valid path in the graph is a sequence of nodes x1 -> x2 -> x3 -> ... -> xk such that there is a directed edge from xi to xi+1 for every 1 <= i < k. The color value of the path is the number of nodes that are colored the most frequently occurring color along that path.

Return the largest color value of any valid path in the given graph, or -1 if the graph contains a cycle.

Example 1:

Input: colors = "abaca", edges = [[0,1],[0,2],[2,3],[3,4]]
Output: 3
Explanation: The path 0 -> 2 -> 3 -> 4 contains 3 nodes that are colored "a" (red in the above image).

Example 2:

Input: colors = "a", edges = [[0,0]]
Output: -1
Explanation: There is a cycle from 0 to 0.

Constraints:

    n == colors.length
    m == edges.length
    1 <= n <= 10^5
    0 <= m <= 10^5
    colors consists of lowercase English letters.
    0 <= aj, bj < n
*/

struct Solution;

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let n = colors.len();
        let colors = colors.chars().collect::<Vec<char>>();
        let mut graph = vec![vec![]; n];
        let mut indegrees = vec![0; n];
        for e in edges {
            indegrees[e[1] as usize] += 1;
            graph[e[0] as usize].push(e[1] as usize);
        }
        let (mut count, mut ret) = (n, 1);
        let mut data = vec![vec![0; 26]; n];
        let mut sk = vec![];
        for i in 0..n {
            if indegrees[i] > 0 {
                continue;
            }
            sk.push(i);
            data[i][colors[i] as usize - 'a' as usize] = 1;
        }
        while let Some(u) = sk.pop() {
            count -= 1;
            for v in &graph[u] {
                let idx = colors[*v] as usize - 'a' as usize;
                for k in 0..26 {
                    if k == idx {
                        data[*v][k] = data[*v][k].max(data[u][k] + 1);
                    } else {
                        data[*v][k] = data[*v][k].max(data[u][k]);
                    }
                }
                ret = ret.max(data[*v][idx]);
                indegrees[*v] -= 1;
                if indegrees[*v] > 0 {
                    continue;
                }
                sk.push(*v);
            }
        }
        if count > 0 { -1 } else { ret }
    }
}

#[test]
fn test() {
    let cases = vec![
        ("abaca", vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![3, 4]], 3),
        ("a", vec![vec![0, 0]], -1),
        ("aabbab", vec![vec![5, 2], vec![5, 1], vec![3, 4], vec![3, 2], vec![4, 1]], 2),
    ];
    for (colors, edges, expected) in cases {
        assert_eq!(Solution::largest_path_value(colors.to_string(), edges), expected);
    }
}
