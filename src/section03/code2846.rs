#![allow(dead_code)]

// 2846. Minimum Edge Weight Equilibrium Queries in a Tree
// https://leetcode.com/problems/minimum-edge-weight-equilibrium-queries-in-a-tree/
// https://leetcode.cn/problems/minimum-edge-weight-equilibrium-queries-in-a-tree/
//
// Hard
//
// There is an undirected tree with n nodes labeled from 0 to n - 1. You are given the integer n
// and a 2D integer array edges of length n - 1, where edges[i] = [ui, vi, wi] indicates that there is an edge
// between nodes ui and vi with weight wi in the tree.
//
// You are also given a 2D integer array queries of length m, where queries[i] = [ai, bi].
// For each query, find the minimum number of operations required to make the weight of every edge on the path
// from ai to bi equal. In one operation, you can choose any edge of the tree and change its weight to any value.
//
// Note that:
//
//     - Queries are independent of each other, meaning that the tree returns to its initial state on each new query.
//     - The path from ai to bi is a sequence of distinct nodes starting with node ai and ending with node bi
//       such that every two adjacent nodes in the sequence share an edge in the tree.
//
// Return an array answer of length m where answer[i] is the answer to the ith query.
//
// Example 1:
//
// Input: n = 7, edges = [[0,1,1],[1,2,1],[2,3,1],[3,4,2],[4,5,2],[5,6,2]], queries = [[0,3],[3,6],[2,6],[0,6]]
// Output: [0,0,1,3]
// Explanation: In the first query, all the edges in the path from 0 to 3 have a weight of 1. Hence, the answer is 0.
// In the second query, all the edges in the path from 3 to 6 have a weight of 2. Hence, the answer is 0.
// In the third query, we change the weight of edge [2,3] to 2. After this operation, all the edges in the path from 2 to 6 have a weight of 2. Hence, the answer is 1.
// In the fourth query, we change the weights of edges [0,1], [1,2] and [2,3] to 2. After these operations, all the edges in the path from 0 to 6 have a weight of 2. Hence, the answer is 3.
// For each queries[i], it can be shown that answer[i] is the minimum number of operations needed to equalize all the edge weights in the path from ai to bi.
//
// Example 2:
//
// Input: n = 8, edges = [[1,2,6],[1,3,4],[2,4,6],[2,5,3],[3,6,6],[3,0,8],[7,0,2]], queries = [[4,6],[0,4],[6,5],[7,4]]
// Output: [1,2,2,3]
// Explanation: In the first query, we change the weight of edge [1,3] to 6. After this operation, all the edges in the path from 4 to 6 have a weight of 6. Hence, the answer is 1.
// In the second query, we change the weight of edges [0,3] and [3,1] to 6. After these operations, all the edges in the path from 0 to 4 have a weight of 6. Hence, the answer is 2.
// In the third query, we change the weight of edges [1,3] and [5,2] to 6. After these operations, all the edges in the path from 6 to 5 have a weight of 6. Hence, the answer is 2.
// In the fourth query, we change the weights of edges [0,7], [0,3] and [1,3] to 6. After these operations, all the edges in the path from 7 to 4 have a weight of 6. Hence, the answer is 3.
// For each queries[i], it can be shown that answer[i] is the minimum number of operations needed to equalize all the edge weights in the path from ai to bi.
//
// Constraints:
//
//     1 <= n <= 10^4
//     edges.length == n - 1
//     edges[i].length == 3
//     0 <= ui, vi < n
//     1 <= wi <= 26
//     The input is generated such that edges represents a valid tree.
//     1 <= queries.length == m <= 2 * 10^4
//     queries[i].length == 2
//     0 <= ai, bi < n
//

struct Solution;

impl Solution {
    fn dfs(u: usize, p: usize, memo: &mut Vec<Vec<i32>>, lev: &mut Vec<i32>, log: usize, adj: &Vec<Vec<(usize, i32)>>) {
        memo[u][0] = p as i32;
        for i in 1..=log {
            memo[u][i] = memo[memo[u][i - 1] as usize][i - 1];
        }
        for x in adj[u].iter() {
            let v = x.0;
            if v != p {
                lev[v] = lev[u] + 1;
                Solution::dfs(v, u, memo, lev, log, adj);
            }
        }
    }

    fn lca(u: usize, v: usize, log: usize, lev: &[i32], memo: &[Vec<i32>]) -> usize {
        let mut u = u;
        let mut v = v;
        if lev[u] < lev[v] {
            std::mem::swap(&mut u, &mut v);
        }
        println!("log: {log}");
        for i in (0..=log).rev() {
            if (lev[u] as i64 - (1_i64 << i)) >= lev[v] as i64 {
                u = memo[u][i] as usize;
            }
        }
        if u == v {
            return u;
        }
        for i in (0..=log).rev() {
            if memo[u][i] != memo[v][i] {
                u = memo[u][i] as usize;
                v = memo[v][i] as usize;
            }
        }
        memo[u][0] as usize
    }

    fn get_count(n: usize, par: usize, adj: &Vec<Vec<(usize, i32)>>, count: &mut Vec<Vec<i32>>) {
        if n != par {
            let (count_n, count_par) = if n < par {
                let (left, right) = count.split_at_mut(par);
                (&mut left[n], &right[0])
            } else {
                let (left, right) = count.split_at_mut(n);
                (&mut right[0], &left[par])
            };
            count_n.iter_mut().zip(count_par.iter()).for_each(|(cn, &cp)| *cn += cp);
        }
        for x in adj[n].iter() {
            if x.0 != par {
                count[x.0][x.1 as usize] += 1;
                Solution::get_count(x.0, n, adj, count);
            }
        }
    }

    pub fn min_operations_queries(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut count = vec![vec![0; 27]; n];
        let mut adj = vec![vec![]; n];
        for x in edges.iter() {
            adj[x[0] as usize].push((x[1] as usize, x[2]));
            adj[x[1] as usize].push((x[0] as usize, x[2]));
        }
        Solution::get_count(0, 0, &adj, &mut count);

        let log = 32;
        let mut memo = vec![vec![-1; log + 1]; n + 1];
        let mut lev = vec![0; n + 1];
        Solution::dfs(0, 0, &mut memo, &mut lev, log, &adj);
        let mut ans = vec![];
        for x in queries.iter() {
            let l = Solution::lca(x[0] as usize, x[1] as usize, log, &lev, &memo);
            let (tot, mx) = (0..27)
                .map(|i| count[x[0] as usize][i] + count[x[1] as usize][i] - 2 * count[l][i])
                .fold((0, 0), |(tot, mx), cur| (tot + cur, mx.max(cur)));
            ans.push(tot - mx);
        }
        ans
    }
}

#[test]
fn test() {
    let n = 7;
    let edges = vec![
        vec![0, 1, 1],
        vec![1, 2, 1],
        vec![2, 3, 1],
        vec![3, 4, 2],
        vec![4, 5, 2],
        vec![5, 6, 2],
    ];
    let queries = vec![vec![0, 3], vec![3, 6], vec![2, 6], vec![0, 6]];
    let ans = vec![0, 0, 1, 3];
    assert_eq!(Solution::min_operations_queries(n, edges, queries), ans);

    let n = 8;
    let edges = vec![
        vec![1, 2, 6],
        vec![1, 3, 4],
        vec![2, 4, 6],
        vec![2, 5, 3],
        vec![3, 6, 6],
        vec![3, 0, 8],
        vec![7, 0, 2],
    ];
    let queries = vec![vec![4, 6], vec![0, 4], vec![6, 5], vec![7, 4]];
    let ans = vec![1, 2, 2, 3];
    assert_eq!(Solution::min_operations_queries(n, edges, queries), ans);
}
