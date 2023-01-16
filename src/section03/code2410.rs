#![allow(dead_code)]

// 2410. Maximum Matching of Players With Trainers
// https://leetcode.com/problems/maximum-matching-of-players-with-trainers/
// https://leetcode.cn/problems/maximum-matching-of-players-with-trainers/
//
// You are given a 0-indexed integer array players, where players[i] represents the ability of the ith player.
// You are also given a 0-indexed integer array trainers, where trainers[j] represents the training capacity of the jth trainer.
//
// The ith player can match with the jth trainer if the player's ability is less than or equal to the trainer's training capacity.
// Additionally, the ith player can be matched with at most one trainer, and the jth trainer can be matched with at most one player.
//
// Return the maximum number of matchings between players and trainers that satisfy these conditions.
//
// Example 1:
//
// Input: players = [4,7,9], trainers = [8,2,5,8]
// Output: 2
// Explanation:
// One of the ways we can form two matchings is as follows:
// - players[0] can be matched with trainers[0] since 4 <= 8.
// - players[1] can be matched with trainers[3] since 7 <= 8.
// It can be proven that 2 is the maximum number of matchings that can be formed.
//
// Example 2:
//
// Input: players = [1,1,1], trainers = [10]
// Output: 1
// Explanation:
// The trainer can be matched with any of the 3 players.
// Each player can only be matched with one trainer, so the maximum answer is 1.
//
// Constraints:
//
// - 1 <= players.length, trainers.length <= 10^5
// - 1 <= players[i], trainers[j] <= 10^9
//

struct Solution;

impl Solution {
    pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut ps = players;
        let mut ts = trainers;
        ps.sort();
        ts.sort();

        while !ps.is_empty() && !ts.is_empty() {
            let p = ps.pop().unwrap_or(1);
            let t = ts.pop().unwrap_or(0);
            if p <= t {
                ans += 1;
            } else {
                ts.push(t);
            }
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::match_players_and_trainers(vec![4, 7, 9], vec![8, 2, 5, 8]), 2);
    assert_eq!(Solution::match_players_and_trainers(vec![1, 1, 1], vec![10]), 1);
}
