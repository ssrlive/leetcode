#![allow(dead_code)]

// 2421. Number of Good Paths
// https://leetcode.com/problems/number-of-good-paths/
// https://leetcode.cn/problems/number-of-good-paths/
//
// There is a tree (i.e. a connected, undirected graph with no cycles) consisting of n nodes numbered from 0 to n - 1 and exactly n - 1 edges.
//
// You are given a 0-indexed integer array vals of length n where vals[i] denotes the value of the ith node.
// You are also given a 2D integer array edges where edges[i] = [ai, bi] denotes that there exists an undirected edge connecting nodes ai and bi.
//
// A good path is a simple path that satisfies the following conditions:
//
// The starting node and the ending node have the same value.
// All nodes between the starting node and the ending node have values less than or equal to the starting node
// (i.e. the starting node's value should be the maximum value along the path).
//
// Return the number of distinct good paths.
//
// Note that a path and its reverse are counted as the same path. For example, 0 -> 1 is considered to be the same
// as 1 -> 0. A single node is also considered as a valid path.
//
// Example 1:
//
// Input: vals = [1,3,2,1,3], edges = [[0,1],[0,2],[2,3],[2,4]]
// Output: 6
// Explanation: There are 5 good paths consisting of a single node.
// There is 1 additional good path: 1 -> 0 -> 2 -> 4.
// (The reverse path 4 -> 2 -> 0 -> 1 is treated as the same as 1 -> 0 -> 2 -> 4.)
// Note that 0 -> 2 -> 3 is not a good path because vals[2] > vals[0].
//
// Example 2:
//
// Input: vals = [1,1,2,2,3], edges = [[0,1],[1,2],[2,3],[2,4]]
// Output: 7
// Explanation: There are 5 good paths consisting of a single node.
// There are 2 additional good paths: 0 -> 1 and 2 -> 3.
//
// Example 3:
//
// Input: vals = [1], edges = []
// Output: 1
// Explanation: The tree consists of only one node, so there is one good path.
//
// Constraints:
//
// - n == vals.length
// - 1 <= n <= 3 * 10^4
// - 0 <= vals[i] <= 10^5
// - edges.length == n - 1
// - edges[i].length == 2
// - 0 <= ai, bi < n
// - ai != bi
// - edges represents a valid tree.
//

struct Solution;

impl Solution {
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        fn find(i: usize, parent: &mut Vec<usize>) -> usize {
            if parent[i] == i {
                return i;
            }
            parent[i] = find(parent[i], parent);
            parent[i]
        }

        let n = vals.len();
        let mut data = vec![];

        for e in edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            data.push((vals[u].max(vals[v]), u, v));
        }
        data.sort();

        let mut parent: Vec<_> = (0..n).collect();
        let mut counts = vec![];
        let mut ret = n;
        for &item in vals.iter().take(n) {
            counts.push((item, 1));
        }

        for (val, u, v) in data {
            let (i, j) = (find(u, &mut parent), find(v, &mut parent));
            let c1 = if counts[i].0 == val { counts[i].1 } else { 0 };
            let c2 = if counts[j].0 == val { counts[j].1 } else { 0 };
            ret += c1 * c2;
            parent[i] = j;
            if c2 > 0 {
                counts[j].1 += c1;
            } else {
                counts[j] = (val, c1);
            }
        }

        ret as _
    }
}

#[test]
fn test() {
    let vals = vec![1, 3, 2, 1, 3];
    let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]];
    assert_eq!(Solution::number_of_good_paths(vals, edges), 6);

    let vals = vec![1, 1, 2, 2, 3];
    let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 4]];
    assert_eq!(Solution::number_of_good_paths(vals, edges), 7);

    let vals = vec![1];
    let edges = vec![];
    assert_eq!(Solution::number_of_good_paths(vals, edges), 1);
}
