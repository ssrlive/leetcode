#![allow(dead_code)]

/*

// 1626. Best Team With No Conflicts
// https://leetcode.com/problems/best-team-with-no-conflicts/
// https://leetcode.cn/problems/best-team-with-no-conflicts/
//
// Medium
//
// You are the manager of a basketball team. For the upcoming tournament, you want to choose the team with the highest overall score. The score of the team is the sum of scores of all the players in the team.

However, the basketball team is not allowed to have conflicts. A conflict exists if a younger player has a strictly higher score than an older player. A conflict does not occur between players of the same age.

Given two lists, scores and ages, where each scores[i] and ages[i] represents the score and age of the ith player, respectively, return the highest overall score of all possible basketball teams.

Example 1:

Input: scores = [1,3,5,10,15], ages = [1,2,3,4,5]
Output: 34
Explanation: You can choose all the players.

Example 2:

Input: scores = [4,5,6,5], ages = [2,1,2,1]
Output: 16
Explanation: It is best to choose the last 3 players. Notice that you are allowed to choose multiple people of the same age.

Example 3:

Input: scores = [1,2,3,5], ages = [8,9,10,1]
Output: 6
Explanation: It is best to choose the first 3 players.

Constraints:

    1 <= scores.length, ages.length <= 1000
    scores.length == ages.length
    1 <= scores[i] <= 10^6
    1 <= ages[i] <= 1000
*/

struct Solution;

impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let mut players = scores.into_iter().zip(ages).map(|(s, a)| (a, s)).collect::<Vec<_>>();
        players.sort_unstable();
        let mut dp = vec![0; players.len()];
        let mut ans = 0;
        for i in 0..players.len() {
            dp[i] = players[i].1;
            for j in 0..i {
                if players[j].1 <= players[i].1 {
                    dp[i] = dp[i].max(dp[j] + players[i].1);
                }
            }
            ans = ans.max(dp[i]);
        }
        ans
    }
}

#[test]
fn test() {
    let scores = vec![1, 3, 5, 10, 15];
    let ages = vec![1, 2, 3, 4, 5];
    assert_eq!(Solution::best_team_score(scores, ages), 34);
    let scores = vec![4, 5, 6, 5];
    let ages = vec![2, 1, 2, 1];
    assert_eq!(Solution::best_team_score(scores, ages), 16);
    let scores = vec![1, 2, 3, 5];
    let ages = vec![8, 9, 10, 1];
    assert_eq!(Solution::best_team_score(scores, ages), 6);
}
