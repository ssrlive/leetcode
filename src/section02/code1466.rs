#![allow(dead_code)]

/*

// 1466. Reorder Routes to Make All Paths Lead to the City Zero
// https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/
// https://leetcode.cn/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/
//
// Medium
//
There are n cities numbered from 0 to n - 1 and n - 1 roads such that there is only one way to travel between two different cities (this network form a tree). Last year, The ministry of transport decided to orient the roads in one direction because they are too narrow.

Roads are represented by connections where connections[i] = [ai, bi] represents a road from city ai to city bi.

This year, there will be a big event in the capital (city 0), and many people want to travel to this city.

Your task consists of reorienting some roads such that each city can visit the city 0. Return the minimum number of edges changed.

It's guaranteed that each city can reach city 0 after reorder.

Example 1:

Input: n = 6, connections = [[0,1],[1,3],[2,3],[4,0],[4,5]]
Output: 3
Explanation: Change the direction of edges show in red such that each node can reach the node 0 (capital).

Example 2:

Input: n = 5, connections = [[1,0],[1,2],[3,2],[3,4]]
Output: 2
Explanation: Change the direction of edges show in red such that each node can reach the node 0 (capital).

Example 3:

Input: n = 3, connections = [[1,0],[2,0]]
Output: 0

Constraints:

    2 <= n <= 5 * 10^4
    connections.length == n - 1
    connections[i].length == 2
    0 <= ai, bi <= n - 1
    ai != bi
*/

struct Solution;

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];

        for c in connections {
            let (u, v) = (c[0] as usize, c[1] as usize);
            graph[u].push((v, 1));
            graph[v].push((u, 0));
        }

        let mut ret = 0;
        let mut sk = vec![0];
        let mut flag = vec![0; n];

        flag[0] = 1;
        while let Some(u) = sk.pop() {
            for (v, val) in &graph[u] {
                if flag[*v] == 1 {
                    continue;
                }

                ret += val;
                flag[*v] = 1;
                sk.push(*v);
            }
        }

        ret
    }
}

#[test]
fn test() {
    let cases = vec![
        (6, vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]], 3),
        (5, vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]], 2),
        (3, vec![vec![1, 0], vec![2, 0]], 0),
    ];
    for (n, connections, expected) in cases {
        assert_eq!(Solution::min_reorder(n, connections), expected);
    }
}
