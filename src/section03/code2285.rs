#![allow(dead_code)]

/*

// 2285. Maximum Total Importance of Roads
// https://leetcode.com/problems/maximum-total-importance-of-roads/
// https://leetcode.cn/problems/maximum-total-importance-of-roads/
//
// Medium
//
// You are given an integer n denoting the number of cities in a country.
// The cities are numbered from 0 to n - 1.

You are also given a 2D integer array roads where roads[i] = [ai, bi] denotes that there exists a bidirectional road connecting cities ai and bi.

You need to assign each city with an integer value from 1 to n, where each value can only be used once. The importance of a road is then defined as the sum of the values of the two cities it connects.

Return the maximum total importance of all roads possible after assigning the values optimally.

Example 1:

Input: n = 5, roads = [[0,1],[1,2],[2,3],[0,2],[1,3],[2,4]]
Output: 43
Explanation: The figure above shows the country and the assigned values of [2,4,5,3,1].
- The road (0,1) has an importance of 2 + 4 = 6.
- The road (1,2) has an importance of 4 + 5 = 9.
- The road (2,3) has an importance of 5 + 3 = 8.
- The road (0,2) has an importance of 2 + 5 = 7.
- The road (1,3) has an importance of 4 + 3 = 7.
- The road (2,4) has an importance of 5 + 1 = 6.
The total importance of all roads is 6 + 9 + 8 + 7 + 7 + 6 = 43.
It can be shown that we cannot obtain a greater total importance than 43.

Example 2:

Input: n = 5, roads = [[0,3],[2,4],[1,3]]
Output: 20
Explanation: The figure above shows the country and the assigned values of [4,3,2,5,1].
- The road (0,3) has an importance of 4 + 5 = 9.
- The road (2,4) has an importance of 2 + 1 = 3.
- The road (1,3) has an importance of 3 + 5 = 8.
The total importance of all roads is 9 + 3 + 8 = 20.
It can be shown that we cannot obtain a greater total importance than 20.

Constraints:

    2 <= n <= 5 * 10^4
    1 <= roads.length <= 5 * 10^4
    roads[i].length == 2
    0 <= ai, bi <= n - 1
    ai != bi
    There are no duplicate roads.
*/

struct Solution;

impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        use std::collections::HashMap;
        let mut connections = roads
            .iter()
            .fold(vec![0; n as usize], |mut state, road| {
                state[road[0] as usize] += 1;
                state[road[1] as usize] += 1;
                state
            })
            .iter()
            .enumerate()
            .map(|(node, &roads)| (roads, node as i32))
            .collect::<Vec<(i32, i32)>>();
        connections.sort();
        let dict: HashMap<i32, i64> =
            connections
                .into_iter()
                .enumerate()
                .fold(HashMap::new(), |mut state, (num, (_, node))| {
                    state.insert(node, (num + 1) as i64);
                    state
                });
        roads
            .into_iter()
            .fold(0, |state, nodes| state + dict[&nodes[0]] + dict[&nodes[1]])
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::maximum_importance(
            5,
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![0, 2], vec![1, 3], vec![2, 4]]
        ),
        43
    );
    assert_eq!(
        Solution::maximum_importance(5, vec![vec![0, 3], vec![2, 4], vec![1, 3]]),
        20
    );
}
