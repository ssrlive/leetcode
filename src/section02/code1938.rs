#![allow(dead_code)]

/*

// 1938. Maximum Genetic Difference Query
// https://leetcode.com/problems/maximum-genetic-difference-query/
// https://leetcode.cn/problems/maximum-genetic-difference-query/
//
// Hard
//
// There is a rooted tree consisting of n nodes numbered 0 to n - 1. Each node's number denotes its unique genetic value (i.e. the genetic value of node x is x). The genetic difference between two genetic values is defined as the bitwise-XOR of their values. You are given the integer array parents, where parents[i] is the parent for node i. If node x is the root of the tree, then parents[x] == -1.

You are also given the array queries where queries[i] = [nodei, vali]. For each query i, find the maximum genetic difference between vali and pi, where pi is the genetic value of any node that is on the path between nodei and the root (including nodei and the root). More formally, you want to maximize vali XOR pi.

Return an array ans where ans[i] is the answer to the ith query.

Example 1:

Input: parents = [-1,0,1,1], queries = [[0,2],[3,2],[2,5]]
Output: [2,3,7]
Explanation: The queries are processed as follows:
- [0,2]: The node with the maximum genetic difference is 0, with a difference of 2 XOR 0 = 2.
- [3,2]: The node with the maximum genetic difference is 1, with a difference of 2 XOR 1 = 3.
- [2,5]: The node with the maximum genetic difference is 2, with a difference of 5 XOR 2 = 7.

Example 2:

Input: parents = [3,7,-1,2,0,7,0,2], queries = [[4,6],[1,15],[0,5]]
Output: [6,14,7]
Explanation: The queries are processed as follows:
- [4,6]: The node with the maximum genetic difference is 0, with a difference of 6 XOR 0 = 6.
- [1,15]: The node with the maximum genetic difference is 1, with a difference of 15 XOR 1 = 14.
- [0,5]: The node with the maximum genetic difference is 2, with a difference of 5 XOR 2 = 7.

Constraints:

    2 <= parents.length <= 10^5
    0 <= parents[i] <= parents.length - 1 for every node i that is not the root.
    parents[root] == -1
    1 <= queries.length <= 3 * 10^4
    0 <= nodei <= parents.length - 1
    0 <= vali <= 2 * 10^5
*/

struct Solution;

impl Solution {
    pub fn max_genetic_difference(parents: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        fn dfs(adjacency_list: &[Vec<usize>], queries: &HashMap<i32, Vec<(i32, usize)>>, cur: i32, trie: &mut Trie, ans: &mut Vec<i32>) {
            trie.insert(cur);
            if let Some(list) = queries.get(&cur) {
                for query in list {
                    let q_val = query.0;
                    let q_idx = query.1;
                    ans[q_idx] = trie.max_xor(q_val);
                }
            }
            for &u in &adjacency_list[cur as usize] {
                dfs(adjacency_list, queries, u as i32, trie, ans);
            }
            trie.remove(cur);
        }

        let n = parents.len();
        let mut root = 0;
        let mut adjacency_list = vec![vec![]; n];
        for (u, v) in parents.into_iter().enumerate() {
            if v == -1 {
                root = u as i32;
            } else {
                adjacency_list[v as usize].push(u);
            }
        }
        let grouped_queries = {
            let mut res = HashMap::new();
            for (idx, query) in queries.iter().enumerate() {
                let u = query[0];
                let p = query[1];
                res.entry(u).or_insert(vec![]).push((p, idx));
            }
            res
        };
        let mut ans = vec![0; queries.len()];
        let mut trie = Trie::new();
        dfs(&adjacency_list, &grouped_queries, root, &mut trie, &mut ans);
        ans
    }
}

use std::collections::HashMap;

#[derive(Default, Debug)]
struct Trie {
    left: Option<Box<Trie>>,
    right: Option<Box<Trie>>,
    ct: usize,
}

impl Trie {
    pub fn new() -> Trie {
        Default::default()
    }

    pub fn insert(&mut self, num: i32) {
        let mut cur = self;
        for i in (0..18).rev() {
            if (num >> i) & 1 > 0 {
                if let Some(ref mut r) = cur.right {
                    cur = r;
                } else {
                    cur.right = Some(Box::new(Trie::new()));
                    cur = cur.right.as_mut().unwrap();
                }
                cur.ct += 1;
            } else {
                if let Some(ref mut l) = cur.left {
                    cur = l;
                } else {
                    cur.left = Some(Box::new(Trie::new()));
                    cur = cur.left.as_mut().unwrap();
                }
                cur.ct += 1;
            }
        }
    }

    pub fn remove(&mut self, num: i32) {
        let mut cur = self;
        for i in (0..18).rev() {
            if (num >> i) & 1 > 0 {
                cur = cur.right.as_mut().unwrap();
                cur.ct -= 1;
            } else {
                cur = cur.left.as_mut().unwrap();
                cur.ct -= 1;
            }
        }
    }

    pub fn max_xor(&self, num: i32) -> i32 {
        let mut cur = self;
        let mut ans = 0;
        for i in (0..18).rev() {
            ans <<= 1;
            if (num >> i) & 1 > 0 {
                match cur.left {
                    Some(ref l) if l.ct > 0 => {
                        cur = l;
                        ans += 1;
                    }
                    _ => cur = cur.right.as_ref().unwrap(),
                }
            } else {
                match cur.right {
                    Some(ref r) if r.ct > 0 => {
                        cur = r;
                        ans += 1;
                    }
                    _ => cur = cur.left.as_ref().unwrap(),
                }
            }
        }
        ans
    }
}

#[test]
fn test() {
    let parents = vec![-1, 0, 1, 1];
    let queries = vec![vec![0, 2], vec![3, 2], vec![2, 5]];
    let res = vec![2, 3, 7];
    assert_eq!(Solution::max_genetic_difference(parents, queries), res);

    let parents = vec![3, 7, -1, 2, 0, 7, 0, 2];
    let queries = vec![vec![4, 6], vec![1, 15], vec![0, 5]];
    let res = vec![6, 14, 7];
    assert_eq!(Solution::max_genetic_difference(parents, queries), res);
}
