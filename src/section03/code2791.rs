#![allow(dead_code)]

// 2791. Count Paths That Can Form a Palindrome in a Tree
// https://leetcode.com/problems/count-paths-that-can-form-a-palindrome-in-a-tree/
// https://leetcode.cn/problems/count-paths-that-can-form-a-palindrome-in-a-tree/
//
// Hard
//
// You are given a tree (i.e. a connected, undirected graph that has no cycles) rooted at node 0 consisting
// of n nodes numbered from 0 to n - 1. The tree is represented by a 0-indexed array parent of size n,
// where parent[i] is the parent of node i. Since node 0 is the root, parent[0] == -1.
//
// You are also given a string s of length n, where s[i] is the character assigned to the edge between i and parent[i]. s[0] can be ignored.
//
// Return the number of pairs of nodes (u, v) such that u < v and the characters assigned to edges on the path from u to v can be rearranged to form a palindrome.
//
// A string is a palindrome when it reads the same backwards as forwards.
//
// Example 1:
//
// Input: parent = [-1,0,0,1,1,2], s = "acaabc"
// Output: 8
// Explanation: The valid pairs are:
// - All the pairs (0,1), (0,2), (1,3), (1,4) and (2,5) result in one character which is always a palindrome.
// - The pair (2,3) result in the string "aca" which is a palindrome.
// - The pair (1,5) result in the string "cac" which is a palindrome.
// - The pair (3,5) result in the string "acac" which can be rearranged into the palindrome "acca".
//
// Example 2:
//
// Input: parent = [-1,0,0,0,0], s = "aaaaa"
// Output: 10
// Explanation: Any pair of nodes (u,v) where u < v is valid.
//
// Constraints:
//
//     n == parent.length == s.length
//     1 <= n <= 10^5
//     0 <= parent[i] <= n - 1 for all i >= 1
//     parent[0] == -1
//     parent represents a valid tree.
//     s consists of only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn count_palindrome_paths(parent: Vec<i32>, s: String) -> i64 {
        let n = parent.len();
        let mut graph = vec![vec![]; n];

        for u in 1..n {
            graph[parent[u] as usize].push(u);
        }

        let mut count = std::collections::HashMap::<i32, i64>::new();
        let s = s.chars().map(|c| c as i32 - 'a' as i32).collect::<Vec<i32>>();
        let mut ret = 0;

        Self::dfs(&graph, 0, &mut count, 0, &s);
        Self::collect(&graph, 0, &count, 0, &mut ret, &s);
        (ret - n as i64) / 2
    }

    fn dfs(g: &Vec<Vec<usize>>, u: usize, cnt: &mut std::collections::HashMap<i32, i64>, mask: i32, s: &Vec<i32>) {
        *cnt.entry(mask).or_insert(0) += 1;
        for v in &g[u] {
            Self::dfs(g, *v, cnt, mask ^ (1 << s[*v]), s);
        }
    }

    fn collect(g: &Vec<Vec<usize>>, u: usize, cnt: &std::collections::HashMap<i32, i64>, mask: i32, ret: &mut i64, s: &Vec<i32>) {
        if let Some(a) = cnt.get(&mask) {
            *ret += *a;
        }
        for i in 0..26 {
            if let Some(a) = cnt.get(&(mask ^ (1 << i))) {
                *ret += *a;
            }
        }
        for v in &g[u] {
            Self::collect(g, *v, cnt, mask ^ (1 << s[*v]), ret, s);
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_palindrome_paths(vec![-1, 0, 0, 1, 1, 2], "acaabc".to_string()), 8);
    assert_eq!(Solution::count_palindrome_paths(vec![-1, 0, 0, 0, 0], "aaaaa".to_string()), 10);
}
