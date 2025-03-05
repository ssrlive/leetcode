#![allow(dead_code)]

// 3241. Time Taken to Mark All Nodes
// https://leetcode.com/problems/time-taken-to-mark-all-nodes/
// https://leetcode.cn/problems/time-taken-to-mark-all-nodes/
//
// Hard
//
// There exists an undirected tree with n nodes numbered 0 to n - 1. You are given a 2D integer array edges of length n - 1,
// where edges[i] = [ui, vi] indicates that there is an edge between nodes ui and vi in the tree.
//
// Initially, all nodes are unmarked. For each node i:
//
//     If i is odd, the node will get marked at time x if there is at least one node adjacent to it which was marked at time x - 1.
//     If i is even, the node will get marked at time x if there is at least one node adjacent to it which was marked at time x - 2.
//
// Return an array times where times[i] is the time when all nodes get marked in the tree, if you mark node i at time t = 0.
//
// Note that the answer for each times[i] is independent, i.e. when you mark node i all other nodes are unmarked.
//
// Example 1:
//
// Input: edges = [[0,1],[0,2]]
//
// Output: [2,4,3]
//
// Explanation:
//
//     For i = 0:
//         Node 1 is marked at t = 1, and Node 2 at t = 2.
//     For i = 1:
//         Node 0 is marked at t = 2, and Node 2 at t = 4.
//     For i = 2:
//         Node 0 is marked at t = 2, and Node 1 at t = 3.
//
// Example 2:
//
// Input: edges = [[0,1]]
//
// Output: [1,2]
//
// Explanation:
//
//     For i = 0:
//         Node 1 is marked at t = 1.
//     For i = 1:
//         Node 0 is marked at t = 2.
//
// Example 3:
//
// Input: edges = [[2,4],[0,1],[2,3],[0,2]]
//
// Output: [4,6,3,5,5]
//
// Explanation:
//
// Constraints:
//
//     2 <= n <= 10^5
//     edges.length == n - 1
//     edges[i].length == 2
//     0 <= edges[i][0], edges[i][1] <= n - 1
//     The input is generated such that edges represents a valid tree.
//

struct Solution;

impl Solution {
    pub fn time_taken(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut e = vec![Vec::new(); n];
        for edge in &edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            e[a].push(b);
            e[b].push(a);
        }

        let mut as_root = vec![0; n];
        Self::as_root(0, n, &e, &mut as_root);
        let mut result = vec![0; n];
        Self::get_result(0, n, 0, &e, &as_root, &mut result);
        result
    }

    fn as_root(node: usize, from: usize, e: &Vec<Vec<usize>>, arr: &mut Vec<i32>) -> i32 {
        let mut res = 0;
        for &c in &e[node] {
            if c == from {
                continue;
            }
            res = res.max(Self::as_root(c, node, e, arr));
        }
        arr[node] = res;
        res + 2 - node as i32 % 2
    }

    fn get_result(node: usize, from: usize, up_val: i32, e: &Vec<Vec<usize>>, as_root: &Vec<i32>, result: &mut Vec<i32>) {
        result[node] = as_root[node].max(up_val);
        let mut rec = ((up_val, from), 0);
        for &c in &e[node] {
            if c == from {
                continue;
            }
            let t = as_root[c] + 2 - c as i32 % 2;
            if t > rec.0.0 {
                rec.1 = rec.0.0;
                rec.0 = (t, c);
            } else if t > rec.1 {
                rec.1 = t;
            }
        }
        for &c in &e[node] {
            if c == from {
                continue;
            }
            Self::get_result(
                c,
                node,
                if c == rec.0.1 { rec.1 } else { rec.0.0 } + 2 - node as i32 % 2,
                e,
                as_root,
                result,
            );
        }
    }
}

#[test]
fn test() {
    let edges = vec![vec![0, 1], vec![0, 2]];
    let expected = vec![2, 4, 3];
    assert_eq!(Solution::time_taken(edges), expected);

    let edges = vec![vec![0, 1]];
    let expected = vec![1, 2];
    assert_eq!(Solution::time_taken(edges), expected);

    let edges = vec![vec![2, 4], vec![0, 1], vec![2, 3], vec![0, 2]];
    let expected = vec![4, 6, 3, 5, 5];
    assert_eq!(Solution::time_taken(edges), expected);
}
