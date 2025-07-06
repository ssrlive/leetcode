#![allow(dead_code)]

// 3608. Minimum Time for K Connected Components
// https://leetcode.com/problems/minimum-time-for-k-connected-components/
// https://leetcode.cn/problems/minimum-time-for-k-connected-components/
//
// Medium
//
// You are given an integer n and an undirected graph with n nodes labeled from 0 to n - 1.
// This is represented by a 2D array edges, where edges[i] = [ui, vi, timei] indicates
// an undirected edge between nodes ui and vi that can be removed at timei.
//
// You are also given an integer k.
//
// Initially, the graph may be connected or disconnected. Your task is to find the minimum time t such
// that after removing all edges with time <= t, the graph contains at least k connected components.
//
// Return the minimum time t.
//
// A connected component is a subgraph of a graph in which there exists a path between any two vertices,
// and no vertex of the subgraph shares an edge with a vertex outside of the subgraph.
//
// Example 1:
//
// Input: n = 2, edges = [[0,1,3]], k = 2
//
// Output: 3
//
// Explanation:
//
//     Initially, there is one connected component {0, 1}.
//     At time = 1 or 2, the graph remains unchanged.
//     At time = 3, edge [0, 1] is removed, resulting in k = 2 connected components {0}, {1}. Thus, the answer is 3.
//
// Example 2:
//
// Input: n = 3, edges = [[0,1,2],[1,2,4]], k = 3
//
// Output: 4
//
// Explanation:
//
//     Initially, there is one connected component {0, 1, 2}.
//     At time = 2, edge [0, 1] is removed, resulting in two connected components {0}, {1, 2}.
//     At time = 4, edge [1, 2] is removed, resulting in k = 3 connected components {0}, {1}, {2}. Thus, the answer is 4.
//
// Example 3:
//
// Input: n = 3, edges = [[0,2,5]], k = 2
//
// Output: 0
//
// Explanation:
//
//     Since there are already k = 2 disconnected components {1}, {0, 2}, no edge removal is needed. Thus, the answer is 0.
//
// Constraints:
//
//     1 <= n <= 10^5
//     0 <= edges.length <= 10^5
//     edges[i] = [ui, vi, timei]
//     0 <= ui, vi < n
//     ui != vi
//     1 <= timei <= 10^9
//     1 <= k <= n
//     There are no duplicate edges.
//

struct Solution;

impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        fn find(parent: &mut [i32], i: i32) -> i32 {
            if parent[i as usize] == i {
                return i;
            }
            parent[i as usize] = find(parent, parent[i as usize]);
            parent[i as usize]
        }
        fn unite(parent: &mut [i32], i: i32, j: i32) -> bool {
            let root_i = find(parent, i);
            let root_j = find(parent, j);
            if root_i != root_j {
                parent[root_i as usize] = root_j;
                true
            } else {
                false
            }
        }

        if edges.is_empty() {
            return 0;
        }
        let mut edges = edges;
        edges.sort_unstable_by(|a, b| b[2].cmp(&a[2]));
        let mut parent: Vec<i32> = (0..n).collect();
        let mut count = n;
        for e in edges {
            let u = e[0];
            let v = e[1];
            let t = e[2];
            if unite(&mut parent, u, v) {
                count -= 1;
            }
            if count < k {
                return t;
            }
        }
        0
    }
}

#[test]
fn test() {
    let n = 2;
    let edges = vec![vec![0, 1, 3]];
    let k = 2;
    assert_eq!(Solution::min_time(n, edges, k), 3);

    let n = 3;
    let edges = vec![vec![0, 1, 2], vec![1, 2, 4]];
    let k = 3;
    assert_eq!(Solution::min_time(n, edges, k), 4);

    let n = 3;
    let edges = vec![vec![0, 2, 5]];
    let k = 2;
    assert_eq!(Solution::min_time(n, edges, k), 0);
}
