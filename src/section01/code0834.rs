#![allow(dead_code)]

// 834. Sum of Distances in Tree
// https://leetcode.com/problems/sum-of-distances-in-tree/
// https://leetcode.cn/problems/sum-of-distances-in-tree/
//
// There is an undirected connected tree with n nodes labeled from 0 to n - 1 and n - 1 edges.
//
// You are given the integer n and the array edges where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree.
//
// Return an array answer of length n where answer[i] is the sum of the distances between the ith node in the tree and all other nodes.
//
// Example 1:
//
// Input: n = 6, edges = [[0,1],[0,2],[2,3],[2,4],[2,5]]
// Output: [8,12,6,10,10,10]
// Explanation: The tree is shown above.
// We can see that dist(0,1) + dist(0,2) + dist(0,3) + dist(0,4) + dist(0,5)
// equals 1 + 1 + 2 + 2 + 2 = 8.
// Hence, answer[0] = 8, and so on.
//
// Example 2:
//
// Input: n = 1, edges = []
// Output: [0]
//
// Example 3:
//
// Input: n = 2, edges = [[1,0]]
// Output: [1,1]
//
// Constraints:
//
// - 1 <= n <= 3 * 10^4
// - edges.length == n - 1
// - edges[i].length == 2
// - 0 <= ai, bi < n
// - ai != bi
// - The given input represents a valid tree.
//

struct Solution;

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for edge in edges {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut ans = vec![0; n];
        let mut count = vec![1; n];
        let mut visited = vec![false; n];

        fn dfs1(graph: &Vec<Vec<usize>>, u: usize, visited: &mut Vec<bool>, count: &mut Vec<usize>, ans: &mut Vec<i32>) {
            visited[u] = true;
            for &v in &graph[u] {
                if !visited[v] {
                    dfs1(graph, v, visited, count, ans);
                    count[u] += count[v];
                    ans[u] += ans[v] + count[v] as i32;
                }
            }
        }

        fn dfs2(graph: &Vec<Vec<usize>>, u: usize, visited: &mut Vec<bool>, count: &mut Vec<usize>, ans: &mut Vec<i32>) {
            visited[u] = true;
            for &v in &graph[u] {
                if !visited[v] {
                    ans[v] = ans[u] - count[v] as i32 + (count.len() - count[v]) as i32;
                    dfs2(graph, v, visited, count, ans);
                }
            }
        }

        dfs1(&graph, 0, &mut visited, &mut count, &mut ans);
        visited = vec![false; n];
        dfs2(&graph, 0, &mut visited, &mut count, &mut ans);

        ans
    }
}

#[test]
fn test() {
    let n = 6;
    let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]];
    let ans = vec![8, 12, 6, 10, 10, 10];
    assert_eq!(Solution::sum_of_distances_in_tree(n, edges), ans);
}
