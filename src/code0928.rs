#![allow(dead_code)]

// 928. Minimize Malware Spread II
// https://leetcode.com/problems/minimize-malware-spread-ii/
// https://leetcode.cn/problems/minimize-malware-spread-ii/
//
// You are given a network of n nodes represented as an n x n adjacency matrix graph, where the ith node is directly connected to the jth node if graph[i][j] == 1.
//
// Some nodes initial are initially infected by malware. Whenever two nodes are directly connected,
// and at least one of those two nodes is infected by malware, both nodes will be infected by malware.
// This spread of malware will continue until no more nodes can be infected in this manner.
//
// Suppose M(initial) is the final number of nodes infected with malware in the entire network after the spread of malware stops.
//
// We will remove exactly one node from initial, completely removing it and any connections from this node to any other node.
//
// Return the node that, if removed, would minimize M(initial). If multiple nodes could be removed to minimize M(initial), return such a node with the smallest index.
//
// Example 1:
//
// Input: graph = [[1,1,0],[1,1,0],[0,0,1]], initial = [0,1]
// Output: 0
//
// Example 2:
//
// Input: graph = [[1,1,0],[1,1,1],[0,1,1]], initial = [0,1]
// Output: 1
//
// Example 3:
//
// Input: graph = [[1,1,0,0],[1,1,1,0],[0,1,1,1],[0,0,1,1]], initial = [0,1]
// Output: 1
//
// Constraints:
//
// - n == graph.length
// - n == graph[i].length
// - 2 <= n <= 300
// - graph[i][j] is 0 or 1.
// - graph[i][j] == graph[j][i]
// - graph[i][i] == 1
// - 1 <= initial.length < n
// - 0 <= initial[i] <= n - 1
// - All the integers in initial are unique.
//

struct Solution;

impl Solution {
    pub fn min_malware_spread(graph: Vec<Vec<i32>>, initial: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        fn dfs(
            start: i32,
            graph: &Vec<Vec<i32>>,
            visited: &mut HashSet<i32>,
            visited_count: &mut Vec<i32>,
            initial: &HashSet<i32>,
        ) {
            if !visited.contains(&start) {
                visited.insert(start);
                visited_count[start as usize] += 1;

                let row = &graph[start as usize];

                for (i, elem) in row.iter().enumerate() {
                    if *elem != 0 && !initial.contains(&(i as i32)) {
                        dfs(i as i32, graph, visited, visited_count, initial);
                    }
                }
            }
        }

        let initial_set: HashSet<i32> = initial.iter().copied().collect();
        let mut visited_count: Vec<i32> = vec![0; graph.len()];

        let result: Vec<(i32, HashSet<i32>)> = initial
            .iter()
            .map(|start| {
                let mut visited = HashSet::new();
                dfs(*start, &graph, &mut visited, &mut visited_count, &initial_set);
                (-start, visited)
            })
            .collect();

        result
            .iter()
            .map(|(i1, s1)| (i1, s1.iter().filter(|elem| visited_count[**elem as usize] == 1).count()))
            .max_by(|(i1, t1), (i2, t2)| t1.cmp(t2).then(i1.cmp(i2)))
            .map_or_else(|| 0, |res| -*res.0)
    }
}

#[test]
fn test() {
    let graph = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
    let initial = vec![0, 1];
    assert_eq!(Solution::min_malware_spread(graph, initial), 0);

    let graph = vec![vec![1, 1, 0], vec![1, 1, 1], vec![0, 1, 1]];
    let initial = vec![0, 1];
    assert_eq!(Solution::min_malware_spread(graph, initial), 1);

    let graph = vec![vec![1, 1, 0, 0], vec![1, 1, 1, 0], vec![0, 1, 1, 1], vec![0, 0, 1, 1]];
    let initial = vec![0, 1];
    assert_eq!(Solution::min_malware_spread(graph, initial), 1);
}
