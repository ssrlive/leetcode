#![allow(dead_code)]

// 3311. Construct 2D Grid Matching Graph Layout
// https://leetcode.com/problems/construct-2d-grid-matching-graph-layout/
// https://leetcode.cn/problems/construct-2d-grid-matching-graph-layout/
//
// Hard
//
// You are given a 2D integer array edges representing an undirected graph having n nodes,
// where edges[i] = [ui, vi] denotes an edge between nodes ui and vi.
//
// Construct a 2D grid that satisfies these conditions:
//
//     The grid contains all nodes from 0 to n - 1 in its cells, with each node appearing exactly once.
//     Two nodes should be in adjacent grid cells (horizontally or vertically) if and only if there is an edge between them in edges.
//
// It is guaranteed that edges can form a 2D grid that satisfies the conditions.
//
// Return a 2D integer array satisfying the conditions above. If there are multiple solutions, return any of them.
//
// Example 1:
//
// Input: n = 4, edges = [[0,1],[0,2],[1,3],[2,3]]
//
// Output: [[3,1],[2,0]]
//
// Explanation:
//
// Example 2:
//
// Input: n = 5, edges = [[0,1],[1,3],[2,3],[2,4]]
//
// Output: [[4,2,3,1,0]]
//
// Explanation:
//
// Example 3:
//
// Input: n = 9, edges = [[0,1],[0,4],[0,5],[1,7],[2,3],[2,4],[2,5],[3,6],[4,6],[4,7],[6,8],[7,8]]
//
// Output: [[8,6,3],[7,4,2],[1,0,5]]
//
// Explanation:
//
// Constraints:
//
//     2 <= n <= 5 * 10^4
//     1 <= edges.length <= 10^5
//     edges[i] = [u[i], v[i]]
//     0 <= u[i] < v[i] < n
//     All the edges are distinct.
//     The input is generated such that edges can form a 2D grid that satisfies the conditions.
//

struct Solution;

impl Solution {
    pub fn construct_grid_layout(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut graph = vec![vec![]; n as usize];
        for e in edges.iter() {
            graph[e[0] as usize].push(e[1]);
            graph[e[1] as usize].push(e[0]);
        }

        let mut adj = vec![0; n as usize];
        let mut adj_num = [0; 5];
        let mut adj1 = vec![];
        let mut adj2 = vec![];

        for i in 0..n {
            adj[i as usize] = graph[i as usize].len() as i32;
            adj_num[adj[i as usize] as usize] += 1;
            if adj[i as usize] == 1 {
                adj1.push(i);
            }
            if adj[i as usize] == 2 {
                adj2.push(i);
            }
        }

        let mut res = vec![];
        if adj_num[1] == 2 {
            res.resize(1, vec![0; n as usize]);
            let start = adj1[0];
            let mut i = 0;
            let mut v = vec![false; n as usize];
            let mut q = std::collections::VecDeque::new();
            q.push_back(start);
            v[start as usize] = true;
            while !q.is_empty() {
                let cur = q.pop_front().unwrap();
                res[0][i as usize] = cur;
                i += 1;
                for next in graph[cur as usize].iter() {
                    if !v[*next as usize] {
                        v[*next as usize] = true;
                        q.push_back(*next);
                    }
                }
            }
            return res;
        } else {
            let mut height = 0;
            let mut width = 0;
            let start = adj2[0];
            let mut v = vec![false; n as usize];
            let mut q = std::collections::VecDeque::new();
            q.push_back(start);
            v[start as usize] = true;
            let mut list = vec![];
            let mut l = 0;
            while !q.is_empty() {
                let size = q.len();
                l += 1;
                let mut next_level = vec![];
                let cur = q.pop_front().unwrap();
                next_level.push(cur);
                let mut min = -1;
                for next in graph[cur as usize].iter() {
                    if !v[*next as usize] && (min == -1 || adj[min as usize] > adj[*next as usize]) {
                        min = *next;
                    }
                }
                if min != -1 {
                    q.push_back(min);
                    v[min as usize] = true;
                    if min != start && adj2.contains(&min) && height == 0 {
                        height = l + 1;
                    }
                }
                for &next in graph[cur as usize].iter() {
                    if !v[next as usize] {
                        v[next as usize] = true;
                        q.push_back(next);
                    }
                }
                let mut size = size - 1;
                while size > 0 {
                    let cur = q.pop_front().unwrap();
                    next_level.push(cur);
                    if adj2.contains(&cur) && width == 0 {
                        width = l;
                    }
                    for &next in graph[cur as usize].iter() {
                        if !v[next as usize] {
                            v[next as usize] = true;
                            q.push_back(next);
                        }
                    }
                    size -= 1;
                }
                list.push(next_level);
            }
            res.resize(height as usize, vec![0; width as usize]);
            let mut lvl: i32 = -1;

            for level in list.iter() {
                let size = level.len() as i32;
                lvl += 1;
                let row = std::cmp::min(lvl, height - 1);
                let col = lvl - row;
                for j in 0..size {
                    let val = level[j as usize];
                    res[(row - j) as usize][(col + j) as usize] = val;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let n = 4;
    let edges = vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]];
    let result = Solution::construct_grid_layout(n, edges);
    assert_eq!(result, vec![vec![0, 2], vec![1, 3]]);

    let n = 5;
    let edges = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![2, 4]];
    let result = Solution::construct_grid_layout(n, edges);
    assert_eq!(result, vec![vec![0, 1, 3, 2, 4]]);

    let n = 9;
    let edges = vec![
        vec![0, 1],
        vec![0, 4],
        vec![0, 5],
        vec![1, 7],
        vec![2, 3],
        vec![2, 4],
        vec![2, 5],
        vec![3, 6],
        vec![4, 6],
        vec![4, 7],
        vec![6, 8],
        vec![7, 8],
    ];
    let result = Solution::construct_grid_layout(n, edges);
    assert_eq!(result, vec![vec![1, 7, 8], vec![0, 4, 6], vec![5, 2, 3]]);
}
