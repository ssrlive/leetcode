#![allow(dead_code)]

// 2876. Count Visited Nodes in a Directed Graph
// https://leetcode.com/problems/count-visited-nodes-in-a-directed-graph/
// https://leetcode.cn/problems/count-visited-nodes-in-a-directed-graph/
//
// Hard
//
// There is a directed graph consisting of n nodes numbered from 0 to n - 1 and n directed edges.
//
// You are given a 0-indexed array edges where edges[i] indicates that there is an edge from node i to node edges[i].
//
// Consider the following process on the graph:
//
// You start from a node x and keep visiting other nodes through edges until you reach a node that you have already visited before on this same process.
// Return an array answer where answer[i] is the number of different nodes that you will visit if you perform the process starting from node i.
//
// Example 1:
//
// Input: edges = [1,2,0,0]
// Output: [3,3,3,4]
// Explanation: We perform the process starting from each node in the following way:
// - Starting from node 0, we visit the nodes 0 -> 1 -> 2 -> 0. The number of different nodes we visit is 3.
// - Starting from node 1, we visit the nodes 1 -> 2 -> 0 -> 1. The number of different nodes we visit is 3.
// - Starting from node 2, we visit the nodes 2 -> 0 -> 1 -> 2. The number of different nodes we visit is 3.
// - Starting from node 3, we visit the nodes 3 -> 0 -> 1 -> 2 -> 0. The number of different nodes we visit is 4.
//
// Example 2:
//
// Input: edges = [1,2,3,4,0]
// Output: [5,5,5,5,5]
// Explanation: Starting from any node we can visit every node in the graph in the process.
//
// Constraints:
//
// n == edges.length
// 2 <= n <= 10^5
// 0 <= edges[i] <= n - 1
// edges[i] != i
//

struct Solution;

impl Solution {
    pub fn count_visited_nodes(edges: Vec<i32>) -> Vec<i32> {
        let n = edges.len();
        let mut ret = vec![0; n];

        for w in 0..n {
            if ret[w] > 0 {
                continue;
            }
            let mut u = w;
            let mut mp = std::collections::HashMap::<usize, usize>::from([(u, 0)]);

            loop {
                let v = edges[u] as usize;
                if ret[v] > 0 {
                    for (node, i) in &mp {
                        ret[*node] = ret[v] + (mp.len() - *i) as i32;
                    }
                    break;
                }
                if mp.contains_key(&v) {
                    let j = *mp.get(&v).unwrap();
                    for (node, i) in &mp {
                        ret[*node] = (mp.len() - j.min(*i)) as i32;
                    }
                    break;
                }
                let sz = mp.len();
                mp.insert(v, sz);
                u = v;
            }
        }

        ret
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_visited_nodes(vec![1, 2, 0, 0]), vec![3, 3, 3, 4]);
    assert_eq!(Solution::count_visited_nodes(vec![1, 2, 3, 4, 0]), vec![5, 5, 5, 5, 5]);
}
