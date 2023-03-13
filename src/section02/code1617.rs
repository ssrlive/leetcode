#![allow(dead_code)]

/*

// 1617. Count Subtrees With Max Distance Between Cities
// https://leetcode.com/problems/count-subtrees-with-max-distance-between-cities/
// https://leetcode.cn/problems/count-subtrees-with-max-distance-between-cities/
//
// Hard
//
// There are n cities numbered from 1 to n. You are given an array edges of size n-1, where edges[i] = [ui, vi] represents a bidirectional edge between cities ui and vi. There exists a unique path between each pair of cities. In other words, the cities form a tree.

A subtree is a subset of cities where every city is reachable from every other city in the subset, where the path between each pair passes through only the cities from the subset. Two subtrees are different if there is a city in one subtree that is not present in the other.

For each d from 1 to n-1, find the number of subtrees in which the maximum distance between any two cities in the subtree is equal to d.

Return an array of size n-1 where the dth element (1-indexed) is the number of subtrees in which the maximum distance between any two cities is equal to d.

Notice that the distance between the two cities is the number of edges in the path between them.

Example 1:

Input: n = 4, edges = [[1,2],[2,3],[2,4]]
Output: [3,4,0]
Explanation:
The subtrees with subsets {1,2}, {2,3} and {2,4} have a max distance of 1.
The subtrees with subsets {1,2,3}, {1,2,4}, {2,3,4} and {1,2,3,4} have a max distance of 2.
No subtree has two nodes where the max distance between them is 3.

Example 2:

Input: n = 2, edges = [[1,2]]
Output: [1]

Example 3:

Input: n = 3, edges = [[1,2],[2,3]]
Output: [2,1]

Constraints:

    2 <= n <= 15
    edges.length == n-1
    edges[i].length == 2
    1 <= ui, vi <= n
    All pairs (ui, vi) are distinct.
*/

struct Solution;

impl Solution {
    pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut g = vec![vec![999; n as usize]; n as usize];
        for e in edges.iter() {
            let i = (e[0] - 1) as usize;
            let j = (e[1] - 1) as usize;
            g[i][j] = 1;
            g[j][i] = 1;
        }
        for k in 0..n as usize {
            for i in 0..n as usize {
                for j in 0..n as usize {
                    g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
                }
            }
        }
        let mut ans = vec![0; n as usize - 1];
        for s in 0_i32..(1 << n) {
            let k = s.count_ones() as i32;
            let (mut e, mut d) = (0_i32, 0_i32);
            for (i, item) in g.iter().enumerate() {
                if s & (1 << i) != 0 {
                    for (j, &item_j) in item.iter().enumerate().skip(i + 1) {
                        if s & (1 << j) != 0 {
                            e += i32::from(g[i][j] == 1);
                            d = d.max(item_j);
                        }
                    }
                }
            }
            if e == k - 1 && d > 0 {
                ans[(d - 1) as usize] += 1;
            }
        }

        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (4, vec![vec![1, 2], vec![2, 3], vec![2, 4]], vec![3, 4, 0]),
        (2, vec![vec![1, 2]], vec![1]),
        (3, vec![vec![1, 2], vec![2, 3]], vec![2, 1]),
    ];
    for (n, edges, expect) in cases {
        assert_eq!(Solution::count_subgraphs_for_each_diameter(n, edges), expect);
    }
}
