#![allow(dead_code)]

// 2508. Add Edges to Make Degrees of All Nodes Even
// https://leetcode.com/problems/add-edges-to-make-degrees-of-all-nodes-even/
// https://leetcode.cn/problems/add-edges-to-make-degrees-of-all-nodes-even/
//
// There is an undirected graph consisting of n nodes numbered from 1 to n. You are given the integer n and a 2D array edges
// where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi. The graph can be disconnected.
//
// You can add at most two additional edges (possibly none) to this graph so that there are no repeated edges and no self-loops.
//
// Return true if it is possible to make the degree of each node in the graph even, otherwise return false.
//
// The degree of a node is the number of edges connected to it.
//
// Example 1:
//
// Input: n = 5, edges = [[1,2],[2,3],[3,4],[4,2],[1,4],[2,5]]
// Output: true
// Explanation: The above diagram shows a valid way of adding an edge.
// Every node in the resulting graph is connected to an even number of edges.
//
// Example 2:
//
// Input: n = 4, edges = [[1,2],[3,4]]
// Output: true
// Explanation: The above diagram shows a valid way of adding two edges.
//
// Example 3:
//
// Input: n = 4, edges = [[1,2],[1,3],[1,4]]
// Output: false
// Explanation: It is not possible to obtain a valid graph with adding at most 2 edges.
//
// Constraints:
//
// - 3 <= n <= 10^5
// - 2 <= edges.length <= 10^5
// - edges[i].length == 2
// - 1 <= a[i], b[i] <= n
// - a[i] != b[i]
// - There are no repeated edges.
//

struct Solution;

impl Solution {
    pub fn is_possible(n: i32, edges: Vec<Vec<i32>>) -> bool {
        fn check_legal(adj: &[Vec<usize>], a: usize, b: usize) -> bool {
            for &val in adj[a].iter() {
                if val == b {
                    return false;
                }
            }
            for &val in adj[b].iter() {
                if val == a {
                    return false;
                }
            }
            true
        }

        let mut adj = vec![vec![]; n as usize + 1];
        let mut node_degree = vec![0; n as usize + 1];

        for c in edges.iter() {
            let x = c[0] as usize;
            let y = c[1] as usize;
            adj[x].push(y);
            adj[y].push(x);
            node_degree[x] += 1;
            node_degree[y] += 1;
        }

        let mut odd_list = vec![];
        for (i, &item) in node_degree.iter().enumerate() {
            if item % 2 == 1 {
                odd_list.push(i);
            }
        }

        if odd_list.is_empty() {
            return true;
        } else if odd_list.len() == 2 {
            let a = odd_list[0];
            let b = odd_list[1];
            if check_legal(&adj, a, b) {
                return true;
            }
            for i in 1..=n as usize {
                if i == a || i == b {
                    continue;
                }
                if check_legal(&adj, i, a) && check_legal(&adj, i, b) {
                    return true;
                }
            }
        } else if odd_list.len() == 4 {
            let n1 = odd_list[0];
            let n2 = odd_list[1];
            let n3 = odd_list[2];
            let n4 = odd_list[3];
            if check_legal(&adj, n1, n2) && check_legal(&adj, n3, n4) {
                return true;
            }
            if check_legal(&adj, n2, n3) && check_legal(&adj, n1, n4) {
                return true;
            }
            if check_legal(&adj, n1, n3) && check_legal(&adj, n2, n4) {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    let edges = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 2], vec![1, 4], vec![2, 5]];
    assert_eq!(Solution::is_possible(5, edges), true);
    assert_eq!(Solution::is_possible(4, vec![vec![1, 2], vec![3, 4]]), true);

    let edges = vec![vec![1, 2], vec![1, 3], vec![1, 4]];
    assert_eq!(Solution::is_possible(4, edges), false);
}
