#![allow(dead_code)]

/*

// 2065. Maximum Path Quality of a Graph
// https://leetcode.com/problems/maximum-path-quality-of-a-graph/
// https://leetcode.cn/problems/maximum-path-quality-of-a-graph/
//
// Hard
//
// There is an undirected graph with n nodes numbered from 0 to n - 1 (inclusive).
// You are given a 0-indexed integer array values where values[i] is the value of the ith node.
// You are also given a 0-indexed 2D integer array edges, where each edges[j] = [uj, vj, timej]
// indicates that there is an undirected edge between the nodes uj and vj, and it takes timej seconds
// to travel between the two nodes. Finally, you are given an integer maxTime.

A valid path in the graph is any path that starts at node 0, ends at node 0, and takes at most maxTime seconds to complete. You may visit the same node multiple times. The quality of a valid path is the sum of the values of the unique nodes visited in the path (each node's value is added at most once to the sum).

Return the maximum quality of a valid path.

Note: There are at most four edges connected to each node.

Example 1:

Input: values = [0,32,10,43], edges = [[0,1,10],[1,2,15],[0,3,10]], maxTime = 49
Output: 75
Explanation:
One possible path is 0 -> 1 -> 0 -> 3 -> 0. The total time taken is 10 + 10 + 10 + 10 = 40 <= 49.
The nodes visited are 0, 1, and 3, giving a maximal path quality of 0 + 32 + 43 = 75.

Example 2:

Input: values = [5,10,15,20], edges = [[0,1,10],[1,2,10],[0,3,10]], maxTime = 30
Output: 25
Explanation:
One possible path is 0 -> 3 -> 0. The total time taken is 10 + 10 = 20 <= 30.
The nodes visited are 0 and 3, giving a maximal path quality of 5 + 20 = 25.

Example 3:

Input: values = [1,2,3,4], edges = [[0,1,10],[1,2,11],[2,3,12],[1,3,13]], maxTime = 50
Output: 7
Explanation:
One possible path is 0 -> 1 -> 3 -> 1 -> 0. The total time taken is 10 + 13 + 13 + 10 = 46 <= 50.
The nodes visited are 0, 1, and 3, giving a maximal path quality of 1 + 2 + 4 = 7.

Constraints:

    n == values.length
    1 <= n <= 1000
    0 <= values[i] <= 10^8
    0 <= edges.length <= 2000
    edges[j].length == 3
    0 <= uj < vj <= n - 1
    10 <= timej, maxTime <= 100
    All the pairs [uj, vj] are unique.
    There are at most four edges connected to each node.
    The graph may not be connected.
*/

struct Solution;

impl Solution {
    pub fn maximal_path_quality(values: Vec<i32>, edges: Vec<Vec<i32>>, max_time: i32) -> i32 {
        fn dfs(
            al: &Vec<Vec<(usize, i32)>>,
            vis: &mut Vec<i32>,
            vals: &Vec<i32>,
            i: usize,
            time: i32,
            val: i32,
            max_val: &mut i32,
        ) -> i32 {
            vis[i] += 1;
            let val = val + if vis[i] == 1 { vals[i] } else { 0 };
            if i == 0 {
                *max_val = std::cmp::max(*max_val, val);
            }
            for (j, t) in &al[i] {
                if time - t >= 0 {
                    dfs(al, vis, vals, *j, time - t, val, max_val);
                }
            }
            vis[i] -= 1;
            *max_val
        }

        let mut max_val = 0;
        let mut al = vec![vec![]; values.len()];
        let mut vis = vec![0; values.len()];
        for e in edges {
            al[e[0] as usize].push((e[1] as usize, e[2]));
            al[e[1] as usize].push((e[0] as usize, e[2]));
        }
        dfs(&al, &mut vis, &values, 0, max_time, 0, &mut max_val)
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![0, 32, 10, 43],
            vec![vec![0, 1, 10], vec![1, 2, 15], vec![0, 3, 10]],
            49,
            75,
        ),
        (
            vec![5, 10, 15, 20],
            vec![vec![0, 1, 10], vec![1, 2, 10], vec![0, 3, 10]],
            30,
            25,
        ),
        (
            vec![1, 2, 3, 4],
            vec![vec![0, 1, 10], vec![1, 2, 11], vec![2, 3, 12], vec![1, 3, 13]],
            50,
            7,
        ),
    ];
    for (values, edges, max_time, expected) in cases {
        assert_eq!(Solution::maximal_path_quality(values, edges, max_time), expected);
    }
}
