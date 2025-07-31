#![allow(dead_code)]

// 3620. Network Recovery Pathways
// https://leetcode.com/problems/network-recovery-pathways/
// https://leetcode.cn/problems/network-recovery-pathways/
//
// Hard
//
// You are given a directed acyclic graph of n nodes numbered from 0 to n − 1. This is represented by a 2D array edges of length m, where edges[i] = [ui, vi, costi] indicates a one‑way communication from node ui to node vi with a recovery cost of costi.
//
// Some nodes may be offline. You are given a boolean array online where online[i] = true means node i is online. Nodes 0 and n − 1 are always online.
//
// A path from 0 to n − 1 is valid if:
//
// All intermediate nodes on the path are online.
// The total recovery cost of all edges on the path does not exceed k.
// For each valid path, define its score as the minimum edge‑cost along that path.
//
// Return the maximum path score (i.e., the largest minimum-edge cost) among all valid paths. If no valid path exists, return -1.
//
// Example 1:
//
// Input: edges = [[0,1,5],[1,3,10],[0,2,3],[2,3,4]], online = [true,true,true,true], k = 10
//
// Output: 3
//
// Explanation:
//
// The graph has two possible routes from node 0 to node 3:
//
// Path 0 → 1 → 3
//
// Total cost = 5 + 10 = 15, which exceeds k (15 > 10), so this path is invalid.
//
// Path 0 → 2 → 3
//
// Total cost = 3 + 4 = 7 <= k, so this path is valid.
//
// The minimum edge‐cost along this path is min(3, 4) = 3.
//
// There are no other valid paths. Hence, the maximum among all valid path‐scores is 3.
//
// Example 2:
//
// Input: edges = [[0,1,7],[1,4,5],[0,2,6],[2,3,6],[3,4,2],[2,4,6]], online = [true,true,true,false,true], k = 12
//
// Output: 6
//
// Explanation:
//
// Node 3 is offline, so any path passing through 3 is invalid.
//
// Consider the remaining routes from 0 to 4:
//
// Path 0 → 1 → 4
//
// Total cost = 7 + 5 = 12 <= k, so this path is valid.
//
// The minimum edge‐cost along this path is min(7, 5) = 5.
//
// Path 0 → 2 → 3 → 4
//
// Node 3 is offline, so this path is invalid regardless of cost.
//
// Path 0 → 2 → 4
//
// Total cost = 6 + 6 = 12 <= k, so this path is valid.
//
// The minimum edge‐cost along this path is min(6, 6) = 6.
//
// Among the two valid paths, their scores are 5 and 6. Therefore, the answer is 6.
//
// Constraints:
//
// n == online.length
// 2 <= n <= 5 * 10^4
// 0 <= m == edges.length <= min(10^5, n * (n - 1) / 2)
// edges[i] = [ui, vi, costi]
// 0 <= ui, vi < n
// ui != vi
// 0 <= costi <= 10^9
// 0 <= k <= 5 * 10^13
// online[i] is either true or false, and both online[0] and online[n − 1] are true.
// The given graph is a directed acyclic graph.
//

struct Solution;

impl Solution {
    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        fn solve(src: i64, target: i64, adj: &[Vec<(i64, i64)>], n: i64) -> i64 {
            let mut dist = vec![i64::MAX / 4; n as usize];
            dist[src as usize] = 0;
            let mut pq = std::collections::BinaryHeap::new();
            pq.push(std::cmp::Reverse((0, src)));
            while let Some(std::cmp::Reverse((d, u))) = pq.pop() {
                if d > dist[u as usize] {
                    continue;
                }
                if u == target {
                    return d;
                }
                for &(v, w) in &adj[u as usize] {
                    if dist[v as usize] > d + w {
                        dist[v as usize] = d + w;
                        pq.push(std::cmp::Reverse((dist[v as usize], v)));
                    }
                }
            }
            i64::MAX / 4
        }

        let n = online.len() as i64;
        let (mut l, mut h) = (0, 1_000_000_000);
        let mut best = -1;
        while l <= h {
            let mid = l + (h - l) / 2;
            let mut adj = vec![vec![]; n as usize];
            for e in &edges {
                let u = e[0] as i64;
                let v = e[1] as i64;
                let c = e[2] as i64;
                if c >= mid && online[u as usize] && online[v as usize] {
                    adj[u as usize].push((v, c));
                }
            }
            let dist = solve(0, n - 1, &adj, n);
            if dist <= k {
                best = mid;
                l = mid + 1;
            } else {
                h = mid - 1;
            }
        }
        best as _
    }
}

#[test]
fn test() {
    let edges = vec![vec![0, 1, 5], vec![1, 2, 6], vec![0, 2, 10]];
    let online = vec![true, true, true];
    let k = 12;
    assert_eq!(Solution::find_max_path_score(edges, online, k), 10);

    let edges = vec![
        vec![0, 1, 7],
        vec![1, 4, 5],
        vec![0, 2, 6],
        vec![2, 3, 6],
        vec![3, 4, 2],
        vec![2, 4, 6],
    ];
    let online = vec![true, true, true, true, true];
    let k = 12;
    assert_eq!(Solution::find_max_path_score(edges, online, k), 6);
}
