#![allow(dead_code)]

/*

// 1686. Stone Game VI
// https://leetcode.com/problems/stone-game-vi/
// https://leetcode.cn/problems/stone-game-vi/
//
// Medium
//
// Alice and Bob take turns playing a game, with Alice starting first.

There are n stones in a pile. On each player's turn, they can remove a stone from the pile and receive points based on the stone's value. Alice and Bob may value the stones differently.

You are given two integer arrays of length n, aliceValues and bobValues. Each aliceValues[i] and bobValues[i] represents how Alice and Bob, respectively, value the ith stone.

The winner is the person with the most points after all the stones are chosen. If both players have the same amount of points, the game results in a draw. Both players will play optimally. Both players know the other's values.

Determine the result of the game, and:

    If Alice wins, return 1.
    If Bob wins, return -1.
    If the game results in a draw, return 0.

Example 1:

Input: aliceValues = [1,3], bobValues = [2,1]
Output: 1
Explanation:
If Alice takes stone 1 (0-indexed) first, Alice will receive 3 points.
Bob can only choose stone 0, and will only receive 2 points.
Alice wins.

Example 2:

Input: aliceValues = [1,2], bobValues = [3,1]
Output: 0
Explanation:
If Alice takes stone 0, and Bob takes stone 1, they will both have 1 point.
Draw.

Example 3:

Input: aliceValues = [2,4,3], bobValues = [1,6,7]
Output: -1
Explanation:
Regardless of how Alice plays, Bob will be able to have more points than Alice.
For example, if Alice takes stone 1, Bob can take stone 2, and Alice takes stone 0, Alice will have 6 points to Bob's 7.
Bob wins.

Constraints:

    n == aliceValues.length == bobValues.length
    1 <= n <= 10^5
    1 <= aliceValues[i], bobValues[i] <= 100
*/

struct Solution;

impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let n = alice_values.len();
        let mut v = vec![];
        for i in 0..n {
            v.push((alice_values[i] + bob_values[i], alice_values[i], bob_values[i]));
        }
        v.sort_by(|a, b| b.0.cmp(&a.0));
        let mut alice = 0;
        let mut bob = 0;
        for (i, item) in v.iter().enumerate().take(n) {
            if i % 2 == 0 {
                alice += item.1;
            } else {
                bob += item.2;
            }
        }
        match alice.cmp(&bob) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 3], vec![2, 1], 1),
        (vec![1, 2], vec![3, 1], 0),
        (vec![2, 4, 3], vec![1, 6, 7], -1),
    ];
    for (alice_values, bob_values, expected) in cases {
        assert_eq!(Solution::stone_game_vi(alice_values, bob_values), expected);
    }
}
