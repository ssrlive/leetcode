#![allow(dead_code)]

// 3249. Count the Number of Good Nodes
// https://leetcode.com/problems/count-the-number-of-good-nodes/
// https://leetcode.cn/problems/count-the-number-of-good-nodes/
//
// Medium
//
// There is an undirected tree with n nodes labeled from 0 to n - 1, and rooted at node 0. You are given a 2D integer array
// edges of length n - 1, where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree.
//
// A node is good if all the subtrees rooted at its children have the same size.
//
// Return the number of good nodes in the given tree.
//
// A subtree of treeName is a tree consisting of a node in treeName and all of its descendants.
//
// Example 1:
//
// Input: edges = [[0,1],[0,2],[1,3],[1,4],[2,5],[2,6]]
//
// Output: 7
//
// Explanation:
//
// All of the nodes of the given tree are good.
//
// Example 2:
//
// Input: edges = [[0,1],[1,2],[2,3],[3,4],[0,5],[1,6],[2,7],[3,8]]
//
// Output: 6
//
// Explanation:
//
// There are 6 good nodes in the given tree. They are colored in the image above.
//
// Example 3:
//
// Input: edges = [[0,1],[1,2],[1,3],[1,4],[0,5],[5,6],[6,7],[7,8],[0,9],[9,10],[9,12],[10,11]]
//
// Output: 12
//
// Explanation:
//
// All nodes except node 9 are good.
//
// Constraints:
//
//     2 <= n <= 10^5
//     edges.length == n - 1
//     edges[i].length == 2
//     0 <= a[i], b[i] < n
//     The input is generated such that edges represents a valid tree.
//

struct Solution;

impl Solution {
    fn dfs(curr: i32, par: i32, size: &mut [i32], graph: &[Vec<i32>]) -> i32 {
        let mut sz = 1;
        for &nbr in &graph[curr as usize] {
            if nbr != par {
                sz += Solution::dfs(nbr, curr, size, graph);
            }
        }
        size[curr as usize] = sz;
        sz
    }

    pub fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32 {
        let n = edges.len() + 1;
        let mut graph = vec![vec![]; n];
        let mut size = vec![0; n];
        for i in 0..n - 1 {
            graph[edges[i][1] as usize].push(edges[i][0]);
            graph[edges[i][0] as usize].push(edges[i][1]);
        }
        Solution::dfs(0, -1, &mut size, &graph);
        let mut res = 0;
        for i in 0..n {
            let mut flag = true;
            let mut prev = -1;
            for &nbr in &graph[i] {
                if size[nbr as usize] < size[i] {
                    if prev == -1 {
                        prev = size[nbr as usize];
                    } else if prev != size[nbr as usize] {
                        flag = false;
                        break;
                    }
                }
            }
            if flag {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::count_good_nodes(vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6]]),
        7
    );
    assert_eq!(
        Solution::count_good_nodes(vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![0, 5],
            vec![1, 6],
            vec![2, 7],
            vec![3, 8]
        ]),
        6
    );
    assert_eq!(
        Solution::count_good_nodes(vec![
            vec![0, 1],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![0, 5],
            vec![5, 6],
            vec![6, 7],
            vec![7, 8],
            vec![0, 9],
            vec![9, 10],
            vec![9, 12],
            vec![10, 11]
        ]),
        12
    );
}
