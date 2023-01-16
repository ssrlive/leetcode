#![allow(dead_code)]

// 1192. Critical Connections in a Network
// https://leetcode.com/problems/critical-connections-in-a-network/
// https://leetcode.cn/problems/critical-connections-in-a-network/
//
// There are n servers numbered from 0 to n - 1 connected by undirected server-to-server connections forming a network where connections[i] = [ai, bi] represents a connection between servers ai and bi. Any server can reach other servers directly or indirectly through the network.
//
// A critical connection is a connection that, if removed, will make some servers unable to reach some other server.
//
// Return all critical connections in the network in any order.
//
// Example 1:
//
// Input: n = 4, connections = [[0,1],[1,2],[2,0],[1,3]]
// Output: [[1,3]]
// Explanation: [[3,1]] is also accepted.
//
// Example 2:
//
// Input: n = 2, connections = [[0,1]]
// Output: [[0,1]]
//
// Constraints:
//
// - 2 <= n <= 10^5
// - n - 1 <= connections.length <= 10^5
// - 0 <= ai, bi <= n - 1
// - ai != bi
// - There are no repeated connections.
//

struct Solution;

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;

        #[allow(clippy::too_many_arguments)]
        fn dfs(
            g: &HashMap<usize, Vec<usize>>,
            u: usize,
            t: &mut i32,
            visited: &mut Vec<bool>,
            disc: &mut Vec<i32>,
            low: &mut Vec<i32>,
            parents: &mut Vec<usize>,
            bridges: &mut Vec<Vec<i32>>,
        ) {
            visited[u] = true;
            disc[u] = *t;
            low[u] = *t;
            *t += 1;
            for &v in &g[&u] {
                if !visited[v] {
                    parents[v] = u;
                    dfs(g, v, t, visited, disc, low, parents, bridges);
                    low[u] = low[u].min(low[v]);
                    if low[v] > disc[u] {
                        bridges.push(vec![u as i32, v as i32]);
                    }
                } else if v != parents[u] {
                    low[u] = low[u].min(disc[v]);
                }
            }
        }

        let n = n as usize;
        let mut g = HashMap::<usize, Vec<_>>::new();
        for conn in connections {
            g.entry(conn[0] as usize).or_default().push(conn[1] as usize);
            g.entry(conn[1] as usize).or_default().push(conn[0] as usize);
        }
        let mut visited = vec![false; n];
        let mut disc = vec![-1; n];
        let mut low = vec![-1; n];
        let mut t = 0;
        let mut parents = vec![n + 1; n];
        let mut ans = Vec::new();
        for u in 0..n {
            if !visited[u] {
                dfs(&g, 0, &mut t, &mut visited, &mut disc, &mut low, &mut parents, &mut ans);
            }
        }
        ans
    }
}

#[test]
fn test() {
    let n = 4;
    let connections = vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]];
    let ans = vec![vec![1, 3]];
    assert_eq!(Solution::critical_connections(n, connections), ans);
}
