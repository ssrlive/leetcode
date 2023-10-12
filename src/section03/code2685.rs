#![allow(dead_code)]

// 2685. Count the Number of Complete Components
// https://leetcode.com/problems/count-the-number-of-complete-components/
// https://leetcode.cn/problems/count-the-number-of-complete-components/
//
// Medium
//
// You are given an integer n. There is an undirected graph with n vertices, numbered from 0 to n - 1.
// You are given a 2D integer array edges where edges[i] = [ai, bi] denotes that there exists
// an undirected edge connecting vertices ai and bi.
//
// Return the number of complete connected components of the graph.
//
// A connected component is a subgraph of a graph in which there exists a path between any two vertices,
// and no vertex of the subgraph shares an edge with a vertex outside of the subgraph.
//
// A connected component is said to be complete if there exists an edge between every pair of its vertices.
//
// Example 1:
//
// Input: n = 6, edges = [[0,1],[0,2],[1,2],[3,4]]
// Output: 3
// Explanation: From the picture above, one can see that all of the components of this graph are complete.
//
// Example 2:
//
// Input: n = 6, edges = [[0,1],[0,2],[1,2],[3,4],[3,5]]
// Output: 1
// Explanation: The component containing vertices 0, 1, and 2 is complete since there is an edge between
// every pair of two vertices. On the other hand, the component containing vertices 3, 4, and 5 is not complete
// since there is no edge between vertices 4 and 5. Thus, the number of complete components in this graph is 1.
//
// Constraints:
//
//     1 <= n <= 50
//     0 <= edges.length <= n * (n - 1) / 2
//     edges[i].length == 2
//     0 <= ai, bi <= n - 1
//     ai != bi
//     There are no repeated edges.
//

struct Solution;

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];

        for e in edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut ret = 0;
        let mut flag = vec![0; n];

        for u in 0..n {
            if flag[u] == 1 {
                continue;
            }

            let mut s = std::collections::HashSet::new();
            let mut count = 0;
            Self::dfs(u, &graph, &mut flag, &mut s, &mut count);
            if count * (count - 1) == s.len() * 2 {
                ret += 1;
            }
        }

        ret
    }

    fn dfs(u: usize, graph: &Vec<Vec<usize>>, flag: &mut Vec<i32>, s: &mut std::collections::HashSet<usize>, count: &mut usize) {
        flag[u] = 1;
        *count += 1;

        for v in &graph[u] {
            let v = *v;
            s.insert(u.min(v) * flag.len() + u.max(v));
            if flag[v] == 1 {
                continue;
            }

            Self::dfs(v, graph, flag, s, count);
        }
    }
}

#[test]
fn test() {
    let n = 6;
    let edges = vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]];

    let res = 3;
    assert_eq!(Solution::count_complete_components(n, edges), res);

    let n = 6;
    let edges = vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4], vec![3, 5]];

    let res = 1;
    assert_eq!(Solution::count_complete_components(n, edges), res);
}
