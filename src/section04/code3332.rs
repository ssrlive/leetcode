#![allow(dead_code)]

// 3332. Maximum Points Tourist Can Earn
// https://leetcode.com/problems/maximum-points-tourist-can-earn/
// https://leetcode.cn/problems/maximum-points-tourist-can-earn/
//
// Medium
//
// You are given two integers, n and k, along with two 2D integer arrays, stayScore and travelScore.
//
// A tourist is visiting a country with n cities, where each city is directly connected to every other city.
// The tourist's journey consists of exactly k 0-indexed days, and they can choose any city as their starting point.
//
// Each day, the tourist has two choices:
//
//    Stay in the current city: If the tourist stays in their current city curr during day i, they will earn stayScore[i][curr] points.
//    Move to another city: If the tourist moves from their current city curr to city dest, they will earn travelScore[curr][dest] points.
//
// Return the maximum possible points the tourist can earn.
//
// Example 1:
//
// Input: n = 2, k = 1, stayScore = [[2,3]], travelScore = [[0,2],[1,0]]
//
// Output: 3
//
// Explanation:
//
// The tourist earns the maximum number of points by starting in city 1 and staying in that city.
//
// Example 2:
//
// Input: n = 3, k = 2, stayScore = [[3,4,2],[2,1,2]], travelScore = [[0,2,1],[2,0,4],[3,2,0]]
//
// Output: 8
//
// Explanation:
//
// The tourist earns the maximum number of points by starting in city 1, staying in that city on day 0, and traveling to city 2 on day 1.
//
// Constraints:
//
//    1 <= n <= 200
//    1 <= k <= 200
//    n == travelScore.length == travelScore[i].length == stayScore[i].length
//    k == stayScore.length
//    1 <= stayScore[i][j] <= 100
//    0 <= travelScore[i][j] <= 100
//    travelScore[i][i] == 0
//

struct Solution;

impl Solution {
    pub fn max_score(n: i32, k: i32, stay_score: Vec<Vec<i32>>, travel_score: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut dp = vec![0; n];
        for stay_score_i in stay_score.iter().take(k) {
            let mut dp2 = dp.clone();
            for curr in 0..n {
                dp2[curr] += stay_score_i[curr];
            }
            for curr in 0..n {
                for (dest, dp2_dest) in dp2.iter_mut().enumerate().take(n) {
                    if curr != dest {
                        *dp2_dest = (*dp2_dest).max(dp[curr] + travel_score[curr][dest]);
                    }
                }
            }
            dp = dp2;
        }
        *dp.iter().max().unwrap()
    }
}

#[test]
fn test() {
    let n = 2;
    let k = 1;
    let stay_score = vec![vec![2, 3]];
    let travel_score = vec![vec![0, 2], vec![1, 0]];
    let output = 3;
    assert_eq!(Solution::max_score(n, k, stay_score, travel_score), output);

    let n = 3;
    let k = 2;
    let stay_score = vec![vec![3, 4, 2], vec![2, 1, 2]];
    let travel_score = vec![vec![0, 2, 1], vec![2, 0, 4], vec![3, 2, 0]];
    let output = 8;
    assert_eq!(Solution::max_score(n, k, stay_score, travel_score), output);
}
