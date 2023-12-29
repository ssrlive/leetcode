#![allow(dead_code)]

// 2973. Find Number of Coins to Place in Tree Nodes
// https://leetcode.com/problems/find-number-of-coins-to-place-in-tree-nodes/
// https://leetcode.cn/problems/find-number-of-coins-to-place-in-tree-nodes/
//
// Hard
//
// You are given an undirected tree with n nodes labeled from 0 to n - 1, and rooted at node 0.
// You are given a 2D integer array edges of length n - 1, where edges[i] = [ai, bi] indicates
// that there is an edge between nodes ai and bi in the tree.
//
// You are also given a 0-indexed integer array cost of length n, where cost[i] is the cost assigned to the ith node.
//
// You need to place some coins on every node of the tree. The number of coins to be placed at node i can be calculated as:
//
//    - If size of the subtree of node i is less than 3, place 1 coin.
//    - Otherwise, place an amount of coins equal to the maximum product of cost values assigned
//      to 3 distinct nodes in the subtree of node i. If this product is negative, place 0 coins.
//
// Return an array coin of size n such that coin[i] is the number of coins placed at node i.
//
// Example 1:
//
// Input: edges = [[0,1],[0,2],[0,3],[0,4],[0,5]], cost = [1,2,3,4,5,6]
// Output: [120,1,1,1,1,1]
// Explanation: For node 0 place 6 * 5 * 4 = 120 coins. All other nodes are leaves with subtree of size 1, place 1 coin on each of them.
//
// Example 2:
//
// Input: edges = [[0,1],[0,2],[1,3],[1,4],[1,5],[2,6],[2,7],[2,8]], cost = [1,4,2,3,5,7,8,-4,2]
// Output: [280,140,32,1,1,1,1,1,1]
// Explanation: The coins placed on each node are:
// - Place 8 * 7 * 5 = 280 coins on node 0.
// - Place 7 * 5 * 4 = 140 coins on node 1.
// - Place 8 * 2 * 2 = 32 coins on node 2.
// - All other nodes are leaves with subtree of size 1, place 1 coin on each of them.
//
// Example 3:
//
// Input: edges = [[0,1],[0,2]], cost = [1,2,-2]
// Output: [0,1,1]
// Explanation: Node 1 and 2 are leaves with subtree of size 1, place 1 coin on each of them.
// For node 0 the only possible product of cost is 2 * 1 * -2 = -4. Hence place 0 coins on node 0.
//
// Constraints:
//
//     2 <= n <= 2 * 10^4
//     edges.length == n - 1
//     edges[i].length == 2
//     0 <= ai, bi < n
//     cost.length == n
//     1 <= |cost[i]| <= 10^4
//     The input is generated such that edges represents a valid tree.
//

struct Solution;

impl Solution {
    const MAX_NODES: usize = 3;

    pub fn placed_coins(edges: Vec<Vec<i32>>, cost: Vec<i32>) -> Vec<i64> {
        let mut ans = vec![0; cost.len()];
        let mut t = vec![vec![]; cost.len()];
        for e in edges {
            t[e[0] as usize].push(e[1] as usize);
            t[e[1] as usize].push(e[0] as usize);
        }
        Solution::dfs(&t, &cost, 0, -1, &mut ans);
        ans
    }

    fn dfs(t: &Vec<Vec<usize>>, cost: &Vec<i32>, root: usize, par: i32, ans: &mut Vec<i64>) -> Vec<i64> {
        let mut useful_cost = vec![cost[root] as i64];
        for &n in &t[root] {
            if n as i32 == par {
                continue;
            }
            let v = Solution::dfs(t, cost, n, root as i32, ans);
            for &e in &v {
                useful_cost.push(e);
            }
        }
        useful_cost.sort_unstable_by(|a, b| b.cmp(a));
        let sz = useful_cost.len();
        if sz < Solution::MAX_NODES {
            ans[root] = 1;
            return useful_cost;
        }
        if useful_cost[1] * useful_cost[2] > useful_cost[sz - 1] * useful_cost[sz - 2] {
            ans[root] = useful_cost[0] * useful_cost[1] * useful_cost[2];
        } else {
            ans[root] = useful_cost[0] * useful_cost[sz - 1] * useful_cost[sz - 2];
        }
        if ans[root] < 0 {
            ans[root] = 0;
        }
        if sz <= 5 {
            return useful_cost;
        }
        vec![
            useful_cost[0],
            useful_cost[1],
            useful_cost[2],
            useful_cost[sz - 2],
            useful_cost[sz - 1],
        ]
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::placed_coins(
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5]],
            vec![1, 2, 3, 4, 5, 6]
        ),
        vec![120, 1, 1, 1, 1, 1]
    );
    assert_eq!(
        Solution::placed_coins(
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 3],
                vec![1, 4],
                vec![1, 5],
                vec![2, 6],
                vec![2, 7],
                vec![2, 8]
            ],
            vec![1, 4, 2, 3, 5, 7, 8, -4, 2]
        ),
        vec![280, 140, 32, 1, 1, 1, 1, 1, 1]
    );
    assert_eq!(Solution::placed_coins(vec![vec![0, 1], vec![0, 2]], vec![1, 2, -2]), vec![0, 1, 1]);
}
