#![allow(dead_code)]

/*

// 2246. Longest Path With Different Adjacent Characters
// https://leetcode.com/problems/longest-path-with-different-adjacent-characters/
// https://leetcode.cn/problems/longest-path-with-different-adjacent-characters/
//
// Hard
//
// You are given a tree (i.e. a connected, undirected graph that has no cycles) rooted at node 0 consisting of n nodes numbered from 0 to n - 1. The tree is represented by a 0-indexed array parent of size n, where parent[i] is the parent of node i. Since node 0 is the root, parent[0] == -1.

You are also given a string s of length n, where s[i] is the character assigned to node i.

Return the length of the longest path in the tree such that no pair of adjacent nodes on the path have the same character assigned to them.

Example 1:

Input: parent = [-1,0,0,1,1,2], s = "abacbe"
Output: 3
Explanation: The longest path where each two adjacent nodes have different characters in the tree is the path: 0 -> 1 -> 3. The length of this path is 3, so 3 is returned.
It can be proven that there is no longer path that satisfies the conditions.

Example 2:

Input: parent = [-1,0,0,0], s = "aabc"
Output: 3
Explanation: The longest path where each two adjacent nodes have different characters is the path: 2 -> 0 -> 3. The length of this path is 3, so 3 is returned.

Constraints:

    n == parent.length == s.length
    1 <= n <= 10^5
    0 <= parent[i] <= n - 1 for all i >= 1
    parent[0] == -1
    parent represents a valid tree.
    s consists of only lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        fn dfs(u: usize, graph: &Vec<Vec<usize>>, s: &[u8], res: &mut Vec<i32>) -> i32 {
            let (mut l1, mut l2) = (0, 0);

            for &v in graph[u].iter() {
                let l = dfs(v, graph, s, res);
                if s[u] != s[v] {
                    if l > l1 {
                        l2 = l1;
                        l1 = l
                    } else if l > l2 {
                        l2 = l
                    }
                }
            }

            res[0] = res[0].max(l1 + l2 + 1);
            l1 + 1
        }

        let mut res = vec![0];
        let mut graph = vec![vec![]; s.len()];

        for (v, &u) in parent.iter().enumerate() {
            if u != -1 {
                graph[u as usize].push(v)
            }
        }

        dfs(0, &graph, s.as_bytes(), &mut res);
        res[0]
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (vec![-1, 0, 0, 1, 1, 2], "abacbe", 3),
        (vec![-1, 0, 0, 0], "aabc", 3),
    ];
    for (parent, s, expect) in cases {
        assert_eq!(Solution::longest_path(parent, s.to_string()), expect);
    }
}
