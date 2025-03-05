#![allow(dead_code)]

/*

// 1514. Path with Maximum Probability
// https://leetcode.com/problems/path-with-maximum-probability/
// https://leetcode.cn/problems/path-with-maximum-probability/
//
// Medium
//
// You are given an undirected weighted graph of n nodes (0-indexed), represented by an edge list where edges[i] = [a, b]
// is an undirected edge connecting the nodes a and b with a probability of success of traversing that edge succProb[i].

Given two nodes start and end, find the path with the maximum probability of success to go from start to end and return its success probability.

If there is no path from start to end, return 0. Your answer will be accepted if it differs from the correct answer by at most 1e-5.

Example 1:

Input: n = 3, edges = [[0,1],[1,2],[0,2]], succProb = [0.5,0.5,0.2], start = 0, end = 2
Output: 0.25000
Explanation: There are two paths from start to end, one having a probability of success = 0.2 and the other has 0.5 * 0.5 = 0.25.

Example 2:

Input: n = 3, edges = [[0,1],[1,2],[0,2]], succProb = [0.5,0.5,0.3], start = 0, end = 2
Output: 0.30000

Example 3:

Input: n = 3, edges = [[0,1]], succProb = [0.5], start = 0, end = 2
Output: 0.00000
Explanation: There is no path between 0 and 2.

Constraints:

    2 <= n <= 10^4
    0 <= start, end < n
    start != end
    0 <= a, b < n
    a != b
    0 <= succProb.length == edges.length <= 2*10^4
    0 <= succProb[i] <= 1
    There is at most one edge between every two nodes.
*/

struct Solution;

impl Solution {
    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start: i32, end: i32) -> f64 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for i in 0..edges.len() {
            let a = edges[i][0] as usize;
            let b = edges[i][1] as usize;
            g[a].push((b, succ_prob[i]));
            g[b].push((a, succ_prob[i]));
        }

        let s = start as usize;
        let e = end as usize;
        let mut stack = vec![(s, 1.0)];
        let mut memo = vec![-10000.0; n];

        while !stack.is_empty() {
            let mut new_stack = vec![];
            while let Some((ci, cv)) = stack.pop() {
                for &(ni, v) in &g[ci] {
                    let nv = cv * v;
                    if nv > memo[ni] {
                        memo[ni] = nv;
                        new_stack.push((ni, nv));
                    }
                }
            }
            stack = new_stack;
        }

        if memo[e] == -10000.0 { 0.0 } else { memo[e] }
    }
}

#[test]
fn test() {
    let cases = vec![
        (3, vec![vec![0, 1], vec![1, 2], vec![0, 2]], vec![0.5, 0.5, 0.2], 0, 2, 0.25000),
        (3, vec![vec![0, 1], vec![1, 2], vec![0, 2]], vec![0.5, 0.5, 0.3], 0, 2, 0.30000),
        (3, vec![vec![0, 1]], vec![0.5], 0, 2, 0.00000),
    ];
    for (n, edges, succ_prob, start, end, expect) in cases {
        assert_eq!(Solution::max_probability(n, edges, succ_prob, start, end), expect);
    }
}
