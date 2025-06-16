#![allow(dead_code)]

// 3585. Find Weighted Median Node in Tree
// https://leetcode.com/problems/find-weighted-median-node-in-tree/
// https://leetcode.cn/problems/find-weighted-median-node-in-tree/
//
// Hard
//
// You are given an integer n and an undirected, weighted tree rooted at node 0 with n nodes numbered from 0 to n - 1.
// This is represented by a 2D array edges of length n - 1, where edges[i] = [ui, vi, wi] indicates an edge from node ui to vi with weight wi.
//
// The weighted median node is defined as the first node x on the path from ui to vi such that the sum
// of edge weights from ui to x is greater than or equal to half of the total path weight.
//
// You are given a 2D integer array queries. For each queries[j] = [uj, vj], determine the weighted median node along the path from uj to vj.
//
// Return an array ans, where ans[j] is the node index of the weighted median for queries[j].
//
// Example 1:
//
// Input: n = 2, edges = [[0,1,7]], queries = [[1,0],[0,1]]
//
// Output: [0,1]
//
// Explanation:
//
// Query	Path	Edge
// Weights	Total
// Path
// Weight	Half	Explanation	Answer
// [1, 0]	1 → 0	[7]	7	3.5	Sum from 1 → 0 = 7 >= 3.5, median is node 0.	0
// [0, 1]	0 → 1	[7]	7	3.5	Sum from 0 → 1 = 7 >= 3.5, median is node 1.	1
//
// Example 2:
//
// Input: n = 3, edges = [[0,1,2],[2,0,4]], queries = [[0,1],[2,0],[1,2]]
//
// Output: [1,0,2]
//
// Explanation:
//
// Query	Path	Edge
// Weights	Total
// Path
// Weight	Half	Explanation	Answer
// [0, 1]	0 → 1	[2]	2	1	Sum from 0 → 1 = 2 >= 1, median is node 1.	1
// [2, 0]	2 → 0	[4]	4	2	Sum from 2 → 0 = 4 >= 2, median is node 0.	0
// [1, 2]	1 → 0 → 2	[2, 4]	6	3	Sum from 1 → 0 = 2 < 3.
// Sum from 1 → 2 = 2 + 4 = 6 >= 3, median is node 2.	2
//
// Example 3:
//
// Input: n = 5, edges = [[0,1,2],[0,2,5],[1,3,1],[2,4,3]], queries = [[3,4],[1,2]]
//
// Output: [2,2]
//
// Explanation:
//
// Query	Path	Edge
// Weights	Total
// Path
// Weight	Half	Explanation	Answer
// [3, 4]	3 → 1 → 0 → 2 → 4	[1, 2, 5, 3]	11	5.5	Sum from 3 → 1 = 1 < 5.5.
// Sum from 3 → 0 = 1 + 2 = 3 < 5.5.
// Sum from 3 → 2 = 1 + 2 + 5 = 8 >= 5.5, median is node 2.	2
// [1, 2]	1 → 0 → 2	[2, 5]	7	3.5
//
// Sum from 1 → 0 = 2 < 3.5.
// Sum from 1 → 2 = 2 + 5 = 7 >= 3.5, median is node 2.
//     2
//
// Constraints:
//
//     2 <= n <= 10^5
//     edges.length == n - 1
//     edges[i] == [ui, vi, wi]
//     0 <= ui, vi < n
//     1 <= wi <= 10^9
//     1 <= queries.length <= 10^5
//     queries[j] == [uj, vj]
//     0 <= uj, vj < n
//     The input is generated such that edges represents a valid tree.
//

struct Solution;

impl Solution {
    pub fn find_median(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        fn dfs(u: i32, p: i32, g: &Vec<Vec<(i32, i32)>>, depth: &mut Vec<i32>, pref: &mut Vec<i64>, par: &mut Vec<Vec<i32>>) {
            for &(v, w) in &g[u as usize] {
                if v != p {
                    depth[v as usize] = depth[u as usize] + 1;
                    pref[v as usize] = pref[u as usize] + w as i64;
                    par[0][v as usize] = u;
                    dfs(v, u, g, depth, pref, par);
                }
            }
        }
        fn lca(mut u: i32, mut v: i32, depth: &[i32], par: &[Vec<i32>], msb: usize) -> i32 {
            if depth[u as usize] < depth[v as usize] {
                std::mem::swap(&mut u, &mut v);
            }
            let k = depth[u as usize] - depth[v as usize];
            for bit in (0..=msb).rev() {
                if (k >> bit) & 1 == 1 {
                    u = par[bit][u as usize];
                }
            }
            if u == v {
                return u;
            }
            for bit in (0..=msb).rev() {
                if par[bit][u as usize] != par[bit][v as usize] {
                    u = par[bit][u as usize];
                    v = par[bit][v as usize];
                }
            }
            par[0][u as usize]
        }
        fn dist(u: i32, v: i32, uv: i32, pref: &[i64]) -> i64 {
            pref[u as usize] + pref[v as usize] - 2 * pref[uv as usize]
        }
        fn find(mut u: i32, mut v: i32, depth: &[i32], pref: &[i64], par: &[Vec<i32>], msb: usize) -> i32 {
            if u == v {
                return u;
            }
            let lca_uv = lca(u, v, depth, par, msb);
            let e_u_lca = depth[u as usize] - depth[lca_uv as usize];
            let e_v_lca = depth[v as usize] - depth[lca_uv as usize];
            let d_u_lca = pref[u as usize] - pref[lca_uv as usize];
            let d = dist(u, v, lca_uv, pref);
            let ans: i32;
            if 2 * d_u_lca >= d {
                let mx = 32 - (e_u_lca as f64).log2() as usize - 1;
                let mut cur = 0;
                for bit in (0..=mx).rev() {
                    if par.len() > bit && 2 * (cur + pref[u as usize] - pref[par[bit][u as usize] as usize]) < d {
                        cur += pref[u as usize] - pref[par[bit][u as usize] as usize];
                        u = par[bit][u as usize];
                    }
                }
                ans = par[0][u as usize];
            } else {
                let mx = 32 - (e_v_lca as f64).log2() as usize - 1;
                let cur = pref[u as usize] - pref[lca_uv as usize];
                for bit in (0..=mx).rev() {
                    if par.len() > bit && 2 * (cur + pref[par[bit][v as usize] as usize] - pref[lca_uv as usize]) >= d {
                        v = par[bit][v as usize];
                    }
                }
                ans = v;
            }
            ans
        }

        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in &edges {
            g[e[0] as usize].push((e[1], e[2]));
            g[e[1] as usize].push((e[0], e[2]));
        }
        let mut pref = vec![0i64; n];
        let mut depth = vec![0; n];
        let msb = 32 - (n as f64).log2() as usize - 1;
        let mut par = vec![vec![0; n]; msb + 1];
        dfs(0, -1, &g, &mut depth, &mut pref, &mut par);
        for j in 1..=msb {
            for i in 0..n {
                par[j][i] = par[j - 1][par[j - 1][i] as usize];
            }
        }
        let m = queries.len();
        let mut ans = vec![0; m];
        for i in 0..m {
            ans[i] = find(queries[i][0], queries[i][1], &depth, &pref, &par, msb);
        }
        ans
    }
}

#[test]
fn test() {
    let n = 2;
    let edges = vec![vec![0, 1, 7]];
    let queries = vec![vec![1, 0], vec![0, 1]];
    let result = Solution::find_median(n, edges, queries);
    assert_eq!(result, vec![0, 1]);

    let n = 3;
    let edges = vec![vec![0, 1, 2], vec![2, 0, 4]];
    let queries = vec![vec![0, 1], vec![2, 0], vec![1, 2]];
    let result = Solution::find_median(n, edges, queries);
    assert_eq!(result, vec![1, 0, 2]);

    let n = 5;
    let edges = vec![vec![0, 1, 2], vec![0, 2, 5], vec![1, 3, 1], vec![2, 4, 3]];
    let queries = vec![vec![3, 4], vec![1, 2]];
    let result = Solution::find_median(n, edges, queries);
    assert_eq!(result, vec![2, 2]);

    let n = 4;
    let edges = vec![vec![0, 1, 14], vec![1, 2, 1], vec![2, 3, 13]];
    let queries = vec![vec![2, 1], vec![1, 1]];
    let result = Solution::find_median(n, edges, queries);
    assert_eq!(result, vec![1, 1]);

    let n = 3;
    let edges = vec![vec![0, 1, 9], vec![0, 2, 7]];
    let queries = vec![vec![1, 2]];
    let result = Solution::find_median(n, edges, queries);
    assert_eq!(result, vec![0]);
}
