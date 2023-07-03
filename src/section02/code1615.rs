#![allow(dead_code)]

/*

// 1615. Maximal Network Rank
// https://leetcode.com/problems/maximal-network-rank/
// https://leetcode.cn/problems/maximal-network-rank/
//
// Medium
//
// There is an infrastructure of n cities with some number of roads connecting these cities. Each roads[i] = [ai, bi] indicates that there is a bidirectional road between cities ai and bi.

The network rank of two different cities is defined as the total number of directly connected roads to either city. If a road is directly connected to both cities, it is only counted once.

The maximal network rank of the infrastructure is the maximum network rank of all pairs of different cities.

Given the integer n and the array roads, return the maximal network rank of the entire infrastructure.

Example 1:

Input: n = 4, roads = [[0,1],[0,3],[1,2],[1,3]]
Output: 4
Explanation: The network rank of cities 0 and 1 is 4 as there are 4 roads that are connected to either 0 or 1. The road between 0 and 1 is only counted once.

Example 2:

Input: n = 5, roads = [[0,1],[0,3],[1,2],[1,3],[2,3],[2,4]]
Output: 5
Explanation: There are 5 roads that are connected to cities 1 or 2.

Example 3:

Input: n = 8, roads = [[0,1],[1,2],[2,3],[2,4],[5,6],[5,7]]
Output: 5
Explanation: The network rank of 2 and 5 is 5. Notice that all the cities do not have to be connected.

Constraints:

    2 <= n <= 100
    0 <= roads.length <= n * (n - 1) / 2
    roads[i].length == 2
    0 <= ai, bi <= n-1
    ai != bi
    Each pair of cities has at most one road connecting them.
*/

struct Solution;

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut adj = vec![vec![]; n as usize];
        for road in roads {
            adj[road[0] as usize].push(road[1] as usize);
            adj[road[1] as usize].push(road[0] as usize);
        }
        let mut max = 0;
        for i in 0..n {
            for j in i + 1..n {
                let mut rank = adj[i as usize].len() + adj[j as usize].len();
                if adj[i as usize].contains(&(j as usize)) {
                    rank -= 1;
                }
                max = max.max(rank);
            }
        }
        max as _
    }
}

#[test]
fn test() {
    let cases = vec![
        (4, vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![1, 3]], 4),
        (5, vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![1, 3], vec![2, 3], vec![2, 4]], 5),
        (8, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 4], vec![5, 6], vec![5, 7]], 5),
    ];
    for (n, roads, expected) in cases {
        assert_eq!(Solution::maximal_network_rank(n, roads), expected);
    }
}
