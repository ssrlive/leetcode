#![allow(dead_code)]

/*
// 2603. Collect Coins in a Tree
// https://leetcode.com/problems/collect-coins-in-a-tree/
// https://leetcode.cn/problems/collect-coins-in-a-tree/
//
// Hard
//
// There exists an undirected and unrooted tree with n nodes indexed from 0 to n - 1.
// You are given an integer n and a 2D integer array edges of length n - 1, where edges[i] = [ai, bi]
// indicates that there is an edge between nodes ai and bi in the tree.
// You are also given an array coins of size n where coins[i] can be either 0 or 1,
// where 1 indicates the presence of a coin in the vertex i.

Initially, you choose to start at any vertex in the tree. Then, you can perform the following operations any number of times:

Collect all the coins that are at a distance of at most 2 from the current vertex, or
Move to any adjacent vertex in the tree.
Find the minimum number of edges you need to go through to collect all the coins and go back to the initial vertex.

Note that if you pass an edge several times, you need to count it into the answer several times.

Example 1:

Input: coins = [1,0,0,0,0,1], edges = [[0,1],[1,2],[2,3],[3,4],[4,5]]
Output: 2
Explanation: Start at vertex 2, collect the coin at vertex 0, move to vertex 3, collect the coin at vertex 5 then move back to vertex 2.

Example 2:

Input: coins = [0,0,0,1,1,0,0,1], edges = [[0,1],[0,2],[1,3],[1,4],[2,5],[5,6],[5,7]]
Output: 2
Explanation: Start at vertex 0, collect the coins at vertices 4 and 3, move to vertex 2,  collect the coin at vertex 7, then move back to vertex 0.

Constraints:

n == coins.length
1 <= n <= 3 * 10^4
0 <= coins[i] <= 1
edges.length == n - 1
edges[i].length == 2
0 <= ai, bi < n
ai != bi
edges represents a valid tree.
*/

struct Solution;

impl Solution {
    pub fn collect_the_coins(coins: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let mut sz = coins.len();
        let mut al = vec![vec![]; sz];
        for e in edges.iter() {
            al[e[0] as usize].push(e[1]);
            al[e[1] as usize].push(e[0]);
        }
        let mut cnt = vec![0; sz];
        let mut steps = vec![30000; sz];
        let mut q = vec![];
        for i in 0..sz {
            cnt[i] = al[i].len() as i32;
            if cnt[i] == 1 {
                q.push(i as i32);
            }
        }
        while !q.is_empty() {
            let i = q.pop().unwrap() as usize;
            steps[i] -= 1;
            if steps[i] > 0 {
                sz -= 1;
                for &j in al[i].iter() {
                    let ju = j as usize;
                    steps[ju] = steps[ju].min(steps[i].min(if coins[i] == 1 { 2 } else { 30000 }));
                    cnt[ju] -= 1;
                    if cnt[ju] == 1 {
                        q.push(j);
                    }
                }
            }
        }
        2 * 0.max(sz as i32 - 1)
    }
}

#[test]
fn test() {
    let coins = vec![1, 0, 0, 0, 0, 1];
    let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
    assert_eq!(Solution::collect_the_coins(coins, edges), 2);

    let coins = vec![0, 0, 0, 1, 1, 0, 0, 1];
    let edges = vec![
        vec![0, 1],
        vec![0, 2],
        vec![1, 3],
        vec![1, 4],
        vec![2, 5],
        vec![5, 6],
        vec![5, 7],
    ];
    assert_eq!(Solution::collect_the_coins(coins, edges), 2);
}
