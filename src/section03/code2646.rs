#![allow(dead_code)]

/*
// 2646. Minimize the Total Price of the Trips
// https://leetcode.com/problems/minimize-the-total-price-of-the-trips/
// https://leetcode.cn/problems/minimize-the-total-price-of-the-trips/
//
// Hard
//
// There exists an undirected and unrooted tree with n nodes indexed from 0 to n - 1. You are given the integer n and a 2D integer array edges of length n - 1, where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree.

Each node has an associated price. You are given an integer array price, where price[i] is the price of the ith node.

The price sum of a given path is the sum of the prices of all nodes lying on that path.

Additionally, you are given a 2D integer array trips, where trips[i] = [starti, endi] indicates that you start the ith trip from the node starti and travel to the node endi by any path you like.

Before performing your first trip, you can choose some non-adjacent nodes and halve the prices.

Return the minimum total price sum to perform all the given trips.

Example 1:

Input: n = 4, edges = [[0,1],[1,2],[1,3]], price = [2,2,10,6], trips = [[0,3],[2,1],[2,3]]
Output: 23
Explanation: The diagram above denotes the tree after rooting it at node 2. The first part shows the initial tree and the second part shows the tree after choosing nodes 0, 2, and 3, and making their price half.
For the 1st trip, we choose path [0,1,3]. The price sum of that path is 1 + 2 + 3 = 6.
For the 2nd trip, we choose path [2,1]. The price sum of that path is 2 + 5 = 7.
For the 3rd trip, we choose path [2,1,3]. The price sum of that path is 5 + 2 + 3 = 10.
The total price sum of all trips is 6 + 7 + 10 = 23.
It can be proven, that 23 is the minimum answer that we can achieve.
Example 2:

Input: n = 2, edges = [[0,1]], price = [2,2], trips = [[0,0]]
Output: 1
Explanation: The diagram above denotes the tree after rooting it at node 0. The first part shows the initial tree and the second part shows the tree after choosing node 0, and making its price half.
For the 1st trip, we choose path [0]. The price sum of that path is 1.
The total price sum of all trips is 1. It can be proven, that 1 is the minimum answer that we can achieve.

Constraints:

1 <= n <= 50
edges.length == n - 1
0 <= ai, bi <= n - 1
edges represents a valid tree.
price.length == n
price[i] is an even integer.
1 <= price[i] <= 1000
1 <= trips.length <= 100
0 <= starti, endi <= n - 1
*/

struct Solution;

impl Solution {
    pub fn minimum_total_price(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>, trips: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![-1; 2]; 51]; // dp optimization to not recalculate values
        let mut mp = vec![0; 51]; // count how many times a node is used in trips

        let mut adj = vec![vec![]; n as usize];
        for i in edges {
            adj[i[0] as usize].push(i[1]);
            adj[i[1] as usize].push(i[0]);
        }
        for i in trips.iter() {
            Self::count_paths(&mut mp, &adj, i[0], i[1]);
        }
        Self::dfs(&mut dp, &mp, &adj, 0, &price, true, -1)
    }

    fn count_paths(mp: &mut [i32], adj: &[Vec<i32>], x: i32, y: i32) {
        let mut q = std::collections::VecDeque::new();
        q.push_back((x, -1));
        let mut vis = vec![0; 51];
        let mut last = vec![0; 51]; // so we can efficiently backtrack the path from node a to node b
        while !q.is_empty() {
            let (mut t, p) = q.pop_front().unwrap();
            if vis[t as usize] != 0 {
                vis[t as usize] += 1;
                continue;
            }
            vis[t as usize] += 1;
            last[t as usize] = p;
            if t == y {
                while t != -1 {
                    mp[t as usize] += 1;
                    t = last[t as usize];
                }
                return;
            }
            for i in &adj[t as usize] {
                q.push_back((*i, t));
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn dfs(dp: &mut Vec<Vec<i32>>, mp: &Vec<i32>, adj: &Vec<Vec<i32>>, pos: usize, price: &Vec<i32>, can_take: bool, last: i32) -> i32 {
        if dp[pos][can_take as usize] != -1 {
            return dp[pos][can_take as usize];
        }
        let mut ans = price[pos] * mp[pos];
        let mut res1 = 0;
        let mut res2 = 0;
        for &i in &adj[pos] {
            if i != last {
                res1 += Self::dfs(dp, mp, adj, i as usize, price, true, pos as i32);
            }
        }
        if can_take {
            for &i in &adj[pos] {
                if i != last {
                    res2 += Self::dfs(dp, mp, adj, i as usize, price, false, pos as i32);
                }
            }
            ans = (ans + res1).min(ans / 2 + res2);
        } else {
            ans += res1;
        }
        dp[pos][can_take as usize] = ans;
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            4,
            vec![vec![0, 1], vec![1, 2], vec![1, 3]],
            vec![2, 2, 10, 6],
            vec![vec![0, 3], vec![2, 1], vec![2, 3]],
            23,
        ),
        (2, vec![vec![0, 1]], vec![2, 2], vec![vec![0, 0]], 1),
    ];

    for (n, edges, price, trips, res) in cases {
        assert_eq!(Solution::minimum_total_price(n, edges, price, trips), res);
    }
}
