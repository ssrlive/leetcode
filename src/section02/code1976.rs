#![allow(dead_code)]

/*

// 1976. Number of Ways to Arrive at Destination
// https://leetcode.com/problems/number-of-ways-to-arrive-at-destination/
// https://leetcode.cn/problems/number-of-ways-to-arrive-at-destination/
//
// Medium
//
// You are in a city that consists of n intersections numbered from 0 to n - 1 with bi-directional roads between some intersections. The inputs are generated such that you can reach any intersection from any other intersection and that there is at most one road between any two intersections.

You are given an integer n and a 2D integer array roads where roads[i] = [ui, vi, timei] means that there is a road between intersections ui and vi that takes timei minutes to travel. You want to know in how many ways you can travel from intersection 0 to intersection n - 1 in the shortest amount of time.

Return the number of ways you can arrive at your destination in the shortest amount of time. Since the answer may be large, return it modulo 109 + 7.

Example 1:

Input: n = 7, roads = [[0,6,7],[0,1,2],[1,2,3],[1,3,3],[6,3,3],[3,5,1],[6,5,1],[2,5,1],[0,4,5],[4,6,2]]
Output: 4
Explanation: The shortest amount of time it takes to go from intersection 0 to intersection 6 is 7 minutes.
The four ways to get there in 7 minutes are:
- 0 ➝ 6
- 0 ➝ 4 ➝ 6
- 0 ➝ 1 ➝ 2 ➝ 5 ➝ 6
- 0 ➝ 1 ➝ 3 ➝ 5 ➝ 6

Example 2:

Input: n = 2, roads = [[1,0,10]]
Output: 1
Explanation: There is only one way to go from intersection 0 to intersection 1, and it takes 10 minutes.

Constraints:

    1 <= n <= 200
    n - 1 <= roads.length <= n * (n - 1) / 2
    roads[i].length == 3
    0 <= ui, vi <= n - 1
    1 <= timei <= 10^9
    ui != vi
    There is at most one road connecting any two intersections.
    You can reach any intersection from any other intersection.
*/

struct Solution;

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let n = n as usize;
        let mut graph = vec![vec![]; n];

        for r in roads {
            let (u, v) = (r[0] as usize, r[1] as usize);
            graph[u].push((v, r[2]));
            graph[v].push((u, r[2]));
        }

        let mut dist = vec![i64::MAX; n];
        let mut pq = BinaryHeap::from([Reverse((0, 0))]);
        dist[0] = 0;

        while let Some(Reverse((d, u))) = pq.pop() {
            if d > dist[u] {
                continue;
            }

            for (v, cost) in &graph[u] {
                if dist[*v] <= dist[u] + *cost as i64 {
                    continue;
                }
                dist[*v] = dist[u] + *cost as i64;
                pq.push(Reverse((dist[*v], *v)));
            }
        }
        let mut pq = BinaryHeap::new();
        let mut count = vec![0; n];
        count[0] = 1;
        for (i, &dist_i) in dist.iter().enumerate() {
            pq.push(Reverse((dist_i, i)));
        }
        while let Some(Reverse((_, u))) = pq.pop() {
            if u == n - 1 {
                return count[u];
            }
            for (v, cost) in &graph[u] {
                if dist[*v] == dist[u] + *cost as i64 {
                    count[*v] = (count[*v] + count[u]) % 1_000_000_007;
                }
            }
        }
        0
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            7,
            vec![
                vec![0, 6, 7],
                vec![0, 1, 2],
                vec![1, 2, 3],
                vec![1, 3, 3],
                vec![6, 3, 3],
                vec![3, 5, 1],
                vec![6, 5, 1],
                vec![2, 5, 1],
                vec![0, 4, 5],
                vec![4, 6, 2],
            ],
            4,
        ),
        (2, vec![vec![1, 0, 10]], 1),
    ];
    for (n, roads, expected) in cases {
        assert_eq!(Solution::count_paths(n, roads), expected);
    }
}
