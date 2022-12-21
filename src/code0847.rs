#![allow(dead_code)]

// 847. Shortest Path Visiting All Nodes
// https://leetcode.com/problems/shortest-path-visiting-all-nodes/
// https://leetcode.cn/problems/shortest-path-visiting-all-nodes/
//
// You have an undirected, connected graph of n nodes labeled from 0 to n - 1.
// You are given an array graph where graph[i] is a list of all the nodes connected with node i by an edge.
//
// Return the length of the shortest path that visits every node. You may start and stop at any node,
// you may revisit nodes multiple times, and you may reuse edges.
//
// Example 1:
//
// Input: graph = [[1,2,3],[0],[0],[0]]
// Output: 4
// Explanation: One possible path is [1,0,2,0,3]
//
// Example 2:
//
// Input: graph = [[1],[0,2,4],[1,3,4],[2],[1,2]]
// Output: 4
// Explanation: One possible path is [0,1,4,2,3]
//
// Constraints:
//
// - n == graph.length
// - 1 <= n <= 12
// - 0 <= graph[i].length < n
// - graph[i] does not contain i.
// - If graph[a] contains b, then graph[b] contains a.
// - The input graph is always connected.
//

struct Solution;

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let mut queue = std::collections::VecDeque::new();
        let mut visited = std::collections::HashSet::new();
        for i in 0..n {
            queue.push_back((i, 1 << i));
            visited.insert((i, 1 << i));
        }
        let mut step = 0;
        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                let (cur, mask) = queue.pop_front().unwrap();
                if mask == (1 << n) - 1 {
                    return step;
                }
                for &next in &graph[cur] {
                    let next_mask = mask | (1 << next);
                    if !visited.contains(&(next as usize, next_mask)) {
                        queue.push_back((next as usize, next_mask));
                        visited.insert((next as usize, next_mask));
                    }
                }
            }
            step += 1;
        }
        0
    }
}

#[test]
fn test() {
    let graph = vec![vec![1, 2, 3], vec![0], vec![0], vec![0]];
    assert_eq!(Solution::shortest_path_length(graph), 4);

    let graph = vec![vec![1], vec![0, 2, 4], vec![1, 3, 4], vec![2], vec![1, 2]];
    assert_eq!(Solution::shortest_path_length(graph), 4);
}
