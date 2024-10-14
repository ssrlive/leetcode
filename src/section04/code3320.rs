#![allow(dead_code)]

// 3320. Count The Number of Winning Sequences
// https://leetcode.com/problems/count-the-number-of-winning-sequences/
// https://leetcode.cn/problems/count-the-number-of-winning-sequences/
//
// Hard
//
// Alice and Bob are playing a fantasy battle game consisting of n rounds where they summon one of three magical creatures each round:
// a Fire Dragon, a Water Serpent, or an Earth Golem. In each round, players simultaneously summon their creature and are awarded points as follows:
//
//     If one player summons a Fire Dragon and the other summons an Earth Golem, the player who summoned the Fire Dragon is awarded a point.
//     If one player summons a Water Serpent and the other summons a Fire Dragon, the player who summoned the Water Serpent is awarded a point.
//     If one player summons an Earth Golem and the other summons a Water Serpent, the player who summoned the Earth Golem is awarded a point.
//     If both players summon the same creature, no player is awarded a point.
//
// You are given a string s consisting of n characters 'F', 'W', and 'E', representing the sequence of creatures Alice will summon in each round:
//
//     If s[i] == 'F', Alice summons a Fire Dragon.
//     If s[i] == 'W', Alice summons a Water Serpent.
//     If s[i] == 'E', Alice summons an Earth Golem.
//
// Bob’s sequence of moves is unknown, but it is guaranteed that Bob will never summon the same creature in two consecutive rounds.
// Bob beats Alice if the total number of points awarded to Bob after n rounds is strictly greater than the points awarded to Alice.
//
// Return the number of distinct sequences Bob can use to beat Alice.
//
// Since the answer may be very large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: s = "FFF"
//
// Output: 3
//
// Explanation:
//
// Bob can beat Alice by making one of the following sequences of moves: "WFW", "FWF", or "WEW".
// Note that other winning sequences like "WWE" or "EWW" are invalid since Bob cannot make the same move twice in a row.
//
// Example 2:
//
// Input: s = "FWEFW"
//
// Output: 18
//
// Explanation:
//
// Bob can beat Alice by making one of the following sequences of moves: "FWFWF", "FWFWE", "FWEFE", "FWEWE", "FEFWF", "FEFWE",
// "FEFEW", "FEWFE", "WFEFE", "WFEWE", "WEFWF", "WEFWE", "WEFEF", "WEFEW", "WEWFW", "WEWFE", "EWFWE", or "EWEWE".
//
// Constraints:
//
//     1 <= s.length <= 1000
//     s[i] is either 'F', 'W', or 'E'.
//

struct Solution;

impl Solution {
    pub fn count_winning_sequences(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![vec![vec![-1; 4]; 2 * n + 1]; n + 1];
        Solution::solve(s.as_bytes(), 0, 0, 0, &mut dp, n as i64) as i32
    }

    fn solve(s: &[u8], last: i64, ind: i64, sum: i64, dp: &mut Vec<Vec<Vec<i64>>>, n: i64) -> i64 {
        if ind == n {
            return if sum >= 1 { 1 } else { 0 };
        }
        if sum + n < 0 || sum + n > 2 * n {
            return 0;
        }
        if dp[ind as usize][(sum + n) as usize][last as usize] != -1 {
            return dp[ind as usize][(sum + n) as usize][last as usize];
        }

        let mut cnt = 0;
        if last != 1 {
            let sum1 = if s[ind as usize] == b'E' {
                sum + 1
            } else if s[ind as usize] == b'W' {
                sum - 1
            } else {
                sum
            };
            cnt = (cnt + Solution::solve(s, 1, ind + 1, sum1, dp, n)) % 1000000007;
        }
        if last != 2 {
            let sum2 = if s[ind as usize] == b'F' {
                sum + 1
            } else if s[ind as usize] == b'E' {
                sum - 1
            } else {
                sum
            };
            cnt = (cnt + Solution::solve(s, 2, ind + 1, sum2, dp, n)) % 1000000007;
        }
        if last != 3 {
            let sum3 = if s[ind as usize] == b'W' {
                sum + 1
            } else if s[ind as usize] == b'F' {
                sum - 1
            } else {
                sum
            };
            cnt = (cnt + Solution::solve(s, 3, ind + 1, sum3, dp, n)) % 1000000007;
        }

        dp[ind as usize][(sum + n) as usize][last as usize] = cnt;
        cnt
    }
}

#[test]
fn test() {
    let s = "FFF".to_string();
    let res = 3;
    assert_eq!(Solution::count_winning_sequences(s), res);

    let s = "FWEFW".to_string();
    let res = 18;
    assert_eq!(Solution::count_winning_sequences(s), res);
}
