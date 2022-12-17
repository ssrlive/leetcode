#![allow(dead_code)]

// 802. Find Eventual Safe States
// https://leetcode.com/problems/find-eventual-safe-states/
//
// There is a directed graph of n nodes with each node labeled from 0 to n - 1. The graph is represented by a 0-indexed 2D integer array graph
// where graph[i] is an integer array of nodes adjacent to node i, meaning there is an edge from node i to each node in graph[i].
//
// A node is a terminal node if there are no outgoing edges. A node is a safe node if every possible path starting from that node leads to a terminal node (or another safe node).
//
// Return an array containing all the safe nodes of the graph. The answer should be sorted in ascending order.
//
// Example 1:
//
// Illustration of graph
// Input: graph = [[1,2],[2,3],[5],[0],[5],[],[]]
// Output: [2,4,5,6]
// Explanation: The given graph is shown above.
// Nodes 5 and 6 are terminal nodes as there are no outgoing edges from either of them.
// Every path starting at nodes 2, 4, 5, and 6 all lead to either node 5 or 6.
//
// Example 2:
//
// Input: graph = [[1,2,3,4],[1,2],[3,4],[0,4],[]]
// Output: [4]
// Explanation:
// Only node 4 is a terminal node, and every path starting at node 4 leads to node 4.
//
// Constraints:
//
// - n == graph.length
// - 1 <= n <= 10^4
// - 0 <= graph[i].length <= n
// - 0 <= graph[i][j] <= n - 1
// - graph[i] is sorted in a strictly increasing order.
// - The graph may contain self-loops.
// - The number of edges in the graph will be in the range [1, 4 * 10^4].
//

struct Solution;

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        fn dfs(node: usize, graph: &Vec<Vec<i32>>, visited: &mut Vec<i32>) -> bool {
            if visited[node] == 1 {
                return false;
            }
            if visited[node] == 2 {
                return true;
            }
            visited[node] = 1;
            for &next in &graph[node] {
                if !dfs(next as usize, graph, visited) {
                    return false;
                }
            }
            visited[node] = 2;
            true
        }

        let mut result = Vec::new();
        let mut visited = vec![0; graph.len()];
        for i in 0..graph.len() {
            if dfs(i, &graph, &mut visited) {
                result.push(i as i32);
            }
        }
        result
    }
}

#[test]
fn test() {
    let graph = vec![vec![1, 2], vec![2, 3], vec![5], vec![0], vec![5], vec![], vec![]];
    assert_eq!(Solution::eventual_safe_nodes(graph), vec![2, 4, 5, 6]);

    let graph = vec![vec![1, 2, 3, 4], vec![1, 2], vec![3, 4], vec![0, 4], vec![]];
    assert_eq!(Solution::eventual_safe_nodes(graph), vec![4]);
}
