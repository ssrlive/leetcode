#![allow(dead_code)]

/*

// 2360. Longest Cycle in a Graph
// https://leetcode.com/problems/longest-cycle-in-a-graph/
// https://leetcode.cn/problems/longest-cycle-in-a-graph/
//
// Hard
//
// You are given a directed graph of n nodes numbered from 0 to n - 1, where each node has at most one outgoing edge.

The graph is represented with a given 0-indexed array edges of size n, indicating that there is a directed edge from node i to node edges[i]. If there is no outgoing edge from node i, then edges[i] == -1.

Return the length of the longest cycle in the graph. If no cycle exists, return -1.

A cycle is a path that starts and ends at the same node.

Example 1:

Input: edges = [3,3,4,2,3]
Output: 3
Explanation: The longest cycle in the graph is the cycle: 2 -> 4 -> 3 -> 2.
The length of this cycle is 3, so 3 is returned.

Example 2:

Input: edges = [2,-1,3,1]
Output: -1
Explanation: There are no cycles in this graph.

Constraints:

    n == edges.length
    2 <= n <= 10^5
    -1 <= edges[i] < n
    edges[i] != i
*/

struct Solution;

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let n = edges.len();
        let (mut ret, mut seq) = (-1, 0);
        let mut flag = vec![-1; n];
        for i in 0..n {
            if flag[i] != -1 {
                continue;
            }
            let (mut u, start) = (i, seq);
            flag[u] = seq;
            seq += 1;
            while edges[u] != -1 {
                let v = edges[u] as usize;
                if flag[v] != -1 {
                    if flag[v] < start {
                        break;
                    }
                    ret = ret.max(flag[u] - flag[v] + 1);
                    break;
                }
                flag[v] = seq;
                seq += 1;
                u = v;
            }
        }
        ret
    }
}

#[test]
fn test() {
    let cases = vec![(vec![3, 3, 4, 2, 3], 3), (vec![2, -1, 3, 1], -1)];
    for (edges, expected) in cases {
        assert_eq!(Solution::longest_cycle(edges), expected);
    }
}
