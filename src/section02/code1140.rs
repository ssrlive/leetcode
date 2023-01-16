#![allow(dead_code)]

// 1140. Stone Game II
// https://leetcode.com/problems/stone-game-ii/
// https://leetcode.cn/problems/stone-game-ii/
//
// Alice and Bob continue their games with piles of stones.  There are a number of piles arranged in a row,
// and each pile has a positive integer number of stones piles[i].  The objective of the game is to end with the most stones.
//
// Alice and Bob take turns, with Alice starting first.  Initially, M = 1.
//
// On each player's turn, that player can take all the stones in the first X remaining piles, where 1 <= X <= 2M.  Then, we set M = max(M, X).
//
// The game continues until all the stones have been taken.
//
// Assuming Alice and Bob play optimally, return the maximum number of stones Alice can get.
//
// Example 1:
//
// Input: piles = [2,7,9,4,4]
// Output: 10
// Explanation:  If Alice takes one pile at the beginning, Bob takes two piles, then Alice takes 2 piles again.
//   Alice can get 2 + 4 + 4 = 10 piles in total. If Alice takes two piles at the beginning, then Bob can take all three piles left.
//   In this case, Alice get 2 + 7 = 9 piles in total. So we return 10 since it's larger.
//
// Example 2:
//
// Input: piles = [1,2,3,4,5,100]
// Output: 104
//
// Constraints:
//
// - 1 <= piles.length <= 100
// - 1 <= piles[i] <= 10^4
//

struct Solution;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        use std::cmp::{max, min};

        let mut cache = vec![vec![-1; 2 * piles.len()]; piles.len()];

        fn calc(index: usize, m: usize, cache: &mut Vec<Vec<i32>>, piles: &Vec<i32>) -> i32 {
            if cache[index][m] != -1 {
                return cache[index][m];
            }

            let tot = match piles.len() - index <= 2 * m {
                true => return (index..piles.len()).fold(0, |acc, i| acc + piles[i]),
                false => (index..piles.len()).fold(0, |acc, i| acc + piles[i]),
            };

            cache[index][m] = tot
                - (1..min(2 * m + 1, piles.len() - index))
                    .fold(tot, |acc, i| min(acc, calc(index + i, max(i, m), cache, piles)));

            cache[index][m]
        }

        calc(0, 1, &mut cache, &piles)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::stone_game_ii(vec![2, 7, 9, 4, 4]), 10);
    assert_eq!(Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 100]), 104);
}
