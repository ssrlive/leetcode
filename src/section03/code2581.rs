#![allow(dead_code)]

/*

// 2581. Count Number of Possible Root Nodes
// https://leetcode.com/problems/count-number-of-possible-root-nodes/
// https://leetcode.cn/problems/count-number-of-possible-root-nodes/
//
// Hard
//
// Alice has an undirected tree with n nodes labeled from 0 to n - 1. The tree is represented as a 2D integer array edges of length n - 1 where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree.

Alice wants Bob to find the root of the tree. She allows Bob to make several guesses about her tree. In one guess, he does the following:

    Chooses two distinct integers u and v such that there exists an edge [u, v] in the tree.
    He tells Alice that u is the parent of v in the tree.

Bob's guesses are represented by a 2D integer array guesses where guesses[j] = [uj, vj] indicates Bob guessed uj to be the parent of vj.

Alice being lazy, does not reply to each of Bob's guesses, but just says that at least k of his guesses are true.

Given the 2D integer arrays edges, guesses and the integer k, return the number of possible nodes that can be the root of Alice's tree. If there is no such tree, return 0.

Example 1:

Input: edges = [[0,1],[1,2],[1,3],[4,2]], guesses = [[1,3],[0,1],[1,0],[2,4]], k = 3
Output: 3
Explanation:
Root = 0, correct guesses = [1,3], [0,1], [2,4]
Root = 1, correct guesses = [1,3], [1,0], [2,4]
Root = 2, correct guesses = [1,3], [1,0], [2,4]
Root = 3, correct guesses = [1,0], [2,4]
Root = 4, correct guesses = [1,3], [1,0]
Considering 0, 1, or 2 as root node leads to 3 correct guesses.

Example 2:

Input: edges = [[0,1],[1,2],[2,3],[3,4]], guesses = [[1,0],[3,4],[2,1],[3,2]], k = 1
Output: 5
Explanation:
Root = 0, correct guesses = [3,4]
Root = 1, correct guesses = [1,0], [3,4]
Root = 2, correct guesses = [1,0], [2,1], [3,4]
Root = 3, correct guesses = [1,0], [2,1], [3,2], [3,4]
Root = 4, correct guesses = [1,0], [2,1], [3,2]
Considering any node as root will give at least 1 correct guess.

Constraints:

    edges.length == n - 1
    2 <= n <= 10^5
    1 <= guesses.length <= 10^5
    0 <= ai, bi, uj, vj <= n - 1
    ai != bi
    uj != vj
    edges represents a valid tree.
    guesses[j] is an edge of the tree.
    guesses is unique.
    0 <= k <= guesses.length
*/

struct Solution;

impl Solution {
    pub fn root_count(edges: Vec<Vec<i32>>, guesses: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::collections::HashSet;

        fn back_tracking(
            u: usize,
            flag: &mut Vec<i32>,
            graph: &Vec<Vec<usize>>,
            cnt: &mut i32,
            s: &HashSet<(usize, usize)>,
        ) -> i32 {
            let mut ret = if *cnt >= 0 { 1 } else { 0 };
            flag[u] = 1;
            for v in &graph[u] {
                if flag[*v] == 1 {
                    continue;
                }
                flag[*v] = 1;
                if s.contains(&(u, *v)) {
                    *cnt -= 1;
                }
                if s.contains(&(*v, u)) {
                    *cnt += 1;
                }
                ret += back_tracking(*v, flag, graph, cnt, s);
                if s.contains(&(u, *v)) {
                    *cnt += 1;
                }
                if s.contains(&(*v, u)) {
                    *cnt -= 1;
                }
            }
            ret
        }

        fn dfs(u: usize, flag: &mut Vec<i32>, graph: &Vec<Vec<usize>>, cnt: &mut i32, s: &HashSet<(usize, usize)>) {
            flag[u] = 1;
            for v in &graph[u] {
                if flag[*v] == 1 {
                    continue;
                }
                flag[*v] = 1;
                if s.contains(&(u, *v)) {
                    *cnt += 1;
                }
                dfs(*v, flag, graph, cnt, s);
            }
        }

        let n = edges.len();
        let mut graph = vec![vec![]; n + 1];
        let mut s = HashSet::new();
        for e in edges {
            graph[e[0] as usize].push(e[1] as usize);
            graph[e[1] as usize].push(e[0] as usize);
        }
        for g in guesses {
            s.insert((g[0] as usize, g[1] as usize));
        }
        let mut cnt = -k;
        let mut flag = vec![0; n + 1];
        dfs(0, &mut flag, &graph, &mut cnt, &s);
        flag = vec![0; n + 1];
        back_tracking(0, &mut flag, &graph, &mut cnt, &s)
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![4, 2]],
            vec![vec![1, 3], vec![0, 1], vec![1, 0], vec![2, 4]],
            3,
            3,
        ),
        (
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]],
            vec![vec![1, 0], vec![3, 4], vec![2, 1], vec![3, 2]],
            1,
            5,
        ),
    ];
    for (edges, guesses, k, expected) in cases {
        assert_eq!(Solution::root_count(edges, guesses, k), expected);
    }
}
