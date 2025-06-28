#![allow(dead_code)]

// 3593. Minimum Increments to Equalize Leaf Paths
// https://leetcode.com/problems/minimum-increments-to-equalize-leaf-paths/
// https://leetcode.cn/problems/minimum-increments-to-equalize-leaf-paths/
//
// Medium
//
// You are given an integer n and an undirected tree rooted at node 0 with n nodes numbered from 0 to n - 1. This is represented by a 2D array edges of length n - 1, where edges[i] = [ui, vi] indicates an edge from node ui to vi .
//
// Each node i has an associated cost given by cost[i], representing the cost to traverse that node.
//
// The score of a path is defined as the sum of the costs of all nodes along the path.
//
// Your goal is to make the scores of all root-to-leaf paths equal by increasing the cost of any number of nodes by any non-negative amount.
//
// Return the minimum number of nodes whose cost must be increased to make all root-to-leaf path scores equal.
//
// Example 1:
//
// Input: n = 3, edges = [[0,1],[0,2]], cost = [2,1,3]
//
// Output: 1
//
// Explanation:
//
// There are two root-to-leaf paths:
//
//     Path 0 → 1 has a score of 2 + 1 = 3.
//     Path 0 → 2 has a score of 2 + 3 = 5.
//
// To make all root-to-leaf path scores equal to 5, increase the cost of node 1 by 2.
// Only one node is increased, so the output is 1.
//
// Example 2:
//
// Input: n = 3, edges = [[0,1],[1,2]], cost = [5,1,4]
//
// Output: 0
//
// Explanation:
//
// There is only one root-to-leaf path:
//
//     Path 0 → 1 → 2 has a score of 5 + 1 + 4 = 10.
//
// Since only one root-to-leaf path exists, all path costs are trivially equal, and the output is 0.
//
// Example 3:
//
// Input: n = 5, edges = [[0,4],[0,1],[1,2],[1,3]], cost = [3,4,1,1,7]
//
// Output: 1
//
// Explanation:
//
// There are three root-to-leaf paths:
//
//     Path 0 → 4 has a score of 3 + 7 = 10.
//     Path 0 → 1 → 2 has a score of 3 + 4 + 1 = 8.
//     Path 0 → 1 → 3 has a score of 3 + 4 + 1 = 8.
//
// To make all root-to-leaf path scores equal to 10, increase the cost of node 1 by 2. Thus, the output is 1.
//
// Constraints:
//
//     2 <= n <= 105
//     edges.length == n - 1
//     edges[i] == [ui, vi]
//     0 <= ui, vi < n
//     cost.length == n
//     1 <= cost[i] <= 109
//     The input is generated such that edges represents a valid tree.
//

struct Solution;

impl Solution {
    pub fn min_increase(n: i32, edges: Vec<Vec<i32>>, cost: Vec<i32>) -> i32 {
        let mut g = vec![vec![]; n as usize];
        for e in edges {
            let u = e[0] as i64;
            let v = e[1] as i64;
            g[u as usize].push(v);
            g[v as usize].push(u);
        }

        fn dfs(i: i64, f: i64, g: &Vec<Vec<i64>>, cost: &Vec<i64>, res: &mut i64) -> i64 {
            let mut score = vec![];
            for &j in &g[i as usize] {
                if j == f {
                    continue;
                }
                score.push(dfs(j, i, g, cost, res));
            }
            if score.is_empty() {
                return cost[i as usize];
            }
            let ma = *score.iter().max().unwrap();
            for &v in &score {
                if ma - v > 0 {
                    *res += 1;
                }
            }
            ma + cost[i as usize]
        }

        let mut res_i64 = 0;
        let cost_i64: Vec<i64> = cost.iter().map(|&x| x as i64).collect();
        dfs(0, -1, &g, &cost_i64, &mut res_i64);
        res_i64 as i32
    }
}

#[test]
fn test() {
    let n = 3;
    let edges = vec![vec![0, 1], vec![0, 2]];
    let cost = vec![2, 1, 3];
    assert_eq!(Solution::min_increase(n, edges, cost), 1);

    let n = 3;
    let edges = vec![vec![0, 1], vec![1, 2]];
    let cost = vec![5, 1, 4];
    assert_eq!(Solution::min_increase(n, edges, cost), 0);

    let n = 5;
    let edges = vec![vec![0, 4], vec![0, 1], vec![1, 2], vec![1, 3]];
    let cost = vec![3, 4, 1, 1, 7];
    assert_eq!(Solution::min_increase(n, edges, cost), 1);
}
