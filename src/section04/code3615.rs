#![allow(dead_code)]

// 3615. Longest Palindromic Path in Graph
// https://leetcode.com/problems/longest-palindromic-path-in-graph/
// https://leetcode.cn/problems/longest-palindromic-path-in-graph/
//
// Hard
//
// You are given an integer n and an undirected graph with n nodes labeled from 0 to n - 1 and a 2D array edges, where edges[i] = [ui, vi] indicates an edge between nodes ui and vi.
//
// You are also given a string label of length n, where label[i] is the character associated with node i.
//
// You may start at any node and move to any adjacent node, visiting each node at most once.
//
// Return the maximum possible length of a
//
// that can be formed by visiting a set of unique nodes along a valid path.
//
// Example 1:
//
// Input: n = 3, edges = [[0,1],[1,2]], label = "aba"
//
// Output: 3
//
// Explanation:
//
//     The longest palindromic path is from node 0 to node 2 via node 1, following the path 0 → 1 → 2 forming string "aba".
//     This is a valid palindrome of length 3.
//
// Example 2:
//
// Input: n = 3, edges = [[0,1],[0,2]], label = "abc"
//
// Output: 1
//
// Explanation:
//
//     No path with more than one node forms a palindrome.
//     The best option is any single node, giving a palindrome of length 1.
//
// Example 3:
//
// Input: n = 4, edges = [[0,2],[0,3],[3,1]], label = "bbac"
//
// Output: 3
//
// Explanation:
//
//     The longest palindromic path is from node 0 to node 1, following the path 0 → 3 → 1, forming string "bcb".
//     This is a valid palindrome of length 3.
//
// Constraints:
//
//     1 <= n <= 14
//     n - 1 <= edges.length <= n * (n - 1) / 2
//     edges[i] == [ui, vi]
//     0 <= ui, vi <= n - 1
//     ui != vi
//     label.length == n
//     label consists of lowercase English letters.
//     There are no duplicate edges.
//

struct Solution;

impl Solution {
    pub fn max_len(n: i32, edges: Vec<Vec<i32>>, label: String) -> i32 {
        let n = n as usize;
        let label = label.as_bytes();
        let mut adj = vec![vec![]; n];
        for e in &edges {
            adj[e[0] as usize].push(e[1] as usize);
            adj[e[1] as usize].push(e[0] as usize);
        }
        let mut dp = vec![vec![vec![-1; n]; n]; 1 << n];
        fn dfs(mask: usize, u: usize, v: usize, dp: &mut Vec<Vec<Vec<i32>>>, adj: &Vec<Vec<usize>>, label: &[u8]) -> i32 {
            if dp[mask][u][v] != -1 {
                return dp[mask][u][v];
            }
            let mut res = 0;
            for &u2 in &adj[u] {
                if mask & (1 << u2) != 0 {
                    continue;
                }
                for &v2 in &adj[v] {
                    if u2 == v2 || mask & (1 << v2) != 0 || label[u2] != label[v2] {
                        continue;
                    }
                    let nm = mask | (1 << u2) | (1 << v2);
                    res = res.max(1 + dfs(nm, u2, v2, dp, adj, label));
                }
            }
            dp[mask][u][v] = res;
            res
        }
        let mut best = 1;
        for u in 0..n {
            let mask = 1 << u;
            let pairs = dfs(mask, u, u, &mut dp, &adj, label);
            best = best.max(1 + 2 * pairs);
        }
        for e in &edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            if label[u] != label[v] {
                continue;
            }
            let mask = (1 << u) | (1 << v);
            let pairs = dfs(mask, u, v, &mut dp, &adj, label);
            best = best.max(2 * (1 + pairs));
        }
        best
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_len(3, vec![vec![0, 1], vec![1, 2]], "aba".into()), 3);
    assert_eq!(Solution::max_len(3, vec![vec![0, 1], vec![0, 2]], "abc".into()), 1);
    assert_eq!(Solution::max_len(4, vec![vec![0, 2], vec![0, 3], vec![3, 1]], "bbac".into()), 3);
    assert_eq!(Solution::max_len(3, vec![vec![1, 0], vec![1, 2]], "loo".into()), 2);
}

#[test]
fn test_performance() {
    // Test with larger cases to verify performance improvements
    use std::time::Instant;

    // Maximum size case (n = 14)
    let edges = vec![
        vec![0, 1],
        vec![1, 2],
        vec![2, 3],
        vec![3, 4],
        vec![4, 5],
        vec![5, 6],
        vec![6, 7],
        vec![7, 8],
        vec![8, 9],
        vec![9, 10],
        vec![10, 11],
        vec![11, 12],
        vec![12, 13],
        vec![0, 7],
        vec![1, 8],
        vec![2, 9],
        vec![3, 10],
        vec![4, 11],
        vec![5, 12],
        vec![6, 13],
    ];
    let label = "abcdefghijklmn".to_string();

    let start = Instant::now();
    let result = Solution::max_len(14, edges, label);
    let duration = start.elapsed();

    println!("Performance test completed in {duration:?}, result: {result}");
    assert!(duration.as_millis() < 1000); // Should complete within 1 second
}
