#![allow(dead_code)]

/*

// 1928. Minimum Cost to Reach Destination in Time
// https://leetcode.com/problems/minimum-cost-to-reach-destination-in-time/
// https://leetcode.cn/problems/minimum-cost-to-reach-destination-in-time/
//
// Hard
//
// There is a country of n cities numbered from 0 to n - 1 where all the cities are connected by bi-directional roads. The roads are represented as a 2D integer array edges where edges[i] = [xi, yi, timei] denotes a road between cities xi and yi that takes timei minutes to travel. There may be multiple roads of differing travel times connecting the same two cities, but no road connects a city to itself.

Each time you pass through a city, you must pay a passing fee. This is represented as a 0-indexed integer array passingFees of length n where passingFees[j] is the amount of dollars you must pay when you pass through city j.

In the beginning, you are at city 0 and want to reach city n - 1 in maxTime minutes or less. The cost of your journey is the summation of passing fees for each city that you passed through at some moment of your journey (including the source and destination cities).

Given maxTime, edges, and passingFees, return the minimum cost to complete your journey, or -1 if you cannot complete it within maxTime minutes.

Example 1:

Input: maxTime = 30, edges = [[0,1,10],[1,2,10],[2,5,10],[0,3,1],[3,4,10],[4,5,15]], passingFees = [5,1,2,20,20,3]
Output: 11
Explanation: The path to take is 0 -> 1 -> 2 -> 5, which takes 30 minutes and has $11 worth of passing fees.

Example 2:

Input: maxTime = 29, edges = [[0,1,10],[1,2,10],[2,5,10],[0,3,1],[3,4,10],[4,5,15]], passingFees = [5,1,2,20,20,3]
Output: 48
Explanation: The path to take is 0 -> 3 -> 4 -> 5, which takes 26 minutes and has $48 worth of passing fees.
You cannot take path 0 -> 1 -> 2 -> 5 since it would take too long.

Example 3:

Input: maxTime = 25, edges = [[0,1,10],[1,2,10],[2,5,10],[0,3,1],[3,4,10],[4,5,15]], passingFees = [5,1,2,20,20,3]
Output: -1
Explanation: There is no way to reach city 5 from city 0 within 25 minutes.

Constraints:

    1 <= maxTime <= 1000
    n == passingFees.length
    2 <= n <= 1000
    n - 1 <= edges.length <= 1000
    0 <= xi, yi <= n - 1
    1 <= timei <= 1000
    1 <= passingFees[j] <= 1000
    The graph may contain multiple edges between two nodes.
    The graph does not contain self loops.
*/

struct Solution;

impl Solution {
    pub fn min_cost(max_time: i32, edges: Vec<Vec<i32>>, passing_fees: Vec<i32>) -> i32 {
        fn solve(dp: &mut Vec<Vec<i32>>, time: i32, adj: &Vec<Vec<(i32, i32)>>, v: i32, pass: &Vec<i32>) -> i32 {
            if time < 0 {
                return std::i32::MAX;
            }
            let n = pass.len() as i32;
            if v == n - 1 {
                return pass[v as usize];
            }
            if dp[v as usize][time as usize] != -1 {
                return dp[v as usize][time as usize];
            }
            let mut cost = std::i32::MAX;
            for x in &adj[v as usize] {
                let u = x.0;
                let temp = x.1;
                if temp <= time {
                    let sol = solve(dp, time - temp, adj, u, pass);
                    cost = cost.min(sol);
                }
            }
            if cost == std::i32::MAX {
                dp[v as usize][time as usize] = std::i32::MAX;
                return std::i32::MAX;
            }
            dp[v as usize][time as usize] = cost + pass[v as usize];
            dp[v as usize][time as usize]
        }

        let mut dp = vec![vec![-1; 1001]; 1001];
        let n = passing_fees.len();
        let mut adj = vec![vec![]; n];
        for edge in edges.iter() {
            let x = edge[0] as usize;
            let y = edge[1] as usize;
            let time = edge[2];
            adj[x].push((y as i32, time));
            adj[y].push((x as i32, time));
        }
        let ans = solve(&mut dp, max_time, &adj, 0, &passing_fees);
        if ans == std::i32::MAX {
            return -1;
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            30,
            vec![
                vec![0, 1, 10],
                vec![1, 2, 10],
                vec![2, 5, 10],
                vec![0, 3, 1],
                vec![3, 4, 10],
                vec![4, 5, 15],
            ],
            vec![5, 1, 2, 20, 20, 3],
            11,
        ),
        (
            29,
            vec![
                vec![0, 1, 10],
                vec![1, 2, 10],
                vec![2, 5, 10],
                vec![0, 3, 1],
                vec![3, 4, 10],
                vec![4, 5, 15],
            ],
            vec![5, 1, 2, 20, 20, 3],
            48,
        ),
        (
            25,
            vec![
                vec![0, 1, 10],
                vec![1, 2, 10],
                vec![2, 5, 10],
                vec![0, 3, 1],
                vec![3, 4, 10],
                vec![4, 5, 15],
            ],
            vec![5, 1, 2, 20, 20, 3],
            -1,
        ),
    ];
    for (max_time, edges, passing_fees, expected) in cases {
        assert_eq!(Solution::min_cost(max_time, edges, passing_fees), expected);
    }
}
