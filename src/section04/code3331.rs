#![allow(dead_code)]

// 3331. Find Subtree Sizes After Changes
// https://leetcode.com/problems/find-subtree-sizes-after-changes/
// https://leetcode.cn/problems/find-subtree-sizes-after-changes/
//
// Medium
//
// You are given a tree rooted at node 0 that consists of n nodes numbered from 0 to n - 1.
// The tree is represented by an array parent of size n, where parent[i] is the parent of node i.
// Since node 0 is the root, parent[0] == -1.
//
// You are also given a string s of length n, where s[i] is the character assigned to node i.
//
// We make the following changes on the tree one time simultaneously for all nodes x from 1 to n - 1:
//
//    Find the closest node y to node x such that y is an ancestor of x, and s[x] == s[y].
//    If node y does not exist, do nothing.
//    Otherwise, remove the edge between x and its current parent and make node y the new parent of x by adding an edge between them.
//
// Return an array answer of size n where answer[i] is the size of the subtree rooted at node i in the final tree.
//
// Example 1:
//
// Input: parent = [-1,0,0,1,1,1], s = "abaabc"
//
// Output: [6,3,1,1,1,1]
//
// Explanation:
//
// The parent of node 3 will change from node 1 to node 0.
//
// Example 2:
//
// Input: parent = [-1,0,4,0,1], s = "abbba"
//
// Output: [5,2,1,1,1]
//
// Explanation:
//
// The following changes will happen at the same time:
//
//    The parent of node 4 will change from node 1 to node 0.
//    The parent of node 2 will change from node 4 to node 1.
//
// Constraints:
//
//    n == parent.length == s.length
//    1 <= n <= 10^5
//    0 <= parent[i] <= n - 1 for all i >= 1.
//    parent[0] == -1
//    parent represents a valid tree.
//    s consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
        let s = s.as_bytes();
        let n = parent.len();
        let mut b = parent.clone();
        for i in 1..n {
            let mut par = parent[i];
            while par != 0 && s[i] != s[par as usize] {
                par = parent[par as usize];
            }
            if s[i] == s[par as usize] {
                b[i] = par;
            }
        }
        let parent = b;
        let mut ans = vec![1; n];
        let mut adj = vec![vec![]; n];
        for i in 1..n {
            adj[parent[i] as usize].push(i);
        }
        Self::dfs(&parent, 0, &mut ans, &adj);
        ans
    }

    fn dfs(_parent: &[i32], i: usize, ans: &mut [i32], adj: &[Vec<usize>]) -> i32 {
        if adj[i].is_empty() {
            return ans[i];
        }
        for j in &adj[i] {
            ans[i] += Self::dfs(_parent, *j, ans, adj);
        }
        ans[i]
    }
}

#[test]
fn test() {
    let parent = vec![-1, 0, 0, 1, 1, 1];
    let s = "abaabc".to_string();
    let output = vec![6, 3, 1, 1, 1, 1];
    assert_eq!(Solution::find_subtree_sizes(parent, s), output);

    let parent = vec![-1, 0, 4, 0, 1];
    let s = "abbba".to_string();
    let output = vec![5, 2, 1, 1, 1];
    assert_eq!(Solution::find_subtree_sizes(parent, s), output);
}
