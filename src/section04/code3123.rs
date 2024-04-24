#![allow(dead_code)]

// 3123. Find Edges in Shortest Paths
// https://leetcode.com/problems/find-edges-in-shortest-paths/
// https://leetcode.cn/problems/find-edges-in-shortest-paths/
//
// Hard
//
// You are given an undirected weighted graph of n nodes numbered from 0 to n - 1.
// The graph consists of m edges represented by a 2D array edges,
// where edges[i] = [ai, bi, wi] indicates that there is an edge between nodes ai and bi with weight wi.
//
// Consider all the shortest paths from node 0 to node n - 1 in the graph.
// You need to find a boolean array answer where answer[i] is true if the edge edges[i] is part of
// at least one shortest path. Otherwise, answer[i] is false.
//
// Return the array answer.
//
// Note that the graph may not be connected.
//
// Example 1:
//
// Input: n = 6, edges = [[0,1,4],[0,2,1],[1,3,2],[1,4,3],[1,5,1],[2,3,1],[3,5,3],[4,5,2]]
//
// Output: [true,true,true,false,true,true,true,false]
//
// Explanation:
//
// The following are all the shortest paths between nodes 0 and 5:
//
// The path 0 -> 1 -> 5: The sum of weights is 4 + 1 = 5.
// The path 0 -> 2 -> 3 -> 5: The sum of weights is 1 + 1 + 3 = 5.
// The path 0 -> 2 -> 3 -> 1 -> 5: The sum of weights is 1 + 1 + 2 + 1 = 5.
//
// Example 2:
//
// Input: n = 4, edges = [[2,0,1],[0,1,1],[0,3,4],[3,2,2]]
//
// Output: [true,false,false,true]
//
// Explanation:
//
// There is one shortest path between nodes 0 and 3, which is the path 0 -> 2 -> 3 with the sum of weights 1 + 2 = 3.
//
// Constraints:
//
// 2 <= n <= 5 * 10^4
// m == edges.length
// 1 <= m <= min(5 * 10^4, n * (n - 1) / 2)
// 0 <= a[i], b[i] < n
// a[i] != b[i]
// 1 <= w[i] <= 10^5
// There are no repeated edges.
//

struct Solution;

impl Solution {
    pub fn find_answer(n: i32, edges: Vec<Vec<i32>>) -> Vec<bool> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let (m, n) = (edges.len(), n as usize);
        let mut g = vec![vec![]; n];

        for (i, edges_i) in edges.iter().enumerate().take(m) {
            let (u, v) = (edges_i[0] as usize, edges_i[1] as usize);
            let w = edges[i][2];
            g[u].push((v, w, i));
            g[v].push((u, w, i));
        }

        let mut pq = BinaryHeap::new();
        let mut dist = vec![i64::MAX; n];
        pq.push(Reverse((0, 0)));
        while let Some(Reverse((d, u))) = pq.pop() {
            if d >= dist[u] {
                continue;
            }
            dist[u] = d;

            for (v, w, _i) in &g[u] {
                if dist[u] + *w as i64 >= dist[*v] {
                    continue;
                }
                pq.push(Reverse((dist[u] + *w as i64, *v)));
            }
        }

        let mut sk = vec![];
        let mut ret = vec![false; m];

        sk.push(n - 1);
        while let Some(u) = sk.pop() {
            if dist[u] == i64::MAX {
                continue;
            }
            for (v, w, i) in &g[u] {
                if dist[u] < dist[*v] + *w as i64 {
                    continue;
                }
                ret[*i] = true;
                sk.push(*v);
            }
        }

        ret
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::find_answer(
            6,
            vec![
                vec![0, 1, 4],
                vec![0, 2, 1],
                vec![1, 3, 2],
                vec![1, 4, 3],
                vec![1, 5, 1],
                vec![2, 3, 1],
                vec![3, 5, 3],
                vec![4, 5, 2]
            ]
        ),
        vec![true, true, true, false, true, true, true, false]
    );
    assert_eq!(
        Solution::find_answer(4, vec![vec![2, 0, 1], vec![0, 1, 1], vec![0, 3, 4], vec![3, 2, 2]]),
        vec![true, false, false, true]
    );
}
