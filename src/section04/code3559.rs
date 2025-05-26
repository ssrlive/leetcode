#![allow(dead_code)]

// 3559. Number of Ways to Assign Edge Weights II
// https://leetcode.com/problems/number-of-ways-to-assign-edge-weights-ii/
// https://leetcode.cn/problems/number-of-ways-to-assign-edge-weights-ii/
//
// Hard
//
// There is an undirected tree with n nodes labeled from 1 to n, rooted at node 1. The tree is represented by a 2D integer
// array edges of length n - 1, where edges[i] = [ui, vi] indicates that there is an edge between nodes ui and vi.
//
// Initially, all edges have a weight of 0. You must assign each edge a weight of either 1 or 2.
//
// The cost of a path between any two nodes u and v is the total weight of all edges in the path connecting them.
//
// You are given a 2D integer array queries. For each queries[i] = [ui, vi], determine the number of ways to assign
// weights to edges in the path such that the cost of the path between ui and vi is odd.
//
// Return an array answer, where answer[i] is the number of valid assignments for queries[i].
//
// Since the answer may be large, apply modulo 109 + 7 to each answer[i].
//
// Note: For each query, disregard all edges not in the path between node ui and vi.
//
// Example 1:
//
// Input: edges = [[1,2]], queries = [[1,1],[1,2]]
//
// Output: [0,1]
//
// Explanation:
//
// - Query [1,1]: The path from Node 1 to itself consists of no edges, so the cost is 0. Thus, the number of valid assignments is 0.
// - Query [1,2]: The path from Node 1 to Node 2 consists of one edge (1 → 2). Assigning weight 1 makes the cost odd,
//   while 2 makes it even. Thus, the number of valid assignments is 1.
//
// Example 2:
//
// Input: edges = [[1,2],[1,3],[3,4],[3,5]], queries = [[1,4],[3,4],[2,5]]
//
// Output: [2,1,4]
//
// Explanation:
//
// - Query [1,4]: The path from Node 1 to Node 4 consists of two edges (1 → 3 and 3 → 4). Assigning weights (1,2) or (2,1)
//   results in an odd cost. Thus, the number of valid assignments is 2.
// - Query [3,4]: The path from Node 3 to Node 4 consists of one edge (3 → 4). Assigning weight 1 makes the cost odd,
//   while 2 makes it even. Thus, the number of valid assignments is 1.
// - Query [2,5]: The path from Node 2 to Node 5 consists of three edges (2 → 1, 1 → 3, and 3 → 5).
//   Assigning (1,2,2), (2,1,2), (2,2,1), or (1,1,1) makes the cost odd. Thus, the number of valid assignments is 4.
//
// Constraints:
//
//     2 <= n <= 10^5
//     edges.length == n - 1
//     edges[i] == [ui, vi]
//     1 <= queries.length <= 10^5
//     queries[i] == [ui, vi]
//     1 <= ui, vi <= n
//

struct Solution;

impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        fn dfs(u: i32, par: i32, lvl: i32, levels: &mut [i32], graph: &[Vec<i32>], lca: &mut [Vec<i32>]) {
            levels[u as usize] = lvl;
            lca[u as usize][0] = par;

            for &v in &graph[u as usize] {
                if v != par {
                    dfs(v, u, lvl + 1, levels, graph, lca);
                }
            }
        }
        fn init(n: i32, max_n: i32, lca: &mut [Vec<i32>]) {
            for i in 1..=max_n {
                for j in 0..n {
                    if lca[j as usize][(i - 1) as usize] != -1 {
                        let par = lca[j as usize][(i - 1) as usize];
                        lca[j as usize][i as usize] = lca[par as usize][(i - 1) as usize];
                    }
                }
            }
        }
        fn find_lca(mut a: i32, mut b: i32, max_n: i32, levels: &[i32], lca: &[Vec<i32>]) -> i32 {
            if levels[a as usize] > levels[b as usize] {
                std::mem::swap(&mut a, &mut b);
            }

            let mut d = levels[b as usize] - levels[a as usize];
            while d > 0 {
                let jump = d.ilog2();
                b = lca[b as usize][jump as usize];
                d -= 1 << jump;
            }

            if a == b {
                return a;
            }

            for i in (0..=max_n).rev() {
                if lca[a as usize][i as usize] != -1 && lca[a as usize][i as usize] != lca[b as usize][i as usize] {
                    a = lca[a as usize][i as usize];
                    b = lca[b as usize][i as usize];
                }
            }

            lca[a as usize][0]
        }

        fn get_distance(u: i32, v: i32, max_n: i32, levels: &[i32], lca: &[Vec<i32>]) -> i32 {
            let lca_node = find_lca(u, v, max_n, levels, lca);
            levels[u as usize] + levels[v as usize] - 2 * levels[lca_node as usize]
        }
        fn power_mod(mut x: i64, mut y: i64, mod_val: i64) -> i64 {
            let mut result = 1;
            while y > 0 {
                if y % 2 == 1 {
                    result = (result * x) % mod_val;
                }
                x = (x * x) % mod_val;
                y /= 2;
            }
            result
        }

        let n = edges.len() as i32 + 1;
        let m = queries.len() as i32;
        let mut graph = vec![vec![]; n as usize];
        for edge in &edges {
            graph[(edge[0] - 1) as usize].push(edge[1] - 1);
            graph[(edge[1] - 1) as usize].push(edge[0] - 1);
        }
        let max_n = n.ilog2() as i32;
        let mut levels = vec![0; n as usize];
        let mut lca = vec![vec![-1; (max_n + 1) as usize]; n as usize];
        dfs(0, -1, 0, &mut levels, &graph, &mut lca);
        init(n, max_n, &mut lca);
        let mut ans = vec![0; m as usize];
        for i in 0..m {
            let u = queries[i as usize][0] - 1;
            let v = queries[i as usize][1] - 1;
            let dist = get_distance(u, v, max_n, &levels, &lca);

            if dist == 0 {
                ans[i as usize] = 0;
                continue;
            }

            ans[i as usize] = (power_mod(2, dist as i64 - 1, 1_000_000_007) % 1_000_000_007) as i32;
        }
        ans
    }
}

#[test]
fn test() {
    let edges = vec![vec![1, 2]];
    let queries = vec![vec![1, 1], vec![1, 2]];
    assert_eq!(Solution::assign_edge_weights(edges, queries), vec![0, 1]);

    let edges = vec![vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]];
    let queries = vec![vec![1, 4], vec![3, 4], vec![2, 5]];
    assert_eq!(Solution::assign_edge_weights(edges, queries), vec![2, 1, 4]);
}
