#![allow(dead_code)]

// 3607. Power Grid Maintenance
// https://leetcode.com/problems/power-grid-maintenance/
// https://leetcode.cn/problems/power-grid-maintenance/
//
// Medium
//
// You are given an integer c representing c power stations, each with a unique identifier id from 1 to c (1‑based indexing).
//
// These stations are interconnected via n bidirectional cables, represented by a 2D array connections,
// where each element connections[i] = [ui, vi] indicates a connection between station ui and station vi.
// Stations that are directly or indirectly connected form a power grid.
//
// Initially, all stations are online (operational).
//
// You are also given a 2D array queries, where each query is one of the following two types:
//
// - [1, x]: A maintenance check is requested for station x. If station x is online, it resolves the check by itself.
//   If station x is offline, the check is resolved by the operational station with the smallest id in the same power grid as x.
//   If no operational station exists in that grid, return -1.
// - [2, x]: Station x goes offline (i.e., it becomes non-operational).
//
// Return an array of integers representing the results of each query of type [1, x] in the order they appear.
//
// Note: The power grid preserves its structure; an offline (non‑operational) node remains part of
//       its grid and taking it offline does not alter connectivity.
//
// Example 1:
//
// Input: c = 5, connections = [[1,2],[2,3],[3,4],[4,5]], queries = [[1,3],[2,1],[1,1],[2,2],[1,2]]
//
// Output: [3,2,3]
//
// Explanation:
//
// - Initially, all stations {1, 2, 3, 4, 5} are online and form a single power grid.
// - Query [1,3]: Station 3 is online, so the maintenance check is resolved by station 3.
// - Query [2,1]: Station 1 goes offline. The remaining online stations are {2, 3, 4, 5}.
// - Query [1,1]: Station 1 is offline, so the check is resolved by the operational station
//   with the smallest id among {2, 3, 4, 5}, which is station 2.
// - Query [2,2]: Station 2 goes offline. The remaining online stations are {3, 4, 5}.
// - Query [1,2]: Station 2 is offline, so the check is resolved by the operational station
//   with the smallest id among {3, 4, 5}, which is station 3.
//
// Example 2:
//
// Input: c = 3, connections = [], queries = [[1,1],[2,1],[1,1]]
//
// Output: [1,-1]
//
// Explanation:
//
//     There are no connections, so each station is its own isolated grid.
//     Query [1,1]: Station 1 is online in its isolated grid, so the maintenance check is resolved by station 1.
//     Query [2,1]: Station 1 goes offline.
//     Query [1,1]: Station 1 is offline and there are no other stations in its grid, so the result is -1.
//
// Constraints:
//
//     1 <= c <= 10^5
//     0 <= n == connections.length <= min(10^5, c * (c - 1) / 2)
//     connections[i].length == 2
//     1 <= ui, vi <= c
//     ui != vi
//     1 <= queries.length <= 2 * 10^5
//     queries[i].length == 2
//     queries[i][0] is either 1 or 2.
//     1 <= queries[i][1] <= c
//

struct Solution;

impl Solution {
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        fn dfs(node: i32, vis: &mut [bool], graph: &[Vec<i32>], s: &mut std::collections::BTreeSet<i32>) {
            vis[node as usize] = true;
            for &child in &graph[node as usize] {
                if vis[child as usize] {
                    continue;
                }
                s.insert(child);
                dfs(child, vis, graph, s);
            }
        }

        let mut graph = vec![vec![]; (c + 1) as usize];
        for connection in &connections {
            graph[(connection[0] - 1) as usize].push(connection[1] - 1);
            graph[(connection[1] - 1) as usize].push(connection[0] - 1);
        }
        let mut pwer = vec![true; (c + 1) as usize];
        let mut ans = Vec::new();
        let mut vis = vec![false; (c + 1) as usize];
        let mut v = Vec::new();
        let mut mp = std::collections::HashMap::new();
        let mut mpp = std::collections::HashMap::new();
        for i in 0..c {
            if vis[i as usize] {
                continue;
            }
            let mut s = std::collections::BTreeSet::new();
            s.insert(i);
            dfs(i, &mut vis, &graph, &mut s);
            v.push(s.clone());
            let mini = *s.iter().next().unwrap();
            mp.insert(mini, v.len() - 1);
            for i in s {
                mpp.insert(i, mini);
            }
        }
        for query in &queries {
            let op = query[0];
            let x = query[1] - 1;
            if op == 2 {
                pwer[x as usize] = false;
                v[mp[&mpp[&x]]].remove(&x);
            } else {
                if pwer[x as usize] {
                    ans.push(x + 1);
                    continue;
                }
                let mini = mpp[&x];
                if !v[mp[&mini]].is_empty() {
                    ans.push(*v[mp[&mini]].iter().next().unwrap() + 1);
                } else {
                    ans.push(-1);
                }
            }
        }
        ans
    }
}

#[test]
fn test() {
    let c = 5;
    let connections = vec![vec![1, 2], vec![2, 3], vec![4, 5]];
    let queries = vec![vec![1, 1], vec![2, 2], vec![1, 3], vec![2, 4]];
    let result = Solution::process_queries(c, connections, queries);
    assert_eq!(result, vec![1, 3]);

    let connections = vec![];
    let queries = vec![vec![1, 1], vec![2, 1], vec![1, 1]];
    let result = Solution::process_queries(c, connections, queries);
    assert_eq!(result, vec![1, -1]);
}
