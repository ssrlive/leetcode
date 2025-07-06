#![allow(dead_code)]

// 3604. Minimum Time to Reach Destination in Directed Graph
// https://leetcode.com/problems/minimum-time-to-reach-destination-in-directed-graph/
// https://leetcode.cn/problems/minimum-time-to-reach-destination-in-directed-graph/
//
// Medium
//
// You are given an integer n and a directed graph with n nodes labeled from 0 to n - 1.
// This is represented by a 2D array edges, where edges[i] = [ui, vi, starti, endi] indicates
// an edge from node ui to vi that can only be used at any integer time t such that starti <= t <= endi.
//
// You start at node 0 at time 0.
//
// In one unit of time, you can either:
//
//     Wait at your current node without moving, or
//     Travel along an outgoing edge from your current node if the current time t satisfies starti <= t <= endi.
//
// Return the minimum time required to reach node n - 1. If it is impossible, return -1.
//
// Example 1:
//
// Input: n = 3, edges = [[0,1,0,1],[1,2,2,5]]
//
// Output: 3
//
// Explanation:
//
// The optimal path is:
//
//     At time t = 0, take the edge (0 → 1) which is available from 0 to 1. You arrive at node 1 at time t = 1, then wait until t = 2.
//     At time t = 2, take the edge (1 → 2) which is available from 2 to 5. You arrive at node 2 at time 3.
//
// Hence, the minimum time to reach node 2 is 3.
//
// Example 2:
//
// Input: n = 4, edges = [[0,1,0,3],[1,3,7,8],[0,2,1,5],[2,3,4,7]]
//
// Output: 5
//
// Explanation:
//
// The optimal path is:
//
//     Wait at node 0 until time t = 1, then take the edge (0 → 2) which is available from 1 to 5. You arrive at node 2 at t = 2.
//     Wait at node 2 until time t = 4, then take the edge (2 → 3) which is available from 4 to 7. You arrive at node 3 at t = 5.
//
// Hence, the minimum time to reach node 3 is 5.
//
// Example 3:
//
// Input: n = 3, edges = [[1,0,1,3],[1,2,3,5]]
//
// Output: -1
//
// Explanation:
//
//     Since there is no outgoing edge from node 0, it is impossible to reach node 2. Hence, the output is -1.
//
// Constraints:
//
//     1 <= n <= 10^5
//     0 <= edges.length <= 10^5
//     edges[i] == [ui, vi, starti, endi]
//     0 <= ui, vi <= n - 1
//     ui != vi
//     0 <= starti <= endi <= 10^9
//

struct Solution;

impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        if n == 1 {
            return 0;
        }

        // Graph: node -> list of {neighbor, {startTime, endTime}}
        #[allow(clippy::type_complexity)]
        let mut mp: std::collections::HashMap<i32, Vec<(i32, (i32, i32))>> = std::collections::HashMap::new();
        for edge in edges {
            mp.entry(edge[0]).or_default().push((edge[1], (edge[2], edge[3])));
        }

        // Min-heap: {current time, node}
        let mut pq: std::collections::BinaryHeap<std::cmp::Reverse<(i32, i32)>> = std::collections::BinaryHeap::new();
        let mut time = vec![i32::MAX; n];

        // Start from node 0 at time 0
        pq.push(std::cmp::Reverse((0, 0)));

        while let Some(std::cmp::Reverse((t, node))) = pq.pop() {
            // Already visited with a better time
            if t >= time[node as usize] {
                continue;
            }

            time[node as usize] = t;

            if let Some(neighbors) = mp.get(&node) {
                for &(nxt, (st, ed)) in neighbors {
                    let mut temp = t;
                    if temp > ed {
                        continue; // Edge not usable
                    }
                    if temp < st {
                        temp = st; // Wait until start time
                    }
                    temp += 1; // Travel takes 1 unit time

                    if temp < time[nxt as usize] {
                        pq.push(std::cmp::Reverse((temp, nxt)));
                    }
                }
            }
        }

        if time[n - 1] == i32::MAX { -1 } else { time[n - 1] }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_time(3, vec![vec![1, 0, 1, 3], vec![1, 2, 3, 5]]), -1);
    assert_eq!(Solution::min_time(3, vec![vec![0, 1, 0, 1], vec![1, 2, 2, 5]]), 3);
    assert_eq!(
        Solution::min_time(4, vec![vec![0, 1, 0, 3], vec![1, 3, 7, 8], vec![0, 2, 1, 5], vec![2, 3, 4, 7]]),
        5
    );
}
