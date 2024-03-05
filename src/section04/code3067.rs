#![allow(dead_code)]

// 3067. Count Pairs of Connectable Servers in a Weighted Tree Network
// https://leetcode.com/problems/count-pairs-of-connectable-servers-in-a-weighted-tree-network/
// https://leetcode.cn/problems/count-pairs-of-connectable-servers-in-a-weighted-tree-network/
//
// Medium
//
// You are given an unrooted weighted tree with n vertices representing servers numbered from 0 to n - 1,
// an array edges where edges[i] = [ai, bi, weighti] represents a bidirectional edge between vertices
// ai and bi of weight weighti. You are also given an integer signalSpeed.
//
// Two servers a and b are connectable through a server c if:
//
// - a < b, a != c and b != c.
// - The distance from c to a is divisible by signalSpeed.
// - The distance from c to b is divisible by signalSpeed.
// - The path from c to b and the path from c to a do not share any edges.
//
// Return an integer array count of length n where count[i] is the number of server pairs that are connectable through the server i.
//
// Example 1:
//
// Input: edges = [[0,1,1],[1,2,5],[2,3,13],[3,4,9],[4,5,2]], signalSpeed = 1
// Output: [0,4,6,6,4,0]
// Explanation: Since signalSpeed is 1, count[c] is equal to the number of pairs of paths that start at c and do not share any edges.
// In the case of the given path graph, count[c] is equal to the number of servers to the left of c multiplied by the servers to the right of c.
//
// Example 2:
//
// Input: edges = [[0,6,3],[6,5,3],[0,3,1],[3,2,7],[3,1,6],[3,4,2]], signalSpeed = 3
// Output: [2,0,0,0,0,0,2]
// Explanation: Through server 0, there are 2 pairs of connectable servers: (4, 5) and (4, 6).
// Through server 6, there are 2 pairs of connectable servers: (4, 5) and (0, 5).
// It can be shown that no two servers are connectable through servers other than 0 and 6.
//
// Constraints:
//
// 2 <= n <= 1000
// edges.length == n - 1
// edges[i].length == 3
// 0 <= ai, bi < n
// edges[i] = [ai, bi, weighti]
// 1 <= weighti <= 10^6
// 1 <= signalSpeed <= 10^6
// The input is generated such that edges represents a valid tree.
//

struct Solution;

impl Solution {
    pub fn count_pairs_of_connectable_servers(edges: Vec<Vec<i32>>, signal_speed: i32) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut graph = vec![vec![]; n];
        for edge in edges {
            graph[edge[0] as usize].push((edge[1] as usize, edge[2]));
            graph[edge[1] as usize].push((edge[0] as usize, edge[2]));
        }

        let mut res = vec![0; n];
        for root in 0..n {
            let mut cnt_sum = 0;
            for &(next, w) in &graph[root] {
                let cnt = Self::dfs(root, next, w, signal_speed, &graph);
                res[root] += cnt * cnt_sum;
                cnt_sum += cnt;
            }
        }
        res
    }

    fn dfs(root: usize, next: usize, dist: i32, signal_speed: i32, graph: &Vec<Vec<(usize, i32)>>) -> i32 {
        let mut cnt = 0;
        if dist % signal_speed == 0 {
            cnt += 1;
        }

        for &(next_next, w) in &graph[next] {
            if next_next != root {
                cnt += Self::dfs(next, next_next, dist + w, signal_speed, graph);
            }
        }
        cnt
    }
}

#[test]
fn test() {
    let edges = vec![vec![0, 1, 1], vec![1, 2, 5], vec![2, 3, 13], vec![3, 4, 9], vec![4, 5, 2]];
    let signal_speed = 1;
    let res = vec![0, 4, 6, 6, 4, 0];
    assert_eq!(Solution::count_pairs_of_connectable_servers(edges, signal_speed), res);

    let edges = vec![
        vec![0, 6, 3],
        vec![6, 5, 3],
        vec![0, 3, 1],
        vec![3, 2, 7],
        vec![3, 1, 6],
        vec![3, 4, 2],
    ];
    let signal_speed = 3;
    let res = vec![2, 0, 0, 0, 0, 0, 2];
    assert_eq!(Solution::count_pairs_of_connectable_servers(edges, signal_speed), res);
}
