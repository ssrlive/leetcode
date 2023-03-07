#![allow(dead_code)]

/*

// 2359. Find Closest Node to Given Two Nodes
// https://leetcode.com/problems/find-closest-node-to-given-two-nodes/
// https://leetcode.cn/problems/find-closest-node-to-given-two-nodes/
//
// Medium
//
// You are given a directed graph of n nodes numbered from 0 to n - 1, where each node has at most one outgoing edge.

The graph is represented with a given 0-indexed array edges of size n, indicating that there is a directed edge from node i to node edges[i]. If there is no outgoing edge from i, then edges[i] == -1.

You are also given two integers node1 and node2.

Return the index of the node that can be reached from both node1 and node2, such that the maximum between the distance from node1 to that node, and from node2 to that node is minimized. If there are multiple answers, return the node with the smallest index, and if no possible answer exists, return -1.

Note that edges may contain cycles.

Example 1:

Input: edges = [2,2,3,-1], node1 = 0, node2 = 1
Output: 2
Explanation: The distance from node 0 to node 2 is 1, and the distance from node 1 to node 2 is 1.
The maximum of those two distances is 1. It can be proven that we cannot get a node with a smaller maximum distance than 1, so we return node 2.

Example 2:

Input: edges = [1,2,-1], node1 = 0, node2 = 2
Output: 2
Explanation: The distance from node 0 to node 2 is 2, and the distance from node 2 to itself is 0.
The maximum of those two distances is 2. It can be proven that we cannot get a node with a smaller maximum distance than 2, so we return node 2.

Constraints:

    n == edges.length
    2 <= n <= 10^5
    -1 <= edges[i] < n
    edges[i] != i
    0 <= node1, node2 < n
*/

struct Solution;

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let n = edges.len();
        let mut d1 = vec![n; n];
        let mut d2 = vec![n; n];
        d1[node1 as usize] = 0;
        d2[node2 as usize] = 0;
        let mut u = node1 as usize;
        while edges[u] != -1 {
            let v = edges[u] as usize;
            if d1[v] < d1[u] {
                break;
            }
            d1[v] = d1[u] + 1;
            u = v;
        }
        u = node2 as usize;
        while edges[u] != -1 {
            let v = edges[u] as usize;
            if d2[v] < d2[u] {
                break;
            }
            d2[v] = d2[u] + 1;
            u = v;
        }
        let mut ret = -1i32;
        let mut mx = n;
        for i in 0..n {
            let temp = d1[i].max(d2[i]);
            if temp >= mx {
                continue;
            }
            ret = i as i32;
            mx = temp;
        }
        ret
    }
}

#[test]
fn test() {
    assert_eq!(Solution::closest_meeting_node(vec![2, 2, 3, -1], 0, 1), 2);
    assert_eq!(Solution::closest_meeting_node(vec![1, 2, -1], 0, 2), 2);
}
