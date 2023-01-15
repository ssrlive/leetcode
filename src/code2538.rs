#![allow(dead_code)]

// 2538. Difference Between Maximum and Minimum Price Sum
// https://leetcode.com/problems/difference-between-maximum-and-minimum-price-sum/
// https://leetcode.cn/problems/difference-between-maximum-and-minimum-price-sum/
//
// There exists an undirected and initially unrooted tree with n nodes indexed from 0 to n - 1. You are given the integer n and a 2D
// integer array edges of length n - 1, where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree.
//
// Each node has an associated price. You are given an integer array price, where price[i] is the price of the ith node.
//
// The price sum of a given path is the sum of the prices of all nodes lying on that path.
//
// The tree can be rooted at any node root of your choice. The incurred cost after choosing root is the
// difference between the maximum and minimum price sum amongst all paths starting at root.
//
// Return the maximum possible cost amongst all possible root choices.
//
// Example 1:
//
// Input: n = 6, edges = [[0,1],[1,2],[1,3],[3,4],[3,5]], price = [9,8,7,6,10,5]
// Output: 24
// Explanation: The diagram above denotes the tree after rooting it at node 2. The first part (colored in red) shows the path with the maximum price sum.
// The second part (colored in blue) shows the path with the minimum price sum.
// - The first path contains nodes [2,1,3,4]: the prices are [7,8,6,10], and the sum of the prices is 31.
// - The second path contains the node [2] with the price [7].
// The difference between the maximum and minimum price sum is 24. It can be proved that 24 is the maximum cost.
//
// Example 2:
//
// Input: n = 3, edges = [[0,1],[1,2]], price = [1,1,1]
// Output: 2
// Explanation: The diagram above denotes the tree after rooting it at node 0. The first part (colored in red) shows the path with the maximum price sum.
// The second part (colored in blue) shows the path with the minimum price sum.
// - The first path contains nodes [0,1,2]: the prices are [1,1,1], and the sum of the prices is 3.
// - The second path contains node [0] with a price [1].
// The difference between the maximum and minimum price sum is 2. It can be proved that 2 is the maximum cost.
//
// Constraints:
//
// - 1 <= n <= 10^5
// - edges.length == n - 1
// - 0 <= ai, bi <= n - 1
// - edges represents a valid tree.
// - price.length == n
// - 1 <= price[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn max_output(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>) -> i64 {
        let mut g = vec![vec![]; n as usize];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let mut ans = 0;
        fn dfs(ans: &mut i64, now: usize, pre: usize, g: &[Vec<usize>], price: &[i32]) -> Vec<i64> {
            let mut cur_max = vec![price[now] as i64, 0];
            for nei in &g[now] {
                if *nei != pre {
                    let sub = dfs(ans, *nei, now, g, price);
                    *ans = (*ans).max(cur_max[0] + sub[1]);
                    *ans = (*ans).max(cur_max[1] + sub[0]);
                    cur_max[0] = cur_max[0].max(sub[0] + price[now] as i64);
                    cur_max[1] = cur_max[1].max(sub[1] + price[now] as i64);
                }
            }
            cur_max
        }

        dfs(&mut ans, 0, n as usize, &g, &price);
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            6,
            vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]],
            vec![9, 8, 7, 6, 10, 5],
            24,
        ),
        (3, vec![vec![0, 1], vec![1, 2]], vec![1, 1, 1], 2),
    ];
    for (n, edges, price, expected) in cases {
        assert_eq!(Solution::max_output(n, edges, price), expected);
    }
}
