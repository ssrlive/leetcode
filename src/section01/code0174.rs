#![allow(dead_code)]

// 174. Dungeon Game
// https://leetcode.com/problems/dungeon-game/
// https://leetcode.cn/problems/dungeon-game/
//
// The demons had captured the princess and imprisoned her in the bottom-right corner of a dungeon.
// The dungeon consists of m x n rooms laid out in a 2D grid.
// Our valiant knight was initially positioned in the top-left room and must fight his way through dungeon to rescue the princess.
//
// The knight has an initial health point represented by a positive integer.
// If at any point his health point drops to 0 or below, he dies immediately.
//
// Some of the rooms are guarded by demons (represented by negative integers),
// so the knight loses health upon entering these rooms; other rooms are either empty (represented as 0)
// or contain magic orbs that increase the knight's health (represented by positive integers).
//
// To reach the princess as quickly as possible, the knight decides to move only rightward or downward in each step.
//
// Return the knight's minimum initial health so that he can rescue the princess.
//
// Note that any room can contain threats or power-ups,
// even the first room the knight enters and the bottom-right room where the princess is imprisoned.
//

struct Solution;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let m = dungeon.len();
        let n = dungeon[0].len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for item in dp.iter_mut() {
            item[n] = i32::MAX;
        }
        dp[m].iter_mut().for_each(|cell| *cell = i32::MAX);
        dp[m][n - 1] = 1;
        dp[m - 1][n] = 1;
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                dp[i][j] = (dp[i + 1][j].min(dp[i][j + 1]) - dungeon[i][j]).max(1);
            }
        }
        dp[0][0]
    }
}

#[test]
fn test_calculate_minimum_hp() {
    let dungeon = vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]];
    assert_eq!(Solution::calculate_minimum_hp(dungeon), 7);
}
