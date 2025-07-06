#![allow(dead_code)]

// 2867. Count Valid Paths in a Tree
// https://leetcode.com/problems/count-valid-paths-in-a-tree/
// https://leetcode.cn/problems/count-valid-paths-in-a-tree/
//
// Hard
//
// There is an undirected tree with n nodes labeled from 1 to n. You are given the integer n and a 2D integer array edges
// of length n - 1, where edges[i] = [ui, vi] indicates that there is an edge between nodes ui and vi in the tree.
//
// Return the number of valid paths in the tree.
//
// A path (a, b) is valid if there exists exactly one prime number among the node labels in the path from a to b.
//
// Note that:
//
// - The path (a, b) is a sequence of distinct nodes starting with node a and ending with node b
//   such that every two adjacent nodes in the sequence share an edge in the tree.
// - Path (a, b) and path (b, a) are considered the same and counted only once.
//
// Example 1:
//                  1
//                /   \
//               3     2
//                    / \
//                   4   5
//
// Input: n = 5, edges = [[1,2],[1,3],[2,4],[2,5]]
// Output: 4
// Explanation: The pairs with exactly one prime number on the path between them are:
// - (1, 2) since the path from 1 to 2 contains prime number 2.
// - (1, 3) since the path from 1 to 3 contains prime number 3.
// - (1, 4) since the path from 1 to 4 contains prime number 2.
// - (2, 4) since the path from 2 to 4 contains prime number 2.
// It can be shown that there are only 4 valid paths.
//
// Example 2:
//                  1
//                /   \
//               2     3
//                \   / \
//                 4 6   5
//
// Input: n = 6, edges = [[1,2],[1,3],[2,4],[3,5],[3,6]]
// Output: 6
// Explanation: The pairs with exactly one prime number on the path between them are:
// - (1, 2) since the path from 1 to 2 contains prime number 2.
// - (1, 3) since the path from 1 to 3 contains prime number 3.
// - (1, 4) since the path from 1 to 4 contains prime number 2.
// - (1, 6) since the path from 1 to 6 contains prime number 3.
// - (2, 4) since the path from 2 to 4 contains prime number 2.
// - (3, 6) since the path from 3 to 6 contains prime number 3.
// It can be shown that there are only 6 valid paths.
//
// Constraints:
//
// 1 <= n <= 10^5
// edges.length == n - 1
// edges[i].length == 2
// 1 <= ui, vi <= n
// The input is generated such that edges represent a valid tree.
//

struct Solution;

struct Helper {
    primes: Vec<usize>,
    graph: Vec<Vec<usize>>,
    count: Vec<(i32, i32)>,
}

impl Helper {
    fn new(edges: Vec<Vec<i32>>) -> Self {
        let mut primes: Vec<usize> = vec![];
        for i in 2..1000_usize {
            if i * i > 100000 {
                break;
            }
            let mut flag = 1;
            for p in &primes {
                if !i.is_multiple_of(*p) {
                    continue;
                }
                flag = 0;
                break;
            }
            if flag == 1 {
                primes.push(i);
            }
        }

        let n = edges.len() + 2;
        let mut graph = vec![vec![]; n];
        for e in edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            graph[u].push(v);
            graph[v].push(u);
        }
        Self {
            primes,
            graph,
            count: vec![(0, 0); n],
        }
    }

    fn prime(&self, i: usize) -> bool {
        if i == 1 {
            return false;
        }
        for p in &self.primes {
            if *p >= i {
                break;
            }
            if i.is_multiple_of(*p) {
                return false;
            }
        }
        true
    }

    fn dfs(&mut self, u: usize, p: usize, zero_cnt: &mut i32, one_cnt: &mut i32) {
        let (mut cnt1, mut cnt2) = (0, 0);

        for v in self.graph[u].clone() {
            if v == p {
                continue;
            }

            let (mut c1, mut c2) = (0, 0);
            self.dfs(v, u, &mut c1, &mut c2);
            cnt1 += c1;
            cnt2 += c2;
        }

        if self.prime(u) {
            self.count[u] = (0, cnt1);
            *zero_cnt = 0;
            *one_cnt = cnt1 + 1;
        } else {
            self.count[u] = (cnt1, cnt2);
            *zero_cnt = cnt1 + 1;
            *one_cnt = cnt2;
        }
    }

    fn re_root(&self, u: usize, p: usize, data: &mut Vec<(i32, i32)>) {
        data[u] = self.count[u];
        if p != 0 {
            let (b1, b2) = (self.prime(u), self.prime(p));
            if !b1 && !b2 {
                data[u] = data[p];
            }
            if !b1 && b2 {
                data[u].1 += data[p].1 - self.count[u].0;
            }
            if b1 && !b2 {
                data[u].1 += data[p].0 + 1
            }
        }

        for v in self.graph[u].clone() {
            if v != p {
                self.re_root(v, u, data);
            }
        }
    }
}

impl Solution {
    pub fn count_paths(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut h = Helper::new(edges);
        let (mut zero_cnt, mut one_cnt) = (0, 0);
        h.dfs(1, 0, &mut zero_cnt, &mut one_cnt);

        let mut data = vec![(0, 0); n as usize + 1];
        h.re_root(1, 0, &mut data);

        let mut ret = 0;
        for d in data {
            ret += d.1 as i64;
        }

        ret / 2
    }
}

#[test]
fn test() {
    let v = vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5]];
    assert_eq!(Solution::count_paths(5, v), 4);

    let v = vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![3, 6]];
    assert_eq!(Solution::count_paths(6, v), 6);
}
