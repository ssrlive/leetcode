#![allow(dead_code)]

// 743. Network Delay Time
// https://leetcode.com/problems/network-delay-time/
// https://leetcode.cn/problems/network-delay-time/
//
// You are given a network of n nodes, labeled from 1 to n. You are also given times, a list of travel times as directed edges times[i] = (ui, vi, wi),
// where ui is the source node, vi is the target node, and wi is the time it takes for a signal to travel from source to target.
//
// We will send a signal from a given node k. Return the minimum time it takes for all the n nodes to receive the signal.
// If it is impossible for all the n nodes to receive the signal, return -1.
//
// Example 1:
//
// Input: times = [[2,1,1],[2,3,1],[3,4,1]], n = 4, k = 2
// Output: 2
//
// Example 2:
//
// Input: times = [[1,2,1]], n = 2, k = 1
// Output: 1
//
// Example 3:
//
// Input: times = [[1,2,1]], n = 2, k = 2
// Output: -1
//
// Constraints:
//
// - 1 <= k <= n <= 100
// - 1 <= times.length <= 6000
// - times[i].length == 3
// - 1 <= ui, vi <= n
// - ui != vi
// - 0 <= wi <= 100
// - All the pairs (ui, vi) are unique. (i.e., no multiple edges.)
//

struct Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        fn _network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> Option<i32> {
            let mut graph = vec![vec![]; n as usize];
            for time in times {
                graph[time[0] as usize - 1].push((time[1] as usize - 1, time[2]));
            }

            let mut dist = vec![std::i32::MAX; n as usize];
            dist[k as usize - 1] = 0;

            let mut heap = std::collections::BinaryHeap::new();
            heap.push(std::cmp::Reverse((0, k as usize - 1)));

            while let Some(std::cmp::Reverse((d, u))) = heap.pop() {
                if d > dist[u] {
                    continue;
                }

                for &(v, w) in &graph[u] {
                    if dist[u] + w < dist[v] {
                        dist[v] = dist[u] + w;
                        heap.push(std::cmp::Reverse((dist[v], v)));
                    }
                }
            }

            let ans = dist.iter().max()?;
            if *ans == std::i32::MAX {
                Some(-1)
            } else {
                Some(*ans)
            }
        }

        _network_delay_time(times, n, k).unwrap_or(-1)
    }
}

#[test]
fn test() {
    let times = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
    assert_eq!(Solution::network_delay_time(times, 4, 2), 2);
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1]], 2, 1), 1);
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1]], 2, 2), -1);
}
