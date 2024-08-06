#![allow(dead_code)]

// 3243. Shortest Distance After Road Addition Queries I
// https://leetcode.com/problems/shortest-distance-after-road-addition-queries-i/
// https://leetcode.cn/problems/shortest-distance-after-road-addition-queries-i/
//
// Medium
//
// You are given an integer n and a 2D integer array queries.
//
// There are n cities numbered from 0 to n - 1. Initially, there is a unidirectional
// road from city i to city i + 1 for all 0 <= i < n - 1.
//
// queries[i] = [ui, vi] represents the addition of a new unidirectional road from city ui to city vi.
// After each query, you need to find the length of the shortest path from city 0 to city n - 1.
//
// Return an array answer where for each i in the range [0, queries.length - 1], answer[i] is the length of
// the shortest path from city 0 to city n - 1 after processing the first i + 1 queries.
//
// Example 1:
//
// Input: n = 5, queries = [[2,4],[0,2],[0,4]]
//
// Output: [3,2,1]
//
// Explanation:
//
// After the addition of the road from 2 to 4, the length of the shortest path from 0 to 4 is 3.
//
// After the addition of the road from 0 to 2, the length of the shortest path from 0 to 4 is 2.
//
// After the addition of the road from 0 to 4, the length of the shortest path from 0 to 4 is 1.
//
// Example 2:
//
// Input: n = 4, queries = [[0,3],[0,2]]
//
// Output: [1,1]
//
// Explanation:
//
// After the addition of the road from 0 to 3, the length of the shortest path from 0 to 3 is 1.
//
// After the addition of the road from 0 to 2, the length of the shortest path remains 1.
//
// Constraints:
//
//     3 <= n <= 500
//     1 <= queries.length <= 500
//     queries[i].length == 2
//     0 <= queries[i][0] < queries[i][1] < n
//     1 < queries[i][1] - queries[i][0]
//     There are no repeated roads among the queries.
//

struct Solution;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut roads = vec![vec![]; n as usize];
        let mut dp = (0..n).collect::<Vec<i32>>();
        let mut res = vec![];
        for q in queries.iter() {
            roads[q[1] as usize].push(q[0]);
            for i in q[1] as usize..n as usize {
                dp[i] = dp[i].min(dp[i - 1] + 1);
                for &l in roads[i].iter() {
                    dp[i] = dp[i].min(dp[l as usize] + 1);
                }
            }
            res.push(dp[n as usize - 1]);
        }
        res
    }
}

#[test]
fn test() {
    let n = 5;
    let queries = vec![vec![2, 4], vec![0, 2], vec![0, 4]];
    let res = vec![3, 2, 1];
    assert_eq!(Solution::shortest_distance_after_queries(n, queries), res);

    let n = 4;
    let queries = vec![vec![0, 3], vec![0, 2]];
    let res = vec![1, 1];
    assert_eq!(Solution::shortest_distance_after_queries(n, queries), res);
}
