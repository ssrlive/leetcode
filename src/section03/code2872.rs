#![allow(dead_code)]

// 2872. Maximum Number of K-Divisible Components
// https://leetcode.com/problems/maximum-number-of-k-divisible-components/
// https://leetcode.cn/problems/maximum-number-of-k-divisible-components/
//
// Hard
//
// There is an undirected tree with n nodes labeled from 0 to n - 1. You are given the integer n and a 2D integer array
// edges of length n - 1, where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree.
//
// You are also given a 0-indexed integer array values of length n, where values[i] is the value associated with the ith node, and an integer k.
//
// A valid split of the tree is obtained by removing any set of edges, possibly empty,
// from the tree such that the resulting components all have values that are divisible by k,
// where the value of a connected component is the sum of the values of its nodes.
//
// Return the maximum number of components in any valid split.
//
// Example 1:
//                  1               1
//                /   \    =>         \
//               2     3          2    3
//              / \              / \
//             4   0            4   0
//
// Input: n = 5, edges = [[0,2],[1,2],[1,3],[2,4]], values = [1,8,1,4,4], k = 6
// Output: 2
// Explanation: We remove the edge connecting node 1 with 2. The resulting split is valid because:
// - The value of the component containing nodes 1 and 3 is values[1] + values[3] = 12.
// - The value of the component containing nodes 0, 2, and 4 is values[0] + values[2] + values[4] = 6.
// It can be shown that no other valid split has more than 2 connected components.
//
// Example 2:
//                  0                   0
//                /   \     =>
//               2     1              2    1
//              / \   / \            / \  / \
//             6   5 4   3          6   5 4   3
//
// Input: n = 7, edges = [[0,1],[0,2],[1,3],[1,4],[2,5],[2,6]], values = [3,0,6,1,5,2,1], k = 3
// Output: 3
// Explanation: We remove the edge connecting node 0 with 2, and the edge connecting node 0 with 1. The resulting split is valid because:
// - The value of the component containing node 0 is values[0] = 3.
// - The value of the component containing nodes 2, 5, and 6 is values[2] + values[5] + values[6] = 9.
// - The value of the component containing nodes 1, 3, and 4 is values[1] + values[3] + values[4] = 6.
// It can be shown that no other valid split has more than 3 connected components.
//
// Constraints:
//
// 1 <= n <= 3 * 10^4
// edges.length == n - 1
// edges[i].length == 2
// 0 <= ai, bi < n
// values.length == n
// 0 <= values[i] <= 10^9
// 1 <= k <= 10^9
// Sum of values is divisible by k.
// The input is generated such that edges represents a valid tree.
//

struct Solution;

struct Helper {
    graph: Vec<Vec<usize>>,
    values: Vec<i32>,
    k: i64,
}

impl Helper {
    fn new(edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> Self {
        let n = edges.len() + 1;
        let mut graph = vec![vec![]; n];

        for edge in edges {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            graph[u].push(v);
            graph[v].push(u);
        }
        Self {
            graph,
            values: values.clone(),
            k: k as i64,
        }
    }

    fn dfs(&self, u: usize, p: usize, sum: &mut Vec<i64>, cnt: &mut i32) {
        sum[u] = self.values[u] as i64;
        for v in &self.graph[u] {
            let v = *v;
            if v == p {
                continue;
            }
            self.dfs(v, u, sum, cnt);
            sum[u] += sum[v];
        }
        if sum[u] % self.k == 0 {
            *cnt += 1;
        }
    }
}

impl Solution {
    pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
        let h = Helper::new(edges, values, k);
        let mut cnt = 0;
        let mut sum = vec![0i64; n as usize];
        h.dfs(0, n as usize, &mut sum, &mut cnt);
        cnt
    }
}

#[test]
fn test() {
    let n = 5;
    let edges = vec![vec![0, 2], vec![1, 2], vec![1, 3], vec![2, 4]];
    let values = vec![1, 8, 1, 4, 4];
    let k = 6;
    let res = 2;
    assert_eq!(Solution::max_k_divisible_components(n, edges, values, k), res);

    let n = 7;
    let edges = vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6]];
    let values = vec![3, 0, 6, 1, 5, 2, 1];
    let k = 3;
    let res = 3;
    assert_eq!(Solution::max_k_divisible_components(n, edges, values, k), res);
}
