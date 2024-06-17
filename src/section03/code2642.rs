#![allow(dead_code)]

/*
// 2642. Design Graph With Shortest Path Calculator
// https://leetcode.com/problems/design-graph-with-shortest-path-calculator/
// https://leetcode.cn/problems/design-graph-with-shortest-path-calculator/
//
// Hard
//
// There is a directed weighted graph that consists of n nodes numbered from 0 to n - 1.
// The edges of the graph are initially represented by the given array edges where edges[i] = [fromi, toi, edgeCosti]
// meaning that there is an edge from fromi to toi with the cost edgeCosti.

Implement the Graph class:

Graph(int n, int[][] edges) initializes the object with n nodes and the given edges.
addEdge(int[] edge) adds an edge to the list of edges where edge = [from, to, edgeCost]. It is guaranteed that there is no edge between the two nodes before adding this one.
int shortestPath(int node1, int node2) returns the minimum cost of a path from node1 to node2. If no path exists, return -1. The cost of a path is the sum of the costs of the edges in the path.

Example 1:

Input
["Graph", "shortestPath", "shortestPath", "addEdge", "shortestPath"]
[[4, [[0, 2, 5], [0, 1, 2], [1, 2, 1], [3, 0, 3]]], [3, 2], [0, 3], [[1, 3, 4]], [0, 3]]
Output
[null, 6, -1, null, 6]

Explanation
Graph g = new Graph(4, [[0, 2, 5], [0, 1, 2], [1, 2, 1], [3, 0, 3]]);
g.shortestPath(3, 2); // return 6. The shortest path from 3 to 2 in the first diagram above is 3 -> 0 -> 1 -> 2 with a total cost of 3 + 2 + 1 = 6.
g.shortestPath(0, 3); // return -1. There is no path from 0 to 3.
g.addEdge([1, 3, 4]); // We add an edge from node 1 to node 3, and we get the second diagram above.
g.shortestPath(0, 3); // return 6. The shortest path from 0 to 3 now is 0 -> 1 -> 3 with a total cost of 2 + 4 = 6.

Constraints:

1 <= n <= 100
0 <= edges.length <= n * (n - 1)
edges[i].length == edge.length == 3
0 <= fromi, toi, from, to, node1, node2 <= n - 1
1 <= edgeCosti, edgeCost <= 10^6
There are no repeated edges and no self-loops in the graph at any point.
At most 100 calls will be made for addEdge.
At most 100 calls will be made for shortestPath.
*/

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Graph {
    al: Vec<Vec<(usize, i32)>>,
}

impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut al = vec![vec![]; n as usize];
        for e in edges {
            al[e[0] as usize].push((e[1] as usize, e[2]));
        }
        Self { al }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        self.al[edge[0] as usize].push((edge[1] as usize, edge[2]));
    }

    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let mut pq = std::collections::BinaryHeap::new();
        let mut cost = vec![i32::MAX; self.al.len()];
        cost[node1 as usize] = 0;
        pq.push(std::cmp::Reverse((0, node1)));
        while let Some(std::cmp::Reverse((cost_i, i))) = pq.pop() {
            if cost_i != cost[i as usize] {
                continue;
            }
            for (j, cost_j) in &self.al[i as usize] {
                if cost_i + *cost_j < cost[*j] {
                    cost[*j] = cost_i + *cost_j;
                    pq.push(std::cmp::Reverse((cost[*j], *j as i32)));
                }
            }
        }
        if cost[node2 as usize] == i32::MAX {
            -1
        } else {
            cost[node2 as usize]
        }
    }
}

#[test]
fn test() {
    let mut g = Graph::new(4, vec![vec![0, 2, 5], vec![0, 1, 2], vec![1, 2, 1], vec![3, 0, 3]]);
    assert_eq!(g.shortest_path(3, 2), 6);
    assert_eq!(g.shortest_path(0, 3), -1);
    g.add_edge(vec![1, 3, 4]);
    assert_eq!(g.shortest_path(0, 3), 6);
}
