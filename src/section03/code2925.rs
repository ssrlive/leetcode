#![allow(dead_code)]

// 2925. Maximum Score After Applying Operations on a Tree
// https://leetcode.com/problems/maximum-score-after-applying-operations-on-a-tree/
// https://leetcode.cn/problems/maximum-score-after-applying-operations-on-a-tree/
//
// Medium
//
// There is an undirected tree with n nodes labeled from 0 to n - 1, and rooted at node 0. You are given a 2D integer array
// edges of length n - 1, where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree.
//
// You are also given a 0-indexed integer array values of length n, where values[i] is the value associated with the ith node.
//
// You start with a score of 0. In one operation, you can:
//
//     Pick any node i.
//     Add values[i] to your score.
//     Set values[i] to 0.
//
// A tree is healthy if the sum of values on the path from the root to any leaf node is different than zero.
//
// Return the maximum score you can obtain after performing these operations on the tree any number of times so that it remains healthy.
//
// Example 1:
//
// Input: edges = [[0,1],[0,2],[0,3],[2,4],[4,5]], values = [5,2,5,2,1,1]
// Output: 11
// Explanation: We can choose nodes 1, 2, 3, 4, and 5. The value of the root is non-zero. Hence, the sum of values on
// the path from the root to any leaf is different than zero. Therefore, the tree is healthy and the score is
// values[1] + values[2] + values[3] + values[4] + values[5] = 11.
// It can be shown that 11 is the maximum score obtainable after any number of operations on the tree.
//
// Example 2:
//
// Input: edges = [[0,1],[0,2],[1,3],[1,4],[2,5],[2,6]], values = [20,10,9,7,4,3,5]
// Output: 40
// Explanation: We can choose nodes 0, 2, 3, and 4.
// - The sum of values on the path from 0 to 4 is equal to 10.
// - The sum of values on the path from 0 to 3 is equal to 10.
// - The sum of values on the path from 0 to 5 is equal to 3.
// - The sum of values on the path from 0 to 6 is equal to 5.
// Therefore, the tree is healthy and the score is values[0] + values[2] + values[3] + values[4] = 40.
// It can be shown that 40 is the maximum score obtainable after any number of operations on the tree.
//
// Constraints:
//
//     2 <= n <= 2 * 104
//     edges.length == n - 1
//     edges[i].length == 2
//     0 <= ai, bi < n
//     values.length == n
//     1 <= values[i] <= 109
//     The input is generated such that edges represents a valid tree.
//

struct Solution;

impl Solution {
    pub fn maximum_score_after_operations(edges: Vec<Vec<i32>>, values: Vec<i32>) -> i64 {
        let values = values.iter().map(|&v| v as i64).collect::<Vec<_>>();
        let mut al = vec![vec![]; values.len()];
        for e in edges {
            al[e[0] as usize].push(e[1] as i64);
            al[e[1] as usize].push(e[0] as i64);
        }
        let (_, healthy) = Self::dfs(0, -1, &al, &values);
        healthy
    }

    fn dfs(i: i64, p: i64, al: &Vec<Vec<i64>>, val: &Vec<i64>) -> (i64, i64) {
        let mut all = 0;
        let mut healthy = -val[i as usize];
        for &j in al[i as usize].iter() {
            if j != p {
                let (all_j, healthy_j) = Self::dfs(j, i, al, val);
                all += all_j;
                healthy = healthy.max(0) + healthy_j;
            }
        }
        (all + val[i as usize], all.max(healthy + val[i as usize]))
    }
}

#[test]
fn test() {
    let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![2, 4], vec![4, 5]];
    let values = vec![5, 2, 5, 2, 1, 1];
    let res = 11;
    assert_eq!(Solution::maximum_score_after_operations(edges, values), res);

    let edges = vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6]];
    let values = vec![20, 10, 9, 7, 4, 3, 5];
    let res = 40;
    assert_eq!(Solution::maximum_score_after_operations(edges, values), res);
}
